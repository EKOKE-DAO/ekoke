# EKOKE

![CI state](https://github.com/EKOKEtoken/ekoke/workflows/build-test/badge.svg)
![Ethereum](https://github.com/EKOKEtoken/ekoke/workflows/ethereum/badge.svg)
![Frontend](https://github.com/EKOKEtoken/ekoke/workflows/frontend/badge.svg)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-%23FE5196?logo=conventionalcommits&logoColor=white)](https://conventionalcommits.org)

<img src="./assets/images/deferred-logo.png" alt="deferred logo" width="128" />

<img src="./assets/images/ekoke-logo.png" alt="ekoke logo" width="128" />

<img src="./assets/images/icp-logo.svg" alt="icp-logo" width="128" />

Powered by **Internet Computer**

---

- [EKOKE](#ekoke)
  - [Documentation](#documentation)
  - [Get started](#get-started)
  - [Project structure](#project-structure)
  - [Changelog](#changelog)
  - [License](#license)

---

## Documentation

Read the [Project Documentation](./docs/README.md)

## Get started

...

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
