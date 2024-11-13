#!/bin/bash


function usage() {
  echo "Usage: $0 [options]"
  echo "Options:"
  echo "  -h, --help                   Display this help message"
  echo "  -n, --network <network>      Network"
  echo "  --to <principal>             Recipient principal"
  echo "  -a, --amount <amount>        Token amount"
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


ARGS=$(getopt -o n:a:h --long network,to,amount,help -- "$@")
while true; do
  case "$1" in

  -n | --network)
    NETWORK="$2"
    shift 2
    ;;

  --to)
    RECIPIENT="$2"
    shift 2
    ;;

  -a | --amount)
    AMOUNT="$2"
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

if [ -z "$RECIPIENT" ]; then
  echo "Recipient principal not provided"
  usage
  exit 1
fi

if [ -z "$AMOUNT" ]; then
  echo "Token amount not provided"
  usage
  exit 1
fi

dfx canister call "$CANISTER_ID" icrc1_transfer "(record { to = record { owner = principal \"$RECIPIENT\" }; amount = $AMOUNT; })"
