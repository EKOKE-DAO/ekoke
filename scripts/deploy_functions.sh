#!/bin/bash

set -e

deploy_dilazionato() {
  echo "deploying sell contract $DILAZIONATO_PRINCIPAL"

  INSTALL_MODE="$1"
  NETWORK="$2"
  DILAZIONATO_PRINCIPAL="$3"
  FLY_PRINCIPAL="$4"
  MARKETPLACE_PRINCIPAL="$5"
  ADMIN_PRINCIPAL="$6"

  dilazionato_init_args="(record {
    fly_canister = principal \"$FLY_PRINCIPAL\";
    marketplace_canister = principal \"$MARKETPLACE_PRINCIPAL\";
    custodians = vec { principal \"$ADMIN_PRINCIPAL\" };
  })"

  dfx deploy --mode=$INSTALL_MODE --yes --network="$NETWORK" --argument="$dilazionato_init_args" dilazionato

}
