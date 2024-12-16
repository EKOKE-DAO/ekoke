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
  - [Documentation](#documentation)
  - [Whitepaper](#whitepaper)
  - [Ethereum Contracts](#ethereum-contracts)
  - [Canisters](#canisters)
  - [Get started](#get-started)
    - [Dependencies](#dependencies)
    - [Build canisters](#build-canisters)
  - [Changelog](#changelog)
  - [License](#license)

---

## Introduction

This repository contains the codebase for the deferred_data and deferred_minter canisters and the Ethereum contracts for the EKOKE DAO.

## Documentation

Read the on the DAO website: <https://ekokedao.com/documentation>.

## Whitepaper

You can find the whitepaper on our website <https://www.ekoketoken.com/whitepaper>

## Ethereum Contracts

- **Deferred**: [0xA0939B965AE2683DA136cFF37FC856Ca46c66Cd6](https://etherscan.io/address/0xA0939B965AE2683DA136cFF37FC856Ca46c66Cd6)
- **EKOKE**: [0x92fBA9067844A419A1C394197aE406768555F71b](https://etherscan.io/address/0x92fBA9067844A419A1C394197aE406768555F71b)
- **EKOKE-presale**: [0xa5fF566D68E3521929F47447b975C4683618C35f](https://etherscan.io/address/0xa5fF566D68E3521929F47447b975C4683618C35f)
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

- Dfx >= 0.24.1

    ```sh
    sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
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

## Changelog

Read [CHANGELOG](./CHANGELOG.md)

## License

You can read the entire license [HERE](LICENSE)
