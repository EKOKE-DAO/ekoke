#!/bin/bash

cd "$(dirname "$0")" || exit 1

CANISTER_IDS="../.dfx/local/canister_ids.json"
DEFERRED_PRINCIPAL="$(cat "$CANISTER_IDS" | jq -r '.deferred.local')"
FLY_PRINCIPAL="$(cat "$CANISTER_IDS" | jq -r '.fly.local')"
MARKETPLACE_PRINCIPAL="$(cat "$CANISTER_IDS" | jq -r '.deferred.local')" # TODO: fix

source ./deploy_functions.sh
source ./did.sh

ADMIN_PRINCIPAL="$(dfx identity get-principal)"
INITIAL_SUPPLY="8880101010000000000"
FLY_INITIAL_BALANCES="$(balances "$ADMIN_PRINCIPAL:250000000000000000")"
SWAP_ACCOUNT="$(account "$ADMIN_PRINCIPAL")"

dfx stop
dfx start --background

cd ../

deploy_deferred "reinstall" "local" "$DEFERRED_PRINCIPAL" "$FLY_PRINCIPAL" "$MARKETPLACE_PRINCIPAL" "$ADMIN_PRINCIPAL"
deploy_fly "reinstall" "local" "$FLY_PRINCIPAL" "$ADMIN_PRINCIPAL" "$INITIAL_SUPPLY" "$FLY_INITIAL_BALANCES" "$DEFERRED_PRINCIPAL" "$MARKETPLACE_PRINCIPAL" "$SWAP_ACCOUNT"

dfx stop

exit $RES
