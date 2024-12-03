# Deferred Data

- [Deferred Data](#deferred-data)
  - [Introduction](#introduction)
  - [HTTP Endpoint](#http-endpoint)
    - [Get contracts](#get-contracts)
    - [Get contract by id](#get-contract-by-id)

## Introduction

Deferred **Data** canister takes care of storing sell contracts and provides the following functionalities:

- **Create contract**: the contract is inserted into the ledger by [deferred-minter](./deferred-minter.md).
- **Close contract**: the contract is closed by [deferred-minter](./deferred-minter.md).
- **Get contract data**: get the data for a contract. Closed contracts are not returned
- **Get all contracts**: get all existing contracts. Closed contracts are not returned
- **Get contract document**: get a contract document with its data and mime type
- **Upload contract document**: The agency can upload documents for a contract
- **Update contract property**: The agency can both update a contract property and restricted property. Mind that when we talk about **contract properties** we don't mean any property, but just those stored in the `properties` and `restricted_properties` fields.

## HTTP Endpoint

### Get contracts

This endpoints gets all the IDs of registered contracts

```txt
GET /contracts
```

Response:

```json
[
  1,
  2,
  3
]
```

### Get contract by id

Get a contract by id

```txt
GET /contract/:id
```

Response:

```json
{
  "id": 1,
  "type": "Sell",
  "sellers": [
    {
      "address": "0x...",
      "quota": 100
    }
  ],
  "buyers": [
    "0x000",
    "0x001",
  ],
  "installments": 4000,
  "value": 400000,
  "deposit": 50000,
  "currency": "USD",
  "properties": [],
  "restrictedProperties": [],
  "documents": {
    "1": {
      "accessList": ["agent"],
      "mimeType": "application/pdf"
    }
  },
  "agency": {
    ...
  },
  "expiration": "2050-01-1",
  "closed": false
}
```

> Restricted properties are redacted based on your permissions
