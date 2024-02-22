# EKOKE Liquidity Pool Canister

- [EKOKE Liquidity Pool Canister](#ekoke-liquidity-pool-canister)
  - [Introduction](#introduction)
  - [Swap Account](#swap-account)
  - [API](#api)
  - [HTTP API](#http-api)
    - [Request protocol](#request-protocol)
    - [Request Body](#request-body)
    - [HTTP Methods](#http-methods)
      - [liquidityPoolBalance](#liquiditypoolbalance)
      - [liquidityPoolAccounts](#liquiditypoolaccounts)

## Introduction

In order to guarantee a real value to the $EKOKE token, the EKOKE DAO manages a Liquidity Pool which has two accounts, the first is ICP and the second is ckBTC.

The liquidity pool can be funded by anyone by sending funds to the account returned by `liquidity_pool_accounts` call.

For each Deferred NFT sold on the marketplace to the contract buyer, the 10% of the amount paid is sent by to the liquidity pool. This guarantees that the value of a $EKOKE is at least 10% of a NFT value. (More or less, there are also other criteria which determines the value of the token).

There's no way for any person to withdraw funds in the **Liquidity Pool**, they are locked forever.

## Swap Account

In order to allow the exchange between the ICP sent after a sale on the marketplace into ckBTC, which will be considered as a value reserve, at the creation of the canister a Swap account is passed as argument.

The swap account is an ICRC account which must have a certain ckBTC amount of it and which will guarantee an allowance to the ekoke liquidity pool canister in order to spend their ckBTC. At this point once a day the canister will try to swap its ICP into ckBTC from the account.

The exchange ratio is calculated by using the XRC canister.

The amount sent will be the minimum between:

- the amount of ICP converted into ckBTC in the liquidity pool
- the amount of ckBTC on the swap account
- the allowance given to the ckBTC ledger for the swap account

## API

See full [DID](../../src/ekoke_liquidity_pool/ekoke-liquidity-pool.did)

## HTTP API

The deferred canister also exposes an HTTP API

### Request protocol

| Method       | GET              |
|--------------|------------------|
| Content-type | application/json |

### Request Body

The request body must be JSON encoded, and must follow this syntax

| Name   | Type   | Description                                  |
|--------|--------|----------------------------------------------|
| method | string | HTTP method name                             |
| params | json   | key-value map of optional request parameters |

### HTTP Methods

This list contains the http available methods with its parameters and response.

#### liquidityPoolBalance

Returns an object containing the balance for each account in the liquidity pool

```json
{
  "icp": nat,
  "ckbtc": nat
}
```

#### liquidityPoolAccounts

Returns the account for the liquidity pool

```json
{
  "icp": {
    "owner": "principal"
  },
  "ckbtc": {
    "owner": "principal"
  }
}
