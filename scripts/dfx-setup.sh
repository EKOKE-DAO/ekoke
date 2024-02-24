#!/bin/bash

dfx stop
dfx start --background --clean
dfx canister create deferred
dfx canister create ekoke-erc20-swap
dfx canister create ekoke-erc20-swap-frontend
dfx canister create ekoke-ledger
dfx canister create ekoke-liquidity-pool
dfx canister create ekoke-reward-pool
dfx canister create marketplace

dfx stop
