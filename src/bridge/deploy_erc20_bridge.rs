use std::str::FromStr;

use anyhow::Ok;
use sp_core::{H160, U256};
use starknet_ff::FieldElement;

use crate::contract_clients::config::Config;
use crate::contract_clients::eth_bridge::BridgeDeployable;
use crate::contract_clients::starknet_sovereign::StarknetSovereignContract;
use crate::contract_clients::token_bridge::StarknetTokenBridge;
use crate::utils::{save_to_json, JsonValueType};
use crate::CliArgs;

pub async fn deploy_erc20_bridge(
    clients: &Config,
    arg_config: &CliArgs,
    core_contract: &StarknetSovereignContract,
    account_address: &str,
    token_bridge_proxy_address: FieldElement,
) -> Result<(StarknetTokenBridge, FieldElement), anyhow::Error> {
    let token_bridge = StarknetTokenBridge::deploy(core_contract.client().clone()).await;

    log::debug!("Token Bridge Deployment Successful [✅]");
    log::debug!("[🚀] Token Bridge Address : {:?}", token_bridge.bridge_address());
    save_to_json("ERC20_l1_bridge_address", &JsonValueType::EthAddress(token_bridge.bridge_address()))?;
    save_to_json("ERC20_l1_registry_address", &JsonValueType::EthAddress(token_bridge.registry_address()))?;
    save_to_json("ERC20_l1_manager_address", &JsonValueType::EthAddress(token_bridge.manager_address()))?;

    let l2_bridge_address = StarknetTokenBridge::deploy_l2_contracts(
        clients.provider_l2(),
        &arg_config.rollup_priv_key,
        account_address,
        token_bridge_proxy_address,
    )
    .await;

    log::debug!("L2 Token Bridge Deployment Successful [✅]");
    log::debug!("[🚀] L2 Token Bridge Address : {:?}", l2_bridge_address);
    save_to_json("ERC20_l2_bridge_address", &JsonValueType::StringType(l2_bridge_address.to_string()))?;

    token_bridge.initialize(core_contract.address()).await;
    token_bridge
        .setup_l2_bridge(clients.provider_l2(), l2_bridge_address, &arg_config.rollup_priv_key, account_address)
        .await;
    token_bridge
        .setup_l1_bridge(
            H160::from_str(&arg_config.l1_deployer_address).unwrap(),
            l2_bridge_address,
            U256::from_dec_str("100000000000000").unwrap(),
        )
        .await;

    Ok((token_bridge, l2_bridge_address))
}

// async fn get_l2_token_address(
//     rpc_provider_l2: &JsonRpcClient<HttpTransport>,
//     l2_bridge_address: &FieldElement,
//     l1_erc_20_address: &H160,
// ) -> FieldElement {
//     rpc_provider_l2
//         .call(
//             FunctionCall {
//                 contract_address: *l2_bridge_address,
//                 entry_point_selector: get_selector_from_name("get_l2_token").unwrap(),
//                 calldata:
// vec![FieldElement::from_byte_slice_be(l1_erc_20_address.as_bytes()).unwrap()],             },
//             BlockId::Tag(BlockTag::Latest),
//         )
//         .await
//         .unwrap()[0]
// }
