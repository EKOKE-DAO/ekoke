#!/bin/bash

cd "$(dirname "$0")" || exit 1

CANISTER_IDS="../.dfx/local/canister_ids.json"
SELL_CONTRACT_PRINCIPAL="$(cat "$CANISTER_IDS" | jq -r '.sell_contract.local')"
FLY_PRINCIPAL="$(cat "$CANISTER_IDS" | jq -r '.sell_contract.local')" # TODO: fix
MARKETPLACE_PRINCIPAL="$(cat "$CANISTER_IDS" | jq -r '.sell_contract.local')" # TODO: fix

ADMIN_PRINCIPAL="$(dfx identity get-principal)"

source ./deploy_functions.sh

dfx stop
dfx start --background

cd ../

deploy_sell_contract "reinstall" "local" "$SELL_CONTRACT_PRINCIPAL" "$FLY_PRINCIPAL" "$MARKETPLACE_PRINCIPAL" "$ADMIN_PRINCIPAL"

cd ./integration-tests/

yarn && yarn test
RES=$?

dfx stop

exit $RES

