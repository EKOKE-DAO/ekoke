# EKOKE Index canister

- [EKOKE Index canister](#ekoke-index-canister)
  - [Introduction](#introduction)
  - [API](#api)
    - [ledger\_id](#ledger_id)
    - [list\_subaccounts](#list_subaccounts)
    - [get\_account\_transactions](#get_account_transactions)
    - [commit](#commit)

## Introduction

The ekoke-index canister takes care of providing the transaction history for each account.

## API

See the full documentation on the [DID](../../src/ekoke_index/ekoke-index.did)

This API implements the index canister standard according to the SNS DAO.

### ledger_id

Returns the ledger ID

### list_subaccounts

Get all the subaccounts for a given principal

### get_account_transactions

Get the transactions for a given account

### commit

This endpoint is called by the ekoke-ledger to commit a transaction into the index.
