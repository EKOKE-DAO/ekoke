# Deferred Data

- [Deferred Data](#deferred-data)
  - [Introduction](#introduction)
  - [HTTP Endpoint](#http-endpoint)
    - [Get contracts](#get-contracts)
    - [Get contract by id](#get-contract-by-id)
    - [Get real estates](#get-real-estates)
    - [Get real estate by id](#get-real-estate-by-id)

Principal: `2m6dw-uaaaa-aaaal-arumq-cai`

## Introduction

Deferred **Data** canister takes care of storing sell contracts and provides the following functionalities:

- **Create contract**: the contract is inserted into the ledger by [deferred-minter](./deferred-minter.md).
- **Close contract**: the contract is closed by [deferred-minter](./deferred-minter.md).
- **Get contract data**: get the data for a contract. Closed contracts are not returned
- **Get all contracts**: get all existing contracts. Closed contracts are not returned
- **Get contract document**: get a contract document with its data and mime type
- **Upload contract document**: The agency can upload documents for a contract
- **Update contract property**: The agency can both update a contract property and restricted property. Mind that when we talk about **contract properties** we don't mean any property, but just those stored in the `properties` and `restricted_properties` fields.
- **Create real estate**: define a new real estate property
- **Get real estate**: get a real estate property by its ID
- **Delete real estate**: delete a real estate property by its ID
- **Update real estate**: update a real estate property by its ID

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

It is also possible to filter contracts using query params:

- seller: seller ETH address
- buyer: buyer ETH address
- agent: agency principal
- minPrice: minimum price
- maxPrice: maximum price (price is)
- position: check if contract property is in a certain range. The following keys are required
  - `latitude`
  - `longitude`
  - `radius` (Km)
- contract_property: name of the contract property followed by the value (e.g. `contract:garden` => `garden=true`).

URL with query params

```txt
GET /contracts?latitude=45.04&longitude=9.89&radius=20&minPrice=10000&maxPrice=2100000&seller=0xE46A267b65Ed8CBAeBA9AdC3171063179b642E7A&buyer=0x0b24F78CF0033FAbf1977D9aA61f583fBF7586D9&garden&city=london
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
      "mimeType": "application/pdf",
      "name": "Document-1",
      "size" 123
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

### Get real estates

This endpoints gets all the IDs of registered contracts

```txt
GET /real-estate
```

Response:

```json
[
  1,
  2,
  3
]
```

It is also possible to filter contracts using query params:

- agent: agency principal
- minPrice: minimum price
- maxPrice: maximum price (price is)
- position: check if contract property is in a certain range. The following keys are required
  - `latitude`
  - `longitude`
  - `radius` (Km)
- name
- description
- address
- country
- continent
- region
- zipCode
- zone
- city
- squareMeters
- rooms
- bathrooms
- floors
- balconies
- garden
- pool
- garage
- parking
- energyClass

URL with query params

```txt
GET /real-estate?latitude=45.04&longitude=9.89&radius=20&garden&city=london
```

### Get real estate by id

Get a contract by id

```txt
GET /real-estate/:id
```

Response:

```json
{
  "id": 1,
  "name": "Villa in the hills",
  "description": "A beautiful villa in the hills",
  "address": "Via Roma 1",
  "country": "Italy",
  "continent": "Europe",
  "region": "Lombardia",
  "zipCode": "20100",
  "latitude": "45.04",
  "longitude": "9.89",
  "zone": "Hills",
  "city": "Milan",
  "squareMeters": 200,
  "rooms": 5,
  "bathrooms": 3,
  "floors": 2,
  "balconies": 2,
  "garden": true,
  "pool": true,
  "garage": true,
  "parking": true,
  "energyClass": "A",
}
```
