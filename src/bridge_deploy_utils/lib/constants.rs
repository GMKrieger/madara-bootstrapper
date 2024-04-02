use starknet_ff::FieldElement;

pub const TEST_CONTRACT_ADDRESS: &str = "0x0000000000000000000000000000000000000000000000000000000000001111";
pub const ACCOUNT_CONTRACT: &str = "0x0000000000000000000000000000000000000000000000000000000000000001";
pub const CAIRO_1_ACCOUNT_CONTRACT: &str = "0x0000000000000000000000000000000000000000000000000000000000000004";
pub const CAIRO_1_ACCOUNT_CONTRACT_CLASS_HASH: &str =
    "0x035ccefcf9d5656da623468e27e682271cd327af196785df99e7fee1436b6276";
pub const ERC20_CAIRO_0_CONTRACT: &str = "0x040e59c2c182a58fb0a74349bfa4769cbbcba32547591dd3fb1def8623997d00";
pub const SEQUENCER_ADDRESS: &str = "0xdead";
pub const UDC_ADDRESS: &str = "0x041a78e741e5af2fec34b695679bc6891742439f7afb8484ecd7766661ad02bf";

// https://github.com/keep-starknet-strange/madara/blob/main/crates/node/src/chain_spec.rs#L185-L186
pub const ACCOUNT_CONTRACT_CLASS_HASH: &str = "0x0279d77db761fba82e0054125a6fdb5f6baa6286fa3fb73450cc44d193c2d37f";
pub const ARGENT_PROXY_CLASS_HASH: &str = "0x0424b7f61e3c5dfd74400d96fdea7e1f0bf2757f31df04387eaa957f095dd7b9";
pub const ARGENT_ACCOUNT_CLASS_HASH_CAIRO_0: &str =
    "0x06f0d6f6ae72e1a507ff4b65181291642889742dbf8f1a53e9ec1c595d01ba7d";
pub const SIGNER_PUBLIC: &str = "0x03603a2692a2ae60abb343e832ee53b55d6b25f02a3ef1565ec691edc7a209b2";
pub const SIGNER_PRIVATE: &str = "0x00c1cf1490de1352865301bb8705143f3ef938f97fdf892f1090dcb5ac7bcd1d";
pub const SALT: &str = "0x0000000000000000000000000000000000000000000000000000000000001111";

// https://github.com/keep-starknet-strange/madara/blob/main/crates/node/src/chain_spec.rs#L191-L192
pub const TEST_CONTRACT_CLASS_HASH: &str = "0x04c5efa8dc6f0554da51f125d04e379ac41153a8b837391083a8dc3771a33388";
pub const MINT_AMOUNT: &str = "0x0000000000000000000000000000000000000000000000000000000000000001";
pub const DEPLOY_ACCOUNT_COST: &str = "0x00000000000000000000000000000000000000000000000000000000ffffffff";
pub const CONTRACT_ADDRESS: &str = "0x0000000000000000000000000000000000000000000000000000000000000001";
pub const FEE_TOKEN_ADDRESS: &str = "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7";
pub const TOKEN_CLASS_HASH: &str = "0x0000000000000000000000000000000000000000000000000000000000010000";
pub const ARGENT_CONTRACT_ADDRESS: &str = "0x0000000000000000000000000000000000000000000000000000000000000002";
pub const OZ_CONTRACT_ADDRESS: &str = "0x0000000000000000000000000000000000000000000000000000000000000003";

pub const L1_CONTRACT_ADDRESS: &str = "0xae0ee0a63a2ce6baeeffe56e7714fb4efe48d419";

pub const MAX_U256: &str = "0xffffffffffffffffffffffffffffffff";
pub const MAX_FEE_OVERRIDE: &str = "0x100000";

// Need to use `from_mont` because this needs to be a constant function call
/// ChainId for Starknet Goerli testnet
pub const SN_GOERLI_CHAIN_ID: FieldElement =
    FieldElement::from_mont([3753493103916128178, 18446744073709548950, 18446744073709551615, 398700013197595345]);

pub const SPEC_VERSION: &str = "0.4.0";

pub const ANVIL_DEFAULT_PUBLIC_ADDRESS: &str = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";
pub const ERC20_SIERRA_PATH: &str = "../starknet-e2e-test/contracts/erc20.sierra.json";
pub const ERC20_CASM_PATH: &str = "../starknet-e2e-test/contracts/erc20.casm.json";

pub const LEGACY_BRIDGE_PATH: &str = "../starknet-e2e-test/contracts/legacy_token_bridge.json";
pub const TOKEN_BRIDGE_SIERRA_PATH: &str = "../starknet-e2e-test/contracts/token_bridge.sierra.json";
pub const TOKEN_BRIDGE_CASM_PATH: &str = "../starknet-e2e-test/contracts/token_bridge.casm.json";
