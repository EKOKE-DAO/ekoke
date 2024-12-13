#!/bin/bash

cd "$(dirname "$0")" || exit 1

source ./deploy_functions.sh
source ./did.sh

ADMIN_PRINCIPAL="$(dfx identity get-principal)"
CHAIN_ID=1
DEFERRED_DATA="2m6dw-uaaaa-aaaal-arumq-cai"
DEFERRED_MINTER="2f5ik-ciaaa-aaaal-aruna-cai"
DEFERRED_ERC721="0x"
REWARD_POOL="0x"
EVM_RPC_PRINCIPAL="7hfb6-caaaa-aaaar-qadga-cai"
ECDSA_KEY="Production"

FALLBACK_CANISTER="$ADMIN_PRINCIPAL"

CANISTER="$1"

if [ -z "$CANISTER" ]; then
  echo "Please provide the canister name as an argument"
  echo "Available canisters:"
  echo "- deferred_data"
  echo "- deferred_minter"
  exit 1
fi

set -e
dfx identity use ekoketoken

cd ../

case "$CANISTER" in

  "deferred_data")
    DEFERRED_MINTER=$(get_arg "deferred-minter" "$FALLBACK_CANISTER")
    
    deploy_deferred_data "reinstall" "ic" "$DEFERRED_MINTER"
    ;;
  
  "deferred_minter")
    deploy_deferred_minter \
      "reinstall" \
      "ic" \
      "$CHAIN_ID" \
      "$DEFERRED_ERC721" \
      "$ECDSA_KEY" \
      "$DEFERRED_DATA" \
      "$ADMIN_PRINCIPAL" \
      "$EVM_RPC_PRINCIPAL" \
      "$REWARD_POOL"
    ;;

  *)
    echo "Invalid canister name"
    echo "Available canisters:"
    echo "- deferred_data"
    echo "- deferred_minter"
    exit 1
    ;;

esac

set +e

exit 0
