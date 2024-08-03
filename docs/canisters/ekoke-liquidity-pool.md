# EKOKE Liquidity Pool Canister

- [EKOKE Liquidity Pool Canister](#ekoke-liquidity-pool-canister)
  - [Introduction](#introduction)
  - [Refund to investors](#refund-to-investors)
  - [API](#api)
  - [HTTP API](#http-api)
    - [Request protocol](#request-protocol)
    - [Request Body](#request-body)
    - [HTTP Methods](#http-methods)
      - [liquidityPoolBalance](#liquiditypoolbalance)
      - [liquidityPoolAccounts](#liquiditypoolaccounts)

## Introduction

In order to guarantee a real value to the $EKOKE token, the EKOKE DAO manages a Liquidity Pool which has currently a ICP account.

The liquidity pool can be funded by anyone by sending funds to the account returned by `liquidity_pool_accounts` call.

For each Deferred NFT sold on the marketplace to the contract buyer, the 10% of the amount paid is sent by to the liquidity pool. This guarantees that the value of a $EKOKE is at least 10% of a NFT value. (More or less, there are also other criteria which determines the value of the token).

## Refund to investors

In case a contract fails to be paid, the liquidity pool receives the initial deposit sent by the buyer. The liquidity pool funds are also used to refund the third party investors.

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
  "icp": nat
}
```

#### liquidityPoolAccounts

Returns the account for the liquidity pool

```json
{
  "icp": {
    "owner": "principal"
  }
}
