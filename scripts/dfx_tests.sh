#!/bin/bash

dfx stop

cargo make dfx-setup
cargo make dfx-build

rm -rf /tmp/dfx-local
cp -r .dfx/local /tmp/dfx-local

dfx identity new --storage-mode=plaintext --force admin
dfx identity use admin

dfx start --background --clean --artificial-delay 0

mkdir -p .dfx/local

wallet_principal=$(dfx identity get-wallet) && dfx ledger fabricate-cycles --t 1000000 --canister $wallet_principal

cp -r /tmp/dfx-local/* .dfx/local

sleep 10

cargo test --no-default-features --features dfx --test integration_tests $@
RC=$?

dfx stop

exit $?
