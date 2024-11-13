#!/bin/bash

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

CANISTER_IDS=".dfx/local/canister_ids.json"
DEFERRED_PRINCIPAL="$(cat "$CANISTER_IDS" | jq -r '.deferred.local')"
EKOKE_ERC20_SWAP_PRINCIPAL=$(cat "$CANISTER_IDS" | jq -r '."ekoke-erc20-swap".local')
EKOKE_ERC20_SWAP_FRONTEND_PRINCIPAL=$(cat "$CANISTER_IDS" | jq -r '."ekoke-erc20-swap-frontend".local')
EKOKE_ICRC_INDEX_PRINCIPAL=$(cat "$CANISTER_IDS" | jq -r '."ekoke-icrc-index".local')
EKOKE_ICRC_LEDGER_PRINCIPAL=$(cat "$CANISTER_IDS" | jq -r '."ekoke-icrc-ledger".local')
EKOKE_LIQUIDITY_POOL_PRINCIPAL=$(cat "$CANISTER_IDS" | jq -r '."ekoke-liquidity-pool".local')
EKOKE_REWARD_POOL_PRINCIPAL=$(cat "$CANISTER_IDS" | jq -r '."ekoke-reward-pool".local')
MARKETPLACE_PRINCIPAL="$(cat "$CANISTER_IDS" | jq -r '.marketplace.local')"

source ./scripts/deploy_functions.sh
source ./scripts/did.sh

ADMIN_PRINCIPAL="$(dfx identity get-principal)"
ERC20_BRIDGE_ADDRESS="0xc08e14F47382BCc1dA6c3Ff366018cAb1c77091F"
ERC20_SWAP_FEE="231634000000000"
ERC20_NETWORK="Sepolia"

set -e

deploy_deferred "reinstall" "local" "$EKOKE_REWARD_POOL_PRINCIPAL" "$MARKETPLACE_PRINCIPAL" "$EKOKE_LIQUIDITY_POOL_PRINCIPAL" "$ADMIN_PRINCIPAL"
deploy_ekoke_icrc_index "reinstall" "local" "$EKOKE_ICRC_LEDGER_PRINCIPAL"
deploy_ekoke_icrc_ledger "reinstall" "local" "$EKOKE_ICRC_INDEX_PRINCIPAL" "$ADMIN_PRINCIPAL"
deploy_ekoke_erc20_swap "reinstall" "local" "$EKOKE_ERC20_SWAP_PRINCIPAL" "$ADMIN_PRINCIPAL" "$EKOKE_ICRC_LEDGER_PRINCIPAL" "$ERC20_BRIDGE_ADDRESS" "$ERC20_SWAP_FEE" "$ERC20_NETWORK"
deploy_ekoke_liquidity_pool "reinstall" "local" "$EKOKE_LIQUIDITY_POOL_PRINCIPAL" "$DEFERRED_PRINCIPAL" "$ADMIN_PRINCIPAL"
deploy_ekoke_reward_pool "reinstall" "local" "$EKOKE_REWARD_POOL_PRINCIPAL" "$ADMIN_PRINCIPAL" "$EKOKE_ICRC_LEDGER_PRINCIPAL" "$DEFERRED_PRINCIPAL" "$MARKETPLACE_PRINCIPAL"
deploy_marketplace "reinstall" "local" "$MARKETPLACE_PRINCIPAL" "$DEFERRED_PRINCIPAL" "$EKOKE_REWARD_POOL_PRINCIPAL" "$ADMIN_PRINCIPAL" "$EKOKE_LIQUIDITY_POOL_PRINCIPAL"

set +e

OUTPUT_FILE="sns_init_test.run.yaml"
cp sns_init_test.yaml $OUTPUT_FILE

# replace the principal in the file
sed -i "s/MARKETPLACE/$MARKETPLACE_PRINCIPAL/g" $OUTPUT_FILE
sed -i "s/DEFERRED/$DEFERRED_PRINCIPAL/g" $OUTPUT_FILE
sed -i "s/EKOKE_ERC20_SWAP_FRONTEND/$EKOKE_ERC20_SWAP_FRONTEND_PRINCIPAL/g" $OUTPUT_FILE
sed -i "s/EKOKE_ERC20_SWAP/$EKOKE_ERC20_SWAP_PRINCIPAL/g" $OUTPUT_FILE
sed -i "s/EKOKE_ICRC_INDEX/$EKOKE_ICRC_INDEX_PRINCIPAL/g" $OUTPUT_FILE
sed -i "s/EKOKE_ICRC_LEDGER/$EKOKE_ICRC_LEDGER_PRINCIPAL/g" $OUTPUT_FILE
sed -i "s/EKOKE_LIQUIDITY_POOL/$EKOKE_LIQUIDITY_POOL_PRINCIPAL/g" $OUTPUT_FILE
sed -i "s/EKOKE_REWARD_POOL/$EKOKE_REWARD_POOL_PRINCIPAL/g" $OUTPUT_FILE

dfx sns prepare-canisters add-nns-root $DEFERRED_PRINCIPAL
dfx sns prepare-canisters add-nns-root $EKOKE_ERC20_SWAP_PRINCIPAL
dfx sns prepare-canisters add-nns-root $EKOKE_ERC20_SWAP_FRONTEND_PRINCIPAL
dfx sns prepare-canisters add-nns-root $EKOKE_ICRC_INDEX_PRINCIPAL
dfx sns prepare-canisters add-nns-root $EKOKE_ICRC_LEDGER_PRINCIPAL
dfx sns prepare-canisters add-nns-root $EKOKE_LIQUIDITY_POOL_PRINCIPAL
dfx sns prepare-canisters add-nns-root $EKOKE_REWARD_POOL_PRINCIPAL
dfx sns prepare-canisters add-nns-root $MARKETPLACE_PRINCIPAL

dfx sns propose --test-neuron-proposer --network local $OUTPUT_FILE
RC=$?

# dfx stop

exit $RC
