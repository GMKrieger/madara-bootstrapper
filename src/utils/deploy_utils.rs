use std::sync::Arc;
use crate::snos::lib::SnosCodec;
use crate::utils::convert_felt_to_u256;
use crate::felt::lib::Felt252Wrapper;
use crate::messages::lib::{MessageL1ToL2, MessageL2ToL1};
use starknet_api::hash::StarkFelt;
use starknet_core_contract_client::clients::StarknetSovereignContractClient;
use starknet_core_contract_client::deploy_starknet_sovereign_behind_unsafe_proxy;
use starknet_core_contract_client::interfaces::{OperatorTrait, StarknetMessagingTrait};
use starknet_proxy_client::proxy_support::{
    CoreContractInitData, CoreContractState, ProxyInitializeData, ProxySupportTrait,
};
use zaun_utils::{LocalWalletSignerMiddleware, StarknetContractClient};
use ethereum_instance::EthereumInstance;
use ethers::{types::{Address, I256, U256}, utils::keccak256};
use super::arg_config::ArgConfig;
use starknet_ff::FieldElement;


pub struct DeployClients {
    eth_client: EthereumInstance,
    client: StarknetSovereignContractClient
}

impl DeployClients {

    pub fn address(&self) -> Address {
        self.client.address()
    }

    pub fn client(&self) -> Arc<LocalWalletSignerMiddleware> {
        self.client.client()
    }

    /// To deploy the instance of ethereum and starknet and returning the struct.
    pub async fn deploy(config: ArgConfig) -> Self {
        let client_instance = EthereumInstance::spawn(config.eth_rpc, config.eth_priv_key, config.eth_chain_id);

        let client = deploy_starknet_sovereign_behind_unsafe_proxy(client_instance.client()).await.expect("Failed to deploy the starknet contact");

        Self {
            eth_client: client_instance,
            client,
        }
    }

    /// Initialize Starknet core contract with the specified data.
    /// Also register Anvil default account as an operator.
    pub async fn initialize_with(&self, init_data: CoreContractInitData) {
        let data = ProxyInitializeData::<0> { sub_contract_addresses: [], eic_address: Default::default(), init_data };

        self.client.initialize_with(data).await.expect("Failed to initialize");

        self.client.register_operator(self.client.client().address()).await.expect("Failed to register operator");
    }

    /// Initialize Starknet core contract with the specified program and config hashes. The rest of parameters will be left default.
    /// Also register Anvil default account as an operator.
    pub async fn initialize(&self, program_hash: StarkFelt, config_hash: StarkFelt) {
        self.initialize_with(CoreContractInitData {
            program_hash: convert_felt_to_u256(program_hash),
            config_hash: convert_felt_to_u256(config_hash),
            ..Default::default()
        })
        .await;
    }

    /// Initialize Starknet core contract with the specified block number and state root hash.
    /// The program and config hashes will be set according to the Madara Goerli configuration.
    /// Also register Anvil default account as an operator.
    pub async fn initialize_for_goerli(&self, block_number: StarkFelt, state_root: StarkFelt) {
        // See SN_OS_PROGRAM_HASH constant
        let program_hash = StarkFelt::from(Felt252Wrapper::from(
            FieldElement::from_hex_be("0x41fc2a467ef8649580631912517edcab7674173f1dbfa2e9b64fbcd82bc4d79").unwrap(),
        ));

        // Hash version:        SN_OS_CONFIG_HASH_VERSION (settlement)
        // Chain ID:            SN_GOERLI_CHAIN_ID (pallet config)
        // Fee token address:   0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7 (genesis
        // config)
        let config_hash = StarkFelt::from(Felt252Wrapper::from(
            FieldElement::from_hex_be("0x036f5e4ea4dd042801c8841e3db8e654124305da0f11824fc1db60c405dbb39f").unwrap(),
        ));

        let init_data = CoreContractInitData {
            program_hash: convert_felt_to_u256(program_hash), // zero program hash would be deemed invalid
            config_hash: convert_felt_to_u256(config_hash),
            initial_state: CoreContractState {
                block_number: I256::from_raw(convert_felt_to_u256(block_number)),
                state_root: convert_felt_to_u256(state_root),
                ..Default::default()
            },
            ..Default::default()
        };

        self.initialize_with(init_data).await;
    }

    
    pub async fn send_message_to_l2(&self, message: &MessageL1ToL2) {
        self.client
            .send_message_to_l2(
                convert_felt_to_u256(message.to_address.0.0),
                convert_felt_to_u256(message.selector),
                message.payload.clone().into_iter().map(convert_felt_to_u256).collect(),
                1.into(),
            )
            .await
            .expect("Failed to call `send_message_to_l2`");
    }

    pub async fn message_to_l1_exists(&self, message: &MessageL2ToL1) -> bool {
        let message_felt_size = message.size_in_felts();
        let mut payload: Vec<u8> = Vec::with_capacity(32 * message_felt_size);
        message.clone().into_encoded_vec().into_iter().for_each(|felt| payload.append(&mut felt.bytes().to_vec()));

        let msg_hash = keccak256(payload);
        let res = self.client.l2_to_l1_messages(msg_hash).await.expect("Failed to call `l2_to_l1_messages`");

        res != U256::zero()
    }

    pub async fn message_to_l2_exists(&self, message: &MessageL1ToL2) -> bool {
        let mut payload: Vec<u8> = Vec::new();
        message.clone().into_encoded_vec().into_iter().for_each(|felt| payload.append(&mut felt.bytes().to_vec()));

        let msg_hash = keccak256(payload);
        let res = self.client.l1_to_l2_messages(msg_hash).await.expect("Failed to call `l2_to_l1_messages`");

        res != U256::zero()
    }
}