#!/bin/bash

dfx stop
dfx start --background
dfx canister create deferred
dfx canister create ekoke-index
dfx canister create ekoke-ledger
dfx canister create marketplace

dfx stop
