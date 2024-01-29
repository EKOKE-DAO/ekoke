# Deferred

![CI state](https://github.com/veeso-dev/deferred/workflows/build-test/badge.svg)
![Ethereum](https://github.com/veeso-dev/deferred/workflows/ethereum/badge.svg)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-%23FE5196?logo=conventionalcommits&logoColor=white)](https://conventionalcommits.org)

<img src="./docs/images/logo.png" alt="deferred logo" width="128" />

<img src="./docs/images/icp-logo.svg" alt="icp-logo" width="128" />

Powered by **Internet Computer**

---

- [Deferred](#deferred)
  - [Introduction](#introduction)
    - [Project Purpose](#project-purpose)
    - [Project components](#project-components)
    - [Sell flow](#sell-flow)
  - [Project structure](#project-structure)
  - [License](#license)

---

## Introduction

### Project Purpose

The project aims to facilitate the sale of a property between two or more parties, referred to in this document as A (seller) and B (buyer/s), using NFTs that serve as promissory notes.

### Project components

Deferred consists of three main canisters:

- **Deferred**: This NFT canister is used to define the sell of a real estate between two parts (the buyer and the seller).
- **EKOKE Token**: A fungible token ERC20-like, deflationary, used as an incentive to promote the buying of the NFTs and the adoption of the "Deferred method".
- **Marketplace**: A D-APP which permits to trade Deferred NFTs.

### Sell flow

The sales process consists of the following steps:

1. A lists their property for sale at a price X.
2. B agrees with A to purchase the property at the established price.
3. On the "IPC" blockchain, "n" Deferred tokens are minted, each with a value of X/n. The NFTs are transferred to A.
4. To acquire the property, B must buy all the NFTs from A at the agreed-upon price.
5. A can sell their NFTs to either B or third parties at the established price to generate liquidity for the property sale.
6. Whenever an NFT is sold, the buyer, as long as they purchase it from A, receives "Y" Ekoke.
7. The Deferred tokens, even after being transferred to a third-party owner, are always available for sale, but there will be no further receipt of Ekoke for each subsequent sale.
8. Once all Deferred tokens are in possession of B, the property officially becomes B's ownership.

## Project structure

The project is composed by the following components:

- **deferred**: A DIP721 ICP canister which represents the Deferred NFTs.
- **Ekoke**: A ICRC-2 token ICP canister which represents the fungible token $EKOKE.
- **Marketplace**: A canister which manages the Deferred tokens sell.
- **RegisterUI**: A website for the real estate agency to register sell-contracts.
- **MarketplaceUI**: A website for the marketplace canister, where the users can buy and sell theirs Deferreds.

## License

You can read the entire license [HERE](LICENSE)
