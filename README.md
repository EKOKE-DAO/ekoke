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
  - [Canisters](#canisters)
  - [Get started](#get-started)
    - [Dependencies](#dependencies)
    - [Build canisters](#build-canisters)
  - [Project structure](#project-structure)
  - [Changelog](#changelog)
  - [License](#license)

---

## Introduction

The Ekoketoken project emerges as the evolutionary response to the traditional real estate landscape, originating from a Milan-based agency's immersion into the realms of blockchain and cryptocurrencies. This innovative venture aspires to redefine real estate transactions by embracing the potential of the Internet Computer (ICP) blockchain.

### Objectives and Vision

Our primary goal is to introduce a novel approach to real estate transactions by enabling the sale of houses in installments on the ICP blockchain. The vision driving this project is rooted in the desire to streamline and optimize the real estate transaction process. By leveraging blockchain technology, we aim to simplify, economize, and democratize the way properties are bought and sold.

### Revolutionizing Real Estate

Ekoketoken stands as a catalyst in revolutionizing the dynamics of real estate transactions. Key features include unprecedented speed, cost-effectiveness, decentralization, security, and confidentiality. By intertwining blockchain and cryptocurrencies with real estate, we embark on a journey to create a paradigm shift, making property transactions more accessible and efficient.

### Main advantages

The advantages offered by Ekoketoken are multifaceted. Real estate transactions conducted on our platform are designed to be faster, more cost-efficient, decentralized to eliminate intermediaries, secure through blockchain's immutable ledger, and confidential to safeguard sensitive information. These transformative elements collectively contribute to a more inclusive and innovative real estate ecosystem.

## Documentation

Read the [Project Documentation](./docs/README.md)

## Whitepaper

You can find the whitepaper on our website <https://www.ekoketoken.com/whitepaper>

## Canisters

- **Deferred**: `v5vof-zqaaa-aaaal-ai5cq-cai`
- **EKOKE Liquidity Pool**: `v2uir-uiaaa-aaaal-ai5ca-cai`
- **EKOKE Reward Pool**: `vtxdn-caaaa-aaaal-ai5dq-cai`
- **Marketplace**: `vuwfz-pyaaa-aaaal-ai5da-cai`

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
