# EKOKE

![CI state](https://github.com/EKOKEtoken/ekoke/workflows/build-test/badge.svg)
![Ethereum](https://github.com/EKOKEtoken/ekoke/workflows/ethereum/badge.svg)
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
  - **ekoke-archive**: block history for ekoke transactions
  - **ekoke-index**: Index canister for ekoke
  - **ekoke-ledger**: A ICRC-2 token ICP canister which represents the fungible token $EKOKE.
  - **ekoke-swap**: A simple web canister to swap Ekoke tokens between ERC20 <> ICRC2 token
- **marketplace**: A canister which manages the Deferred tokens sell and $EKOKE rewards giveaway.

## Changelog

Read [CHANGELOG](./CHANGELOG.md)

## License

You can read the entire license [HERE](LICENSE)
