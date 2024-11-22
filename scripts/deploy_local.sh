#!/bin/bash

cd "$(dirname "$0")" || exit 1

STOP="${1:-1}"

CANISTER_IDS="../.dfx/local/canister_ids.json"
DEFERRED_DATA_PRINCIPAL="$(cat "$CANISTER_IDS" | jq -r '.deferred_data.local')"
DEFERRED_MINTER_PRINCIPAL="$(cat "$CANISTER_IDS" | jq -r '.deferred_minter.local')"

source ./deploy_functions.sh
source ./did.sh

ADMIN_PRINCIPAL="$(dfx identity get-principal)"
CHAIN_ID="11155111"
DEFERRED_ERC721="0xc08e14F47382BCc1dA6c3Ff366018cAb1c77091F"
ECDSA_KEY="Dfx"
EVM_RPC_PRINCIPAL="7hfb6-caaaa-aaaar-qadga-cai"
REWARD_POOL="0xc08e14F47382BCc1dA6c3Ff366018cAb1c77091F"

dfx stop
dfx start --background

cd ../

set -e


deploy_deferred_data "reinstall" "local" "$DEFERRED_MINTER_PRINCIPAL"
deploy_deferred_minter \
    "reinstall" \
    "local" \
    $CHAIN_ID \
    $DEFERRED_ERC721 \
    $ECDSA_KEY \
    $DEFERRED_DATA_PRINCIPAL \
    $ADMIN_PRINCIPAL \
    $EVM_RPC_PRINCIPAL \
    $REWARD_POOL


set +e

if [  "$STOP" -eq 0 ]; then
    exit 0
fi

dfx stop

exit 0
