#!/usr/bin/env python3
# -*- coding: utf-8 -*-

from io import TextIOWrapper
from math import floor

INITIAL_TOTAL_SUPPLY = 8_700_000.0
NFT_VALUE = 100
AVG_REAL_ESTATE_VALUE = 400_000
INITIAL_RMC = 0.0000042
INITIAL_RMC_HALVING_INTERVAL = 4 * 12  # 8 years
REAL_ESTATE_PER_MONTH = 1.0
REAL_ESTATE_PER_MONTH_GROWTH_FACTOR = 1.30
REAL_ESTATE_PER_MONTH_GROWTH_FACTOR_DECREASE = 0.996
REWARD_PERIOD = 12 * 100  # 100 years


def calculate_fly_reward(remaining_supply: float, rmc: float) -> float:
    reward = remaining_supply * rmc
    if reward < 0.000000000001:  # less than picofly
        return 0.000000000001
    else:
        return remaining_supply * rmc


def init_csv(handle: TextIOWrapper):
    handle.write(
        "real_estate_sold,reward_per_nft,contract_value,remaining_supply,rmc,real_estate_per_month,date\n"
    )


def write_reward_data(
    handle: TextIOWrapper,
    reward_per_nft: int,
    current_month: int,
    remaining_supply: int,
    rmc: float,
    real_estate_sold: int,
    real_estate_per_month: int,
):
    year = 2023 + ((current_month - 1) // 12) + 1
    month = ((current_month - 1) % 12) + 1
    if month < 10:
        month = f"0{month}"
    contract_value = reward_per_nft * (AVG_REAL_ESTATE_VALUE // NFT_VALUE)

    handle.write(
        f"{real_estate_sold},{reward_per_nft},{contract_value},{remaining_supply},{rmc},{real_estate_per_month},{year}-{month}\n"
    )


# Let's make some simulations
current_month = 1
remaining_supply = INITIAL_TOTAL_SUPPLY
rmc = INITIAL_RMC
halving_interval = INITIAL_RMC_HALVING_INTERVAL
real_estate_sold = 0
real_estate_per_month = REAL_ESTATE_PER_MONTH
real_estate_per_month_growth_factor = REAL_ESTATE_PER_MONTH_GROWTH_FACTOR

handle = open("fly_reward.csv", "w")
init_csv(handle)

while current_month <= REWARD_PERIOD:
    remaining_months = min(REWARD_PERIOD - current_month + 1, halving_interval)
    for _ in range(0, remaining_months):
        for _ in range(0, floor(real_estate_per_month)):
            # reserve reward for real estate in month
            reward_per_nft = calculate_fly_reward(remaining_supply, rmc)
            if reward_per_nft == 0:
                exit(0)
            remaining_supply -= reward_per_nft * (AVG_REAL_ESTATE_VALUE // NFT_VALUE)
            if remaining_supply < 0:
                exit(0)
            real_estate_sold += 1
            write_reward_data(
                handle,
                reward_per_nft,
                current_month,
                remaining_supply,
                rmc,
                real_estate_sold,
                floor(real_estate_per_month),
            )
        # increase month
        current_month += 1
        # increase real estate sold per month
        if current_month % 12 == 1:
            real_estate_per_month *= real_estate_per_month_growth_factor
            # decrease the growth factor by 0.1% every year
            if (
                real_estate_per_month_growth_factor
                * REAL_ESTATE_PER_MONTH_GROWTH_FACTOR_DECREASE
                > 1
            ):
                real_estate_per_month_growth_factor *= (
                    REAL_ESTATE_PER_MONTH_GROWTH_FACTOR_DECREASE
                )

    # make halve
    if rmc / 2 > 0.000000000001:
        rmc /= 2

handle.close()
