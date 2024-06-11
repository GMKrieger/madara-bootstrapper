use std::str::FromStr;
use std::time::Duration;

use starknet_accounts::{Account, ConnectedAccount};
use starknet_ff::FieldElement;
use starknet_providers::jsonrpc::HttpTransport;
use starknet_providers::JsonRpcClient;
use tokio::time::sleep;

use crate::contract_clients::config::Config;
use crate::contract_clients::eth_bridge::{BridgeDeployable, StarknetLegacyEthBridge};
use crate::contract_clients::starknet_sovereign::StarknetSovereignContract;
use crate::contract_clients::utils::{
    build_single_owner_account, declare_contract_util_func, deploy_proxy_contract, init_governance_proxy,
    DeclarationInput, RpcAccount,
};
use crate::helpers::account_actions::{get_contract_address_from_deploy_tx, AccountActions};
use crate::utils::constants::{ERC20_LEGACY_PATH, LEGACY_BRIDGE_PATH, PROXY_LEGACY_PATH, STARKGATE_PROXY_PATH};
use crate::utils::{convert_to_hex, invoke_contract, save_to_json, wait_for_transaction, JsonValueType};
use crate::CliArgs;

pub struct EthBridge<'a> {
    account: RpcAccount<'a>,
    account_address: FieldElement,
    arg_config: &'a CliArgs,
    clients: &'a Config,
    core_contract: &'a StarknetSovereignContract,
}

pub struct EthBridgeSetupOutput {
    pub legacy_proxy_class_hash: FieldElement,
    pub starkgate_proxy_class_hash: FieldElement,
    pub erc20_legacy_class_hash: FieldElement,
    pub legacy_eth_bridge_class_hash: FieldElement,
    pub eth_proxy_address: FieldElement,
    pub eth_bridge_proxy_address: FieldElement,
    pub eth_bridge: StarknetLegacyEthBridge,
}

impl<'a> EthBridge<'a> {
    pub fn new(
        account: RpcAccount<'a>,
        account_address: FieldElement,
        arg_config: &'a CliArgs,
        clients: &'a Config,
        core_contract: &'a StarknetSovereignContract,
    ) -> Self {
        Self { account, account_address, arg_config, clients, core_contract }
    }

    pub async fn setup(&self) -> EthBridgeSetupOutput {
        let legacy_proxy_class_hash = declare_contract_util_func(DeclarationInput::LegacyDeclarationInputs(
            String::from(PROXY_LEGACY_PATH),
            self.arg_config.rollup_seq_url.clone(),
        ))
        .await;
        log::debug!("Legacy proxy class hash declared.");
        save_to_json("legacy_proxy_class_hash", &JsonValueType::StringType(legacy_proxy_class_hash.to_string()))
            .unwrap();
        sleep(Duration::from_secs(10)).await;

        let starkgate_proxy_class_hash = declare_contract_util_func(DeclarationInput::LegacyDeclarationInputs(
            String::from(STARKGATE_PROXY_PATH),
            self.arg_config.rollup_seq_url.clone(),
        ))
        .await;
        log::debug!("Starkgate proxy class hash declared.");
        save_to_json("starkgate_proxy_class_hash", &JsonValueType::StringType(starkgate_proxy_class_hash.to_string()))
            .unwrap();
        sleep(Duration::from_secs(10)).await;

        let erc20_legacy_class_hash = declare_contract_util_func(DeclarationInput::LegacyDeclarationInputs(
            String::from(ERC20_LEGACY_PATH),
            self.arg_config.rollup_seq_url.clone(),
        ))
        .await;
        log::debug!("ERC20 legacy class hash declared.");
        save_to_json("erc20_legacy_class_hash", &JsonValueType::StringType(erc20_legacy_class_hash.to_string()))
            .unwrap();
        sleep(Duration::from_secs(10)).await;

        let legacy_eth_bridge_class_hash = declare_contract_util_func(DeclarationInput::LegacyDeclarationInputs(
            String::from(LEGACY_BRIDGE_PATH),
            self.arg_config.rollup_seq_url.clone(),
        ))
        .await;
        log::debug!("Legacy ETH Bridge class hash declared !!!");
        save_to_json(
            "legacy_eth_bridge_class_hash",
            &JsonValueType::StringType(legacy_eth_bridge_class_hash.to_string()),
        )
        .unwrap();
        sleep(Duration::from_secs(10)).await;

        let eth_proxy_address = deploy_proxy_contract(
            &self.account,
            self.account_address,
            legacy_proxy_class_hash,
            // salt taken from : https://sepolia.starkscan.co/tx/0x06a5a493cf33919e58aa4c75777bffdef97c0e39cac968896d7bee8cc67905a1
            FieldElement::from_str("0x322c2610264639f6b2cee681ac53fa65c37e187ea24292d1b21d859c55e1a78").unwrap(),
            FieldElement::ONE,
        )
        .await;
        log::info!("✴️ ETH ERC20 proxy deployed [ETH : {:?}]", eth_proxy_address);
        save_to_json("l2_eth_address_proxy", &JsonValueType::StringType(eth_proxy_address.to_string())).unwrap();
        sleep(Duration::from_secs(10)).await;

        let eth_bridge_proxy_address = deploy_proxy_contract(
            &self.account,
            self.account_address,
            legacy_proxy_class_hash,
            FieldElement::from_str("0xabcdabcdabcd").unwrap(),
            FieldElement::ZERO,
        )
        .await;
        log::info!("✴️ ETH Bridge proxy deployed [ETH Bridge : {:?}]", eth_bridge_proxy_address);
        save_to_json("ETH_l2_bridge_address_proxy", &JsonValueType::StringType(eth_bridge_proxy_address.to_string()))
            .unwrap();
        sleep(Duration::from_secs(10)).await;

        init_governance_proxy(&self.account, eth_proxy_address, "eth_proxy_address : init_governance_proxy").await;
        sleep(Duration::from_secs(10)).await;
        init_governance_proxy(
            &self.account,
            eth_bridge_proxy_address,
            "eth_bridge_proxy_address : init_governance_proxy",
        )
        .await;
        sleep(Duration::from_secs(10)).await;

        let eth_bridge = StarknetLegacyEthBridge::deploy(self.core_contract.client().clone()).await;

        log::info!("✴️ ETH Bridge L1 deployment completed [Eth Bridge Address (L1) : {:?}]", eth_bridge.address());
        save_to_json("ETH_l1_bridge_address", &JsonValueType::EthAddress(eth_bridge.address())).unwrap();

        let account = build_single_owner_account(
            self.clients.provider_l2(),
            &self.arg_config.rollup_priv_key,
            &convert_to_hex(&self.account_address.to_string()),
            false,
        );

        let l2_bridge_address = StarknetLegacyEthBridge::deploy_l2_contracts(
            self.clients.provider_l2(),
            legacy_eth_bridge_class_hash,
            eth_bridge_proxy_address,
            &account,
        )
        .await;

        log::info!("✴️ ETH Bridge L2 deployment completed [Eth Bridge Address (L2) : {:?}]", l2_bridge_address);
        save_to_json("ETH_l2_bridge_address", &JsonValueType::StringType(l2_bridge_address.to_string())).unwrap();

        let eth_address = deploy_eth_token_on_l2(
            self.clients.provider_l2(),
            eth_proxy_address,
            erc20_legacy_class_hash,
            &account,
            l2_bridge_address,
        )
        .await;

        log::info!("✴️ L2 ETH token deployment successful.");
        // save_to_json("l2_eth_address", &JsonValueType::StringType(eth_address.to_string()))?;

        eth_bridge.initialize(self.core_contract.address()).await;
        log::info!("✴️ ETH Bridge initialization on L1 completed");

        sleep(Duration::from_secs(self.arg_config.l1_wait_time.parse().unwrap())).await;

        eth_bridge
            .setup_l2_bridge(
                self.clients.provider_l2(),
                l2_bridge_address,
                eth_address,
                &self.arg_config.rollup_priv_key,
                &account,
            )
            .await;
        log::info!("✴️ ETH Bridge initialization and setup on L2 completed");

        eth_bridge
            .setup_l1_bridge(
                "10000000000000000000000000000000000000000",
                "10000000000000000000000000000000000000000",
                l2_bridge_address,
            )
            .await;
        log::info!("✴️ ETH Bridge setup on L1 completed");

        EthBridgeSetupOutput {
            legacy_proxy_class_hash,
            starkgate_proxy_class_hash,
            erc20_legacy_class_hash,
            legacy_eth_bridge_class_hash,
            eth_proxy_address,
            eth_bridge_proxy_address,
            eth_bridge,
        }
    }
}

pub async fn deploy_eth_token_on_l2(
    rpc_provider_l2: &JsonRpcClient<HttpTransport>,
    eth_proxy_address: FieldElement,
    eth_erc20_class_hash: FieldElement,
    account: &RpcAccount<'_>,
    eth_legacy_bridge_address: FieldElement,
) -> FieldElement {
    let deploy_tx = account
        .invoke_contract(
            account.address(),
            "deploy_contract",
            vec![eth_erc20_class_hash, FieldElement::ZERO, FieldElement::ZERO, FieldElement::ZERO],
            None,
        )
        .send()
        .await
        .expect("Error deploying the contract proxy.");
    wait_for_transaction(rpc_provider_l2, deploy_tx.transaction_hash, "deploy_eth_token_on_l2 : deploy").await.unwrap();
    let contract_address = get_contract_address_from_deploy_tx(account.provider(), &deploy_tx).await.unwrap();

    log::debug!("Contract address (eth erc20) : {:?}", contract_address);

    let add_implementation_txn = invoke_contract(
        eth_proxy_address,
        "add_implementation",
        vec![
            contract_address,
            FieldElement::ZERO,
            FieldElement::from(4u64),
            FieldElement::from_byte_slice_be("Ether".as_bytes()).unwrap(),
            FieldElement::from_byte_slice_be("ETH".as_bytes()).unwrap(),
            FieldElement::from_str("18").unwrap(),
            eth_legacy_bridge_address,
            FieldElement::ZERO,
        ],
        account,
    )
    .await;

    wait_for_transaction(
        rpc_provider_l2,
        add_implementation_txn.transaction_hash,
        "deploy_eth_token_on_l2 : add_implementation",
    )
    .await
    .unwrap();

    let upgrade_to_txn = invoke_contract(
        eth_proxy_address,
        "upgrade_to",
        vec![
            contract_address,
            FieldElement::ZERO,
            FieldElement::from(4u64),
            FieldElement::from_byte_slice_be("Ether".as_bytes()).unwrap(),
            FieldElement::from_byte_slice_be("ETH".as_bytes()).unwrap(),
            FieldElement::from_str("18").unwrap(),
            eth_legacy_bridge_address,
            FieldElement::ZERO,
        ],
        account,
    )
    .await;

    wait_for_transaction(rpc_provider_l2, upgrade_to_txn.transaction_hash, "deploy_eth_token_on_l2 : upgrade_to")
        .await
        .unwrap();
    eth_proxy_address
}
