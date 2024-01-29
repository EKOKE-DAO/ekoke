#!/bin/bash

cd "$(dirname "$0")" || exit 1

CANISTER_IDS="../.dfx/local/canister_ids.json"
DEFERRED_PRINCIPAL="$(cat "$CANISTER_IDS" | jq -r '.deferred.local')"
EKOKE_PRINCIPAL="$(cat "$CANISTER_IDS" | jq -r '.ekoke.local')"
MARKETPLACE_PRINCIPAL="$(cat "$CANISTER_IDS" | jq -r '.marketplace.local')"

source ./deploy_functions.sh
source ./did.sh

ADMIN_PRINCIPAL="$(dfx identity get-principal)"
INITIAL_SUPPLY="8880101010000000000"
EKOKE_INITIAL_BALANCES="$(balances "$ADMIN_PRINCIPAL:250000000000000000")"
SWAP_ACCOUNT="$(account "$ADMIN_PRINCIPAL")"
EKOKE_MINTING_ACCOUNT="$(account "$ADMIN_PRINCIPAL" "{33;169;149;73;231;146;144;124;94;39;94;84;81;6;141;173;223;77;67;238;141;202;180;135;86;35;26;143;183;113;49;35}")"

dfx stop
dfx start --background

cd ../

deploy_deferred "reinstall" "local" "$DEFERRED_PRINCIPAL" "$EKOKE_PRINCIPAL" "$MARKETPLACE_PRINCIPAL" "$ADMIN_PRINCIPAL"
deploy_ekoke "reinstall" "local" "$EKOKE_PRINCIPAL" "$ADMIN_PRINCIPAL" "$INITIAL_SUPPLY" "$EKOKE_INITIAL_BALANCES" "$DEFERRED_PRINCIPAL" "$MARKETPLACE_PRINCIPAL" "$SWAP_ACCOUNT" "$EKOKE_MINTING_ACCOUNT"
deploy_marketplace "reinstall" "local" "$MARKETPLACE_PRINCIPAL" "$DEFERRED_PRINCIPAL" "$EKOKE_PRINCIPAL" "$ADMIN_PRINCIPAL"

dfx stop

exit $RES
