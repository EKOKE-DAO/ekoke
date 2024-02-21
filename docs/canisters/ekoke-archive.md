# EKOKE-archive

- [EKOKE-archive](#ekoke-archive)

## Introduction

The ekoke-index canister takes care of providing the transaction history for each account.

## API

See the full documentation on the [DID](../../src/ekoke_archive/ekoke-archive.did)

This API implements the archive canister standard according to the SNS DAO.

### append_blocks

It is called by ekoke-ledger to append blocks.

> Currently this method is not used or implemented. Currently we use just the commit method to achieve this

### get_blocks

Get blocks.

> Currently, this method is not implemented and no matter what, it returns always an empty vec

### get_transaction

Get transaction by id

### get_transactions

Get transactions by count and offset

### remaining_capacity

Get block remaining capacity (Max is 10GB).

### commit

This endpoint is called by the ekoke-ledger to commit a transaction into the index.
Sequentially, it calls `commit` on the ekoke-index to synchronize the indexes.
