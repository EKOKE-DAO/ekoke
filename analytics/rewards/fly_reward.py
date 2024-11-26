#!/usr/bin/env python3
# -*- coding: utf-8 -*-

from io import TextIOWrapper
from math import floor
import random

INITIAL_TOTAL_SUPPLY = 7_000_000.0
NFT_VALUE = 100
AVG_REAL_ESTATE_VALUE = 400_000
INITIAL_RMC = 0.0000042
INITIAL_RMC_HALVING_INTERVAL = 4 * 12  # 4 years
REAL_ESTATE_PER_MONTH = 1.0
REWARD_PERIOD = 12 * 100  # 100 years
INITIAL_AVIDITY = 1.0
FEE = 0.00001000
MIN_REWARD = FEE * 10


def calculate_fly_reward(remaining_supply: float, rmc: float, avidity: float) -> float:
    reward = remaining_supply * rmc * avidity
    if reward < MIN_REWARD:  # less than transfer fee
        return MIN_REWARD
    else:
        return reward


def init_csv(handle: TextIOWrapper):
    handle.write(
        "real_estate_sold,reward_per_nft,contract_value,remaining_supply,rmc,avidity,real_estate_per_month,date\n"
    )


def write_reward_data(
    handle: TextIOWrapper,
    reward_per_nft: int,
    current_month: int,
    remaining_supply: int,
    rmc: float,
    avidity: float,
    real_estate_sold: int,
    real_estate_per_month: int,
):
    year = 2023 + ((current_month - 1) // 12) + 1
    month = ((current_month - 1) % 12) + 1
    if month < 10:
        month = f"0{month}"
    contract_value = reward_per_nft * (AVG_REAL_ESTATE_VALUE // NFT_VALUE)

    handle.write(
        f"{real_estate_sold},{reward_per_nft},{contract_value},{remaining_supply},{rmc},{avidity},{real_estate_per_month},{year}-{month}\n"
    )


def randomize_real_estate_per_month(
    last_real_estate_per_month: float, max_value: int
) -> int:
    floor_step = round(last_real_estate_per_month / 10) + 1
    ceil_step = round(last_real_estate_per_month / 5) + 1

    min_value = max(1, last_real_estate_per_month - floor_step)
    max_value = min(max_value, last_real_estate_per_month + ceil_step)

    return round(random.uniform(min_value, max_value))


def adjust_avidity(avidity: float, last_cpm: float, cpm: float) -> float:
    if cpm > last_cpm:
        new_avidity = avidity - 0.1
    else:
        new_avidity = avidity + 0.1

    return max(0.1, min(1.0, new_avidity))


# Let's make some simulations
current_month = 1
remaining_supply = INITIAL_TOTAL_SUPPLY
rmc = INITIAL_RMC
halving_interval = INITIAL_RMC_HALVING_INTERVAL
real_estate_sold = 0
real_estate_per_month = REAL_ESTATE_PER_MONTH
avidity = INITIAL_AVIDITY
last_cpm = 1
cpm = 0

handle = open("fly_reward.csv", "w")
init_csv(handle)

while current_month <= REWARD_PERIOD:
    remaining_months = min(REWARD_PERIOD - current_month + 1, halving_interval)
    for _ in range(0, remaining_months):
        for _ in range(0, floor(real_estate_per_month)):
            # reserve reward for real estate in month
            reward_per_nft = calculate_fly_reward(remaining_supply, rmc, avidity)
            if reward_per_nft == 0:
                print("No more reward")
                handle.close()
                exit(0)
            remaining_supply -= reward_per_nft * (AVG_REAL_ESTATE_VALUE // NFT_VALUE)
            if remaining_supply < 0:
                print("Distributing more than total supply")
                handle.close()
                exit(0)
            real_estate_sold += 1
            cpm += 1
            write_reward_data(
                handle,
                reward_per_nft,
                current_month,
                remaining_supply,
                rmc,
                avidity,
                real_estate_sold,
                floor(real_estate_per_month),
            )
        # increase real estate sold per month
        max_increasing_value = max(1, floor(current_month / 5))
        real_estate_per_month = randomize_real_estate_per_month(
            real_estate_per_month, max_increasing_value
        )  # increase month
        current_month += 1
        avidity = adjust_avidity(avidity, last_cpm, real_estate_per_month)
        last_cpm = cpm
        cpm = 0

    # make halve
    if rmc / 2 > 0.000000000001:
        rmc /= 2

handle.close()
