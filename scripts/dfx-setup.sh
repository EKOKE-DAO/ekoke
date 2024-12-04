#!/bin/bash

dfx stop
dfx start --background --clean
dfx canister create deferred_data
dfx canister create deferred_minter

# sns
dfx canister create sns_governance
dfx canister create sns_index
dfx canister create sns_ledger
dfx canister create sns_root
dfx canister create sns_swap

dfx stop
