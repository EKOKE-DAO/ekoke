#!/bin/bash

dfx stop
dfx start --background --clean
dfx canister create deferred
dfx canister create ekoke-archive
dfx canister create ekoke-erc20-swap
dfx canister create ekoke-index
dfx canister create ekoke-ledger
dfx canister create ekoke-liquidity-pool
dfx canister create marketplace

dfx stop
