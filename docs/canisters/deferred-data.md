# Deferred Data

- [Deferred Data](#deferred-data)
  - [Introduction](#introduction)
  - [HTTP Endpoint](#http-endpoint)
    - [Get contracts](#get-contracts)
    - [Get contract by id](#get-contract-by-id)
  - [Contract Properties](#contract-properties)

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

## Contract Properties

These are the Properties that may be inserted into the Contract.

| Property              | Type         | Description |
|-----------------------|--------------|------------------------------------------------|
| contract:name         | TextContent  | A title for the property                       |
| contract:description  | TextContent  | A description for the property                 |
| contract:image        | TextContent  | URL or base64 encoded image                    |
| contract:address      | TextContent  | Address where the property is located          |
| contract:country      | TextContent  | Country where the property is located          |
| contract:continent    | TextContent  | Continent where the property is located        |
| contract:region       | TextContent  | Region where the property is located           |
| contract:zipCode      | TextContent  | Zip code where the property is located         |
| contract:latitude     | TextContent  | Latitude where the property is located         |
| contract:longitude    | TextContent  | Longitude where the property is located        |
| contract:zone         | TextContent  | Zone of the city where the property is located |
| contract:city         | TextContent  | City where the property is located             |
| contract:squareMeters | Nat64Content | Property square meters                         |
| contract:rooms        | Nat64Content | Amount of rooms                                |
| contract:bathrooms    | Nat64Content | Amount of Bathrooms                            |
| contract:floors       | Nat64Content | Floors                                         |
| contract:balconies    | Nat64Content | Amount of balconies                            |
| contract:garden       | BoolContent  | Has garden                                     |
| contract:pool         | BoolContent  | Has pool                                       |
| contract:garage       | BoolContent  | Has garage                                     |
| contract:parking      | BoolContent  | Has a private parking                          |
| contract:energyClass  | TextContent  | Optional energy class                          |
| contract:youtubeUrl   | TextContent  | URL to a YouTube video showcasing the property |
