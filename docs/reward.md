# EKOKE Rewards

- [EKOKE Rewards](#ekoke-rewards)
  - [Introduction](#introduction)
  - [When are rewards distributed](#when-are-rewards-distributed)
  - [Reward Pool](#reward-pool)
    - [How to reserve a manual pool](#how-to-reserve-a-manual-pool)
  - [Reward Deflationary Algorithm](#reward-deflationary-algorithm)
    - [Reward Multiplier Coefficient](#reward-multiplier-coefficient)
    - [Avidity](#avidity)
    - [Token Price](#token-price)
    - [Reward Analysis](#reward-analysis)
      - [Fast growhth scenario](#fast-growhth-scenario)
      - [Slow growth scenario](#slow-growth-scenario)
      - [Exponential cases](#exponential-cases)

## Introduction

This document describes how EKOKE manages the EKOKE token rewards.

## When are rewards distributed

Rewards are automatically given to the buyer of a **Deferred ERC721** when he buys a token on the [Marketplace contract](./contracts/marketplace.md).

## Reward Pool

The reward pool can work in two different modes:

- **Automatic Mode:** If there is still available liquidity in the reward pool, then this mode should be preferred. Basically when a contract is created, the deferred canister will query the reward pool to check whether there is already a pool associated to that contract. If there's not a certain liquidity will be reserved as reward for token buyers following the Reward Pool Algorithm, which you can read about in the next chapter.
- **Manual Mode**: A user can opt to reserve a EKOKE pool using its EKOKE tokens for a contract before is created. This mode becomes mandatory in the case the liquidity pool of the reward canister is empty.

### How to reserve a manual pool

In order to reserve a manual pool, the user must burn a certain amount of their tokens. Once their tokens are burned, the reward pool can mint those tokens again as rewards.

## Reward Deflationary Algorithm

This algorithm is applied only on contract registered when the **Automatic Mode** is used.

Once a contract is registered on Deferred, Deferred will call the reward pool canister to collect the reward to assign for each contract's token sold.

The reward, if a pool is still not reserved for the contract will be calculated using the following formula:

```txt
reward = (rewardPoolLiquidity * RMC * avidity * tokenPrice) / 100
```

![reward-formula](../../assets/images/reward-formula.png)

Where:

- RMC is the "reward multiplier coefficient"
- avidity is the "preservation pool coefficient"
- token value: the price of a token for the contract

The reward will never be lower than ICRC_FEE * 10 though.

### Reward Multiplier Coefficient

The RMC is a coefficient which is used to establish the reward for a contract, trough the multiplication of the pool liquidity.

The initial value of the RMC will be `0.0000042` and the value will be halved each 4 years.

Its value will never reach a value under `2e-8`.

### Avidity

In case the growth of real estate sell is too high, this could cause an early reward pool drainage. For this reason is has been implemented a further coefficient, called **Avidity**, which is multiplied to the RMC when the reward is calculated.

The avidity value follows this schema.

1. Avidity has an initial value of 1.0
2. Each month I track the amount of contracts created (**CPM**)
3. After another month I calculate **CPM2**
4. I calculate the difference between **CPM2** and **CPM1**
   1. If CPM2 is greater than CPM1, then I decrease the avidity by 0.1
   2. Otherwise I increment avidity by 0.1
5. Avidity becomes `avidity = max(0.1, min(0.1, new_avidity))`

### Token Price

The token price is used to proportionate the amount of reward tokens based on the NFT price. The reason behind this is to prevent tokens with for instance value of 1$ to have the same reward as tokens with value of 100$.

The base reward is set for token price of 100$. So it will be `1000%` if the token price is `1000$` or it will be
10% with a token price of 10$ etc.

### Reward Analysis

#### Fast growhth scenario

These analysis have been done with this initial scenario

1. Time period 100 years
2. Increase per year of the growth factor 0.04%
3. Average NFT sold each month: 4.000

![chart1](../../assets/images/charts/fast1.png)

![chart2](../../assets/images/charts/fast2.png)

![chart3](../../assets/images/charts/fast3.png)

![chart4](../../assets/images/charts/fast4.png)

#### Slow growth scenario

These analysis have been done with this initial scenario

1. Time period 100 years
2. Increase per year of the growth factor 0.01%
3. Average NFT sold each month: 1.000

![chart1](../../assets/images/charts/slow1.png)

![chart2](../../assets/images/charts/slow2.png)

![chart3](../../assets/images/charts/slow3.png)

![chart4](../../assets/images/charts/slow4.png)

#### Exponential cases

We have analysed cases where an exponential growth factor could exhaust the reward pool in a few years. For example a compound growth factor in sells of the 50% could exhaust the pool in about 20 years, while with a factor of 30% in about 50 years. But if the growth factor stays under the 25% the pool could keep reserving liquidity for hundreds of years.

These cases should be anyway mitigated by the introduction of the avidity coefficient.
