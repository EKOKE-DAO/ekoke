#!/bin/bash

dfx canister create deferred_data
dfx canister create deferred_minter
dfx canister create sns_governance
dfx canister create sns_index
dfx canister create sns_ledger
dfx canister create sns_root
dfx canister create sns_swap

CANISTER_IDS=".dfx/local/canister_ids.json"
DEFERRED_DATA_PRINCIPAL="$(cat "$CANISTER_IDS" | jq -r '.deferred_data.local')"
DEFERRED_MINTER_PRINCIPAL="$(cat "$CANISTER_IDS" | jq -r '.deferred_minter.local')"

source ./scripts/deploy_functions.sh
source ./scripts/did.sh

ADMIN_PRINCIPAL="$(dfx identity get-principal)"
ERC20_BRIDGE_ADDRESS="0xc08e14F47382BCc1dA6c3Ff366018cAb1c77091F"
ERC20_SWAP_FEE="231634000000000"
ERC20_NETWORK="Sepolia"

set -e

deploy_deferred_data "reinstall" "local" "$EKOKE_REWARD_POOL_PRINCIPAL" "$MARKETPLACE_PRINCIPAL" "$EKOKE_LIQUIDITY_POOL_PRINCIPAL" "$ADMIN_PRINCIPAL"
deploy_deferred_minter "reinstall" "local" "$EKOKE_REWARD_POOL_PRINCIPAL" "$MARKETPLACE_PRINCIPAL" "$EKOKE_LIQUIDITY_POOL_PRINCIPAL" "$ADMIN_PRINCIPAL"
set +e

OUTPUT_FILE="sns_init_test.run.yaml"
cp sns_init_test.yaml $OUTPUT_FILE

# replace the principal in the file
sed -i "s/DEFERRED_DATA/$DEFERRED_DATA_PRINCIPAL/g" $OUTPUT_FILE
sed -i "s/DEFERRED_MINTER/$DEFERRED_MINTER_PRINCIPAL/g" $OUTPUT_FILE

dfx sns prepare-canisters add-nns-root $DEFERRED_DATA_PRINCIPAL
dfx sns prepare-canisters add-nns-root $DEFERRED_MINTER_PRINCIPAL

dfx sns propose --test-neuron-proposer --network local $OUTPUT_FILE
RC=$?

# dfx stop

exit $RC
