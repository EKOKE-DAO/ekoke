#!/bin/bash


function usage() {
  echo "Usage: $0 [options]"
  echo "Options:"
  echo "  -h, --help                   Display this help message"
  echo "  -n, --network <network>      Network"
  echo "  --principal <principal>      Account owner"
}

function canister_principal() {
  NETWORK="$1"

  case "$NETWORK" in
    ic)
      cat canister_ids.json | jq -r '.ekoke-icrc-ledger'
      ;;
    
    *)
      cat .dfx/local/canister_ids.json | jq -r '."ekoke-icrc-ledger".local'
      ;;
  esac
}


ARGS=$(getopt -o n:h --long network,principal,help -- "$@")
while true; do
  case "$1" in

  -n | --network)
    NETWORK="$2"
    shift 2
    ;;

  --principal)
    OWNER="$2"
    shift 2
    ;;

  -h | --help)
    usage
    exit 255
    ;;

  --)
    shift
    break
    ;;

  *)
    break
    ;;
  esac
done

CANISTER_ID=$(canister_principal "$NETWORK")

if [ -z "$CANISTER_ID" ]; then
  echo "Canister ID not found"
  usage
  exit 1
fi

if [ -z "$OWNER" ]; then
  echo "Owner not found"
  usage
  exit 1
fi

dfx canister call "$CANISTER_ID" icrc1_balance_of "(record { owner = principal \"$OWNER\"; })"
