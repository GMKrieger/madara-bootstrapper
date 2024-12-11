# Madara Bootstrapper 👾

[![Coverage Status](https://coveralls.io/repos/github/madara-alliance/madara-bootstrapper/badge.svg?branch=main)](https://coveralls.io/github/madara-alliance/madara-bootstrapper?branch=main)

Madara Bootstrapper is a tool that helps to deploy the **Token Bridge** & **Eth Bridge** contract
between a madara/katana Appchain and another L2 or L1 network. It will also declare wallet
contracts from **OpenZappelin**, **Argent** and **Braavos**. You can find the full list of contracts
in [Info](#info-ℹ)

## Index 📇

- [Madara Bootstrap 👾](#madara-bootstrap-)
  - [Index 📇](#index-)
  - [Testing 🛠️](#testing-)
    - [IMP 🚨](#imp-)
  - [Run 🚀](#run-)
    - [Local 💻](#local-)
    - [Docker 🐳](#docker-)
  - [Info ℹ️](#info-ℹ)
    - [Contract Descriptions 🗒️](#contract-descriptions-)
    - [Generate Subxt Artifacts 🔨](#to-generate-the-madara-subxt-artifacts-)

**Currently Supported :**

- Madara App Chain <----> Ethereum / EVM based chains
- 👷🏼 more coming soon......

## Testing 🛠️

There are three test in the repository :

- bridge deployment e2e
- eth bridge deposit and claim
- erc20 token bridge deposit and claim

### IMP 🚨

- You need to comment/remove the #[ignore] tags in [src/tests/mod.rs](src/tests/mod.rs) file
- Only one test can be run at one time as all the tests are e2e tests.
- You also would need to restart both the chains after running each test.
- You would need to clone [Madara](https://github.com/madara-alliance/madara.git) repo by running :
    ```shell
    git clone --branch d188aa91efa78bcc54f92aa1035295fd50e068d2 https://github.com/madara-alliance/madara.git
  ```

```shell
# 1. Run madara instance with eth as settlement layer :
./target/release/madara --dev --da-layer=ethereum --da-conf=examples/da-confs/ethereum.json --settlement=Ethereum --settlement-conf=examples/da-confs/ethereum.json
# 2. Run anvil instance
~/.foundry/bin/anvil

# 3. Run tests
RUST_LOG=debug cargo test -- --nocapture
```

## Run 🚀

### Local 💻

You can provide the env variables as arguments also, or you can also provide them in .env file.

Refer [.env.example](.env.example) file for setup

```shell
cp .env.example .env
cargo build --release
RUST_LOG=info cargo run -- --help

# If you have provided env vars in .env
RUST_LOG=info cargo run

# To run in dev mode (uses unsafe proxy and minimal setup)
RUST_LOG=info cargo run -- --dev
```

**IMP 🚨** : It will store all the addresses in [data/addresses.json](data/addresses.json)

### Docker 🐳

1. You need to set up the .env file first. Fill all the variables in .env file

   ```shell
   cp .env.example .env
   ```

2. You need to run docker compose command to build the image

   ```shell
   docker compose build
   ```

3. Run the image

   ```shell
   # If both the networks are running locally
   docker compose -f docker-compose-local.yml up
   # If you are hosting on already deployed networks
   docker compose up
   ```

**IMP 🚨** : It will store all the addresses in [data/addresses.json](data/addresses.json)

## Info ℹ️

### Contract Descriptions 🗒️

| Contract                                      | Source Link                                                                                             | Local Path                                                                                                       |
| --------------------------------------------- |---------------------------------------------------------------------------------------------------------| ---------------------------------------------------------------------------------------------------------------- |
| OpenZeppelinAccount (legacy : starknet)       | <https://sepolia.starkscan.co/class/0x05c478ee27f2112411f86f207605b2e2c58cdb647bac0df27f660ef2252359c6> | [src/contracts/OpenZeppelinAccount.json](./src/contracts/OpenZeppelinAccount.json)                               |
| OpenZeppelinAccount (modified : openzeppelin) | [src/contracts/account.cairo](src/contracts/account.cairo)                                              | [src/contracts/OpenZeppelinAccountCairoOne.sierra.json](./src/contracts/OpenZeppelinAccountCairoOne.sierra.json) |
| UDC (Universal Deployer Contract)             | <https://sepolia.starkscan.co/class/0x07b3e05f48f0c69e4a65ce5e076a66271a527aff2c34ce1083ec6e1526997a69> | [src/contracts/udc.json](./src/contracts/udc.json)                                                               |

Here are some contract descriptions on why they are used
in our context.

- `OpenZeppelinAccount (legacy : starknet)` : Contract used for declaring a temp account for declaring V1
  contract that will be used to deploy the user account with provided private key in env.
- `OpenZeppelinAccount (modified : openzeppelin)` : OZ account contract modified to include `deploy_contract`
  function as we deploy the UDC towards the end of the bootstrapper setup.

> [!IMPORTANT]
> For testing in Github CI we are using the madara binary build with
> `--disable-fee-flag`. The source for madara code :
> <https://github.com/karnotxyz/madara/tree/madara-ci-build>
