# Deferred

- [Deferred](#deferred)
  - [Introduction](#introduction)
  - [Contract creation](#contract-creation)
    - [Lazy minting](#lazy-minting)
  - [Token transfers](#token-transfers)

---

## Introduction

Deferred ERC721 is indeed an implementation in Solidity of a ERC721 standard to be able to implement an NFT for each mortgage installment to pay a real estate on the EKOKE DAO.

Each NFT URI points to the contract URL on the [deferred-data](../canisters/deferred-data.md) canister.

## Contract creation

NFTs on Deferred are minted with the contract creation.

The `createContract` method can only be called by the [deferred-minter](../canisters/deferred-minter.md) canister, which provides the following data:

```solidity
struct CreateContractRequest {
    /// @dev The id of the contract
    uint256 contractId;
    /// @dev metadata uri pointing to deferred-data canister uri
    string metadataUri;
    /// @dev Contract sellers
    SellerRequest[] sellers;
    /// @dev The contract buyers
    address[] buyers;
    /// @dev Reward for buying a token
    uint256 ekokeReward;
    /// @dev The price of the token in USD
    uint256 tokenPriceUsd;
    /// @dev the amount of tokens to mint
    uint256 tokensAmount;
}
```

Once called, if the data are valid, the deferred contract will call the [Reward Pool](./RewardPool.md) to reserve `ekokeReward * tokensAmount` EKOKE and finally it will **lazy mint** the tokens.

### Lazy minting

Since a contract can thousands of tokens, we can't of course mint all the tokens at the contract creation, so we had to find a way to make it extremely cheap: this method is called Lazy minting.

How does it work? So basically we know that all the tokens of a contract are actually all the same and all have the same owner. The only thing that can distinguish one token from another, is the ID.

So instead of actually minting them, we just store the token ID range in the contract data. So the only thing we actually store at the contract creation is the contract data with the ID range for the tokens.

The tokens are eventually phyisically minted when they are first bought. So when a token is bought, the `transferFrom` method is called and this causes the contract to check whether the contract is already minted or not. If it's not, the contract only at this point is minted to the new owner and so it becomes phyisically minted.

## Token transfers

It's important to know that **Deferred ERC721 can't be transferred by the token owner**, but only by the [Marketplace](./Marketplace.md). The marketplace is always allowed to transfer tokens, while the user is never allowed to. It's neither allowed to call `approve` on the contract.

This is of course done to prevent users from selling the tokens breaking their intrisic price, defined at the contract creation. The **token price will never change**.
