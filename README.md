# EKOKE

<p align="center">
  <img src="./assets/images/ekoke-logo.png" alt="ekoke logo" width="256" />
</p>

![CI state](https://github.com/EKOKEtoken/ekoke/workflows/build-test/badge.svg)
![Ethereum](https://github.com/EKOKEtoken/ekoke/workflows/ethereum/badge.svg)
![Frontend](https://github.com/EKOKEtoken/ekoke/workflows/frontend/badge.svg)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-%23FE5196?logo=conventionalcommits&logoColor=white)](https://conventionalcommits.org)

Powered by **Internet Computer**

---

- [EKOKE](#ekoke)
  - [Introduction](#introduction)
    - [Objectives and Vision](#objectives-and-vision)
    - [Revolutionizing Real Estate](#revolutionizing-real-estate)
    - [Main advantages](#main-advantages)
  - [Documentation](#documentation)
  - [Whitepaper](#whitepaper)
  - [Ethereum Contracts](#ethereum-contracts)
  - [Canisters](#canisters)
  - [Get started](#get-started)
    - [Dependencies](#dependencies)
    - [Build canisters](#build-canisters)
  - [Project structure](#project-structure)
  - [Changelog](#changelog)
  - [License](#license)

---

## Introduction

The EKOKE DAO project emerges as the evolutionary response to the traditional real estate landscape, originating from a Milan-based agency's immersion into the realms of blockchain and cryptocurrencies. This innovative venture aspires to redefine real estate transactions by embracing the potential of the blockchain.

### Objectives and Vision

Our primary goal is to introduce a novel approach to real estate transactions by enabling the sale of houses in installments on the Ethereum blockchain and the Internet Computer blockchain as the database for sell contract data. The vision driving this project is rooted in the desire to streamline and optimize the real estate transaction process. By leveraging blockchain technology, we aim to simplify, economize, and democratize the way properties are bought and sold.

### Revolutionizing Real Estate

EKOKE DAO stands as a catalyst in revolutionizing the dynamics of real estate transactions. Key features include unprecedented speed, cost-effectiveness, decentralization, security, and confidentiality. By intertwining blockchain and cryptocurrencies with real estate, we embark on a journey to create a paradigm shift, making property transactions more accessible and efficient.

### Main advantages

The advantages offered by EKOKE DAO are multifaceted. Real estate transactions conducted on our platform are designed to be faster, more cost-efficient, decentralized to eliminate intermediaries, secure through blockchain's immutable ledger, and confidential to safeguard sensitive information. These transformative elements collectively contribute to a more inclusive and innovative real estate ecosystem.

## Documentation

Read the [Project Documentation](./docs/README.md)

## Whitepaper

You can find the whitepaper on our website <https://www.ekoketoken.com/whitepaper>

## Ethereum Contracts

- **Deferred**: [0xA0939B965AE2683DA136cFF37FC856Ca46c66Cd6](https://etherscan.io/address/0xA0939B965AE2683DA136cFF37FC856Ca46c66Cd6)
- **EKOKE**: [0x92fBA9067844A419A1C394197aE406768555F71b](https://etherscan.io/address/0x92fBA9067844A419A1C394197aE406768555F71b)
- **EKOKE-presale**: [0x0AA24850527dAC93EFA962E9D8a8E08f8DC083DF](https://etherscan.io/address/0x0AA24850527dAC93EFA962E9D8a8E08f8DC083DF)
- **Marketplace**: [0x70414531075AC3ca41a1Fca6217a977AF908a7E2](https://etherscan.io/address/0x70414531075AC3ca41a1Fca6217a977AF908a7E2)
- **RewardPool**: [0x161b3061b67C77bb866ECbA67Fa29936A51011F0](https://etherscan.io/address/0x161b3061b67C77bb866ECbA67Fa29936A51011F0)

## Canisters

- **deferred-data**: [2m6dw-uaaaa-aaaal-arumq-cai](https://dashboard.internetcomputer.org/canister/2m6dw-uaaaa-aaaal-arumq-cai)
- **deferred-minter**: [2f5ik-ciaaa-aaaal-aruna-cai](https://dashboard.internetcomputer.org/canister/2f5ik-ciaaa-aaaal-aruna-cai)
  - Ethereum Minter Address: [0x1b22db91529cefbd793e34e2f75d33ac744b1bcd](https://etherscan.io/address/0x1b22db91529cefbd793e34e2f75d33ac744b1bcd)
- **dapp**: [ygihe-dyaaa-aaaal-ai4la-cai](https://dashboard.internetcomputer.org/canister/ygihe-dyaaa-aaaal-ai4la-cai)

## Get started

### Dependencies

Before getting started with ekoke, you need to install these dependencies:

- Rust >= 1.74

    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

- Dfx >= 0.16

    ```sh
    sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)
    dfx extension install sns
    ```

- cargo-make

    ```sh
    cargo install cargo-make
    ```

- Wasm32 target

    ```sh
    rustup target add wasm32-unknown-unknown
    ```

### Build canisters

In order to build canister you need to setup the dfx environment and then build the source code, luckily all these steps are automated with cargo-make.

```sh
cargo make dfx-setup
cargo make dfx-build
```

---

## Project structure

The project is composed by the following components:

- **deferred**: A DIP721 ICP canister which represents the Deferred NFTs.
- **EKOKE**: A ICRC-2 token ICP canister which represents the fungible token $EKOKE.
  - **ekoke-erc20-swap**: Canister to swap EKOKE between ICRC and ERC20
  - **ekoke-erc20-swap-frontend**: Frontend for erc20-swap canister
  - **ekoke-liquidity-pool**: Canister which manages the EKOKE liquidity pool
  - **ekoke-reward-pool**: Canister which handles the reward pool for EKOKE
- **marketplace**: A canister which manages the Deferred tokens sell and $EKOKE rewards giveaway.

## Changelog

Read [CHANGELOG](./CHANGELOG.md)

## License

You can read the entire license [HERE](LICENSE)
