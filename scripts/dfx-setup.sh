#!/bin/bash

dfx stop
dfx start --background --clean
dfx canister create deferred
dfx canister create ekoke-erc20-swap
dfx canister create ekoke-erc20-swap-frontend
dfx canister create ekoke-icrc-index
dfx canister create ekoke-icrc-ledger
dfx canister create ekoke-liquidity-pool
dfx canister create ekoke-reward-pool
dfx canister create marketplace
dfx canister create sns_governance
dfx canister create sns_index
dfx canister create sns_ledger
dfx canister create sns_root
dfx canister create sns_swap

dfx stop
