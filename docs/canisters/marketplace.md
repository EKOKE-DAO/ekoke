# Marketplace

- [Marketplace](#marketplace)
  - [Introduction](#introduction)
  - [API](#api)
    - [get\_token\_price\_icp](#get_token_price_icp)
    - [buy\_token](#buy_token)
    - [admin\_set\_ekoke\_ledger\_canister](#admin_set_ekoke_ledger_canister)
    - [admin\_set\_ekoke\_liquidity\_pool\_canister](#admin_set_ekoke_liquidity_pool_canister)
    - [admin\_set\_deferred\_canister](#admin_set_deferred_canister)
    - [admin\_set\_xrc\_canister](#admin_set_xrc_canister)
    - [admin\_set\_icp\_ledger\_canister](#admin_set_icp_ledger_canister)
    - [admin\_set\_admins](#admin_set_admins)
    - [admin\_set\_interest\_rate\_for\_buyer](#admin_set_interest_rate_for_buyer)
    - [admin\_cycles](#admin_cycles)
  - [Buy process](#buy-process)

## Introduction

The Marketplace canister is the canister which takes care of intermediate the sell of the Deferred NFTs and to notify the ekoke-ledger canister to send reward to the first buyer of a NFT and to send funds to the ekoke-liquidity-pool canister.

From a technical perspective the marketplace canister doesn't provide any user interface, but just two call to provide a way to achieve the sell.

This means that anybody can implement their own interface for the ekoke marketplace.

## API

The full API can be seen on the canister [DID](../../src/marketplace/marketplace.did)

### get_token_price_icp

Get ICP token price for the provided token

### buy_token

See **Buy process**

### admin_set_ekoke_ledger_canister

set the ekoke ledger canister

### admin_set_ekoke_liquidity_pool_canister

set principal for ekoke-liquidity-pool canister

### admin_set_deferred_canister

update the principal for the deferred canister

### admin_set_xrc_canister

update the canister for XRC.

### admin_set_icp_ledger_canister

update the principal of the ICP ledger canister

### admin_set_admins

set canister administrators

### admin_set_interest_rate_for_buyer

set the interest rate for contract buyer.

### admin_cycles

get canister cycles

## Buy process

1. The user goes to the marketplace page of a token
2. The user calls with their IC wallet `get_token_icp` to marketplace to get the current price as $ICP token for the token
3. marketplace calls `deferred` to get the contract info for that token
4. deferred returns the contract to marketplace
5. marketplace set `fiat_value` to `token.value`
6. marketplace checks whether the caller is a contract buyer
   1. if so, it multiplies the fiat_value by `1.1`
7. marketplace sets `currency = contract.currency`
8. marketplace checks whether they already have an exchange rate between currency and ICP for the current date
   1. if not, it queries the XRC canister to get the current exchange rate
9. marketplace gets the ICP price for the contract value
10. marketplace sums to the price the ICP canister fee
    1. if the caller is buyer of the contract, it sums the fee twice
11. marketplace returns the ICP price to the user
12. the user gives allowance to the marketplace canister for the ICP value of the token
13. the user calls `buy_token` providing the token id.
14. marketplace verifies the given allowance for ICP is equal to the contract ICP value
15. marketplace checks whether the caller is a contract buyer
    1. if so, marketplace gets the principal of the liquidity pool from `ekoke-liquidity-pool`
    2. if so, marketplace sends the 10% of the ICP price to the liquidity pool calling `icrc2_transfer_from` on the ICP ledger canister
16. the marketplace sends to the current owner of the NFT the ICP value with `icrc2_transfer_from` on the ICP ledger
17. marketplace calls `transfer_from` on deferred to transfer the ownership of the NFT to the caller
18. marketplace verifies whether the token had a previous owner
    1. if true, marketplace calls `send_reward` on `ekoke-ledger` canister and sends the reward to the caller
19. marketplace verifies whether the caller is buyer of the contract
    1. if true, it calls `burn` on deferred canister and burns the NFT.
    2. if true, it transfers the interest on the liquidity pool
