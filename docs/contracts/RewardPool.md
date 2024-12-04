# Reward Pool

- [Reward Pool](#reward-pool)
  - [Introduction](#introduction)

---

## Introduction

The reward pool contract is used to track the current reward pools reserved for each contract and to distribute the rewards to the users who buy NFTs on the [Marketplace](./Marketplace.md).

The Reward Pool contract provides two methods, one to reserve the pool `reservePool`, which is called by [Deferred](./Deferred.md) to reserve the pool when a **Sell contract** is being created, and a method `sendReward` called by [Marketplace](./Marketplace.md) to send the reward to the user who is buying the NFT on the marketplace.
