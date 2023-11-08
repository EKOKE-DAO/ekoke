#!/bin/bash

set -e

deploy_sell_contract() {
  echo "deploying sell contract $SELL_CONTRACT_PRINCIPAL"

  INSTALL_MODE="$1"
  NETWORK="$2"
  SELL_CONTRACT_PRINCIPAL="$3"
  FLY_PRINCIPAL="$4"
  MARKETPLACE_PRINCIPAL="$5"
  ADMIN_PRINCIPAL="$6"

  sell_contract_init_args="(record {
    fly_canister = principal \"$FLY_PRINCIPAL\";
    marketplace_canister = principal \"$MARKETPLACE_PRINCIPAL\";
    custodians = vec { principal \"$ADMIN_PRINCIPAL\" };
  })"

  dfx canister install --mode=$INSTALL_MODE --yes --network="$NETWORK" --argument="$sell_contract_init_args" sell_contract

}
