#!/bin/bash

cd "$(dirname "$0")" || exit 1

CANISTER_IDS="../.dfx/local/canister_ids.json"
DILAZIONATO_PRINCIPAL="$(cat "$CANISTER_IDS" | jq -r '.dilazionato.local')"
FLY_PRINCIPAL="$(cat "$CANISTER_IDS" | jq -r '.dilazionato.local')" # TODO: fix
MARKETPLACE_PRINCIPAL="$(cat "$CANISTER_IDS" | jq -r '.dilazionato.local')" # TODO: fix

ADMIN_PRINCIPAL="$(dfx identity get-principal)"

source ./deploy_functions.sh

dfx stop
dfx start --background

cd ../

deploy_dilazionato "reinstall" "local" "$DILAZIONATO_PRINCIPAL" "$FLY_PRINCIPAL" "$MARKETPLACE_PRINCIPAL" "$ADMIN_PRINCIPAL"

cd ./integration-tests/

yarn && yarn test
RES=$?

dfx stop

exit $RES

