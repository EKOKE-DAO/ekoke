# Marketplace

- [Marketplace](#marketplace)
  - [Introduction](#introduction)
  - [Buy process](#buy-process)
    - [The user gives USDT allowance to the Marketplace to buy the desider token](#the-user-gives-usdt-allowance-to-the-marketplace-to-buy-the-desider-token)
    - [Buy token](#buy-token)

---

## Introduction

The marketplace contract has the purpose of being able to buy Deferred ERC721 tokens using USDT.

The token buyer will also receive EKOKE tokens as a reward as described in the [reward document](../reward.md) with the reward amount established at the contract registration.

Mind that **ONLY the first token buyer will receive the reward**.

If the Buyer of the contract buys the token, he will pay the token price plus the interest rate. The interest rate is then transferred to the **Marketplace wallet**, where it will be sealed to back the EKOKE token value.

## Buy process

The Marketplace contract provides two functions useful to buy a process, the `buyToken` method and the `tokenPriceForCaller` view method.

```solidity
function tokenPriceForCaller(
    uint256 _tokenId
) external view returns (uint256) {}
```

```solidity
function buyToken(uint256 _tokenId) external {}
```

So the buy process is divided in two parts.

### The user gives USDT allowance to the Marketplace to buy the desider token

1. The user calls `tokenPriceForCaller` providing the ID of the token he wills to buy
2. The contract returns the USDT price to pay for the token
   1. `sellContract.tokenPriceUsd` if the caller is a third party
   2. `sellContract.tokenPriceUsd + interests(sellContract.tokenPriceUsd)` if the caller is a contract buyer
3. The user gives allowance to `usdErc20()` to the address for the returned amount.

### Buy token

At this point the user can call `buyToken` providing the ID of the token he got the price for.

At this point the smart contract will perform the following operations:

1. Get the contract information related to the provided tokenId
2. Checks whether the caller is a contract buyer
3. Checks whether that token has already been sold on the marketplace
4. `shouldSendReward = contract.ekokeReward > 0 && !tokenAlreadySold`
5. Calculates the pay the caller will pay (as seen before)
6. Checks whether the caller has given allowance for the price to pay
7. The contract transfers USDT for an amount of `tokenPriceUsd` to the current NFT owner
8. If the caller is the deferred contract buyer, the contract transfers the interest rate of `tokenPriceUsd` to the Marketplace wallet
9. The contract transfers the NFT to the caller using `safeTransferFrom` on `Deferred`
