# Deferred Minter

- [Deferred Minter](#deferred-minter)
  - [Introduction](#introduction)
    - [Create a sell contract](#create-a-sell-contract)
      - [create contract requirements](#create-contract-requirements)
      - [Create contract](#create-contract)
    - [Close a sell contract](#close-a-sell-contract)
      - [close contract requirements](#close-contract-requirements)
      - [Close contract](#close-contract)

## Introduction

Deferred **Minter** canister takes care of these two use cases:

- Create a **Sell contract**
- Close a **Sell contract**

Let's see them in details in how their process works.

### Create a sell contract

#### create contract requirements

A party involved in the sell process (buyer/seller/agency) must send Ethereum to pay the fee to the minter

1. Get the Ethereum address of the minter with `get_eth_address`
2. Send to the minter the amount of ethereum to cover a fee of `700_000` wei. Go to the gas tracker <https://etherscan.io/gastracker> and insert `700000` in `Custom gas limit` to see the required fee. To be sure the process works, add a 10-15% to the fee.

#### Create contract

At this point the agency can send the `ContractRegistration` data and call the `create_contract` endpoint on the canister.

This endpoint will call `create_contract` on the **Deferred** Ethereum ERC721 which will mint the tokens and after that it will call `create_contract` on **deferred_data** to store the contract on the ledger.

After that the NFTs are lazy-generated on the Ethereum smart contract and are owned by the sellers based on their share (quota) defined in the contract data.

### Close a sell contract

#### close contract requirements

A party involved in the sell process (buyer/seller/agency) must send Ethereum to pay the fee to the minter

1. Get the Ethereum address of the minter with `get_eth_address`
2. Send to the minter the amount of ethereum to cover a fee of `80_000` wei. Go to the gas tracker <https://etherscan.io/gastracker> and insert `80000` in `Custom gas limit` to see the required fee. To be sure the process works, add a 10-15% to the fee.

#### Close contract

At this point the agency can close the contract by calling `close_contract` on the canister.

This will mark the contract as closed both on the ledger and on the ERC721.

Once the contract is closed tokens can't be traded anymore and the sell contract is completed.

> ❗ The agency must ensure before closing the contract that the buyer owns all the tokens
