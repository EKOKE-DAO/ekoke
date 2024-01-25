#!/bin/bash

set -e

deploy_deferred() {\
  INSTALL_MODE="$1"
  NETWORK="$2"
  DEFERRED_PRINCIPAL="$3"
  FLY_PRINCIPAL="$4"
  MARKETPLACE_PRINCIPAL="$5"
  ADMIN_PRINCIPAL="$6"

  echo "deploying deferred canister $DEFERRED_PRINCIPAL"

  deferred_init_args="(record {
    fly_canister = principal \"$FLY_PRINCIPAL\";
    marketplace_canister = principal \"$MARKETPLACE_PRINCIPAL\";
    custodians = vec { principal \"$ADMIN_PRINCIPAL\" };
  })"

  dfx deploy --mode=$INSTALL_MODE --yes --network="$NETWORK" --argument="$deferred_init_args" deferred

}

deploy_fly() {
  INSTALL_MODE="$1"
  NETWORK="$2"
  FLY_PRINCIPAL="$3"
  ADMINS="$4"
  TOTAL_SUPPLY="$5"
  INITIAL_BALANCES="$6"
  DEFERRED_PRINCIPAL="$7"
  MARKETPLACE_PRINCIPAL="$8"
  SWAP_ACCOUNT="$9"
  MINTING_ACCOUNT="${10}"
  ERC20_BRIDGE_ADDRESS="${11}"
  ERC20_SWAP_FEE="${12}"

  echo "deploying fly canister $FLY_PRINCIPAL"

  fly_init_args="(record {
    deferred_canister = principal \"$DEFERRED_PRINCIPAL\";
    marketplace_canister = principal \"$MARKETPLACE_PRINCIPAL\";
    swap_account = $SWAP_ACCOUNT;
    admins = vec { $(for admin in $ADMINS; do echo "principal \"$admin\";"; done) };
    total_supply = $TOTAL_SUPPLY;
    initial_balances = $INITIAL_BALANCES;
    minting_account = $MINTING_ACCOUNT;
    xrc_canister = principal \"uf6dk-hyaaa-aaaaq-qaaaq-cai\";
    ckbtc_canister = principal \"mxzaz-hqaaa-aaaar-qaada-cai\";
    icp_ledger_canister = principal \"ryjl3-tyaaa-aaaaa-aaaba-cai\";
    cketh_minter_canister = principal \"qcg3w-tyaaa-aaaaa-aaaea-cai\";
    cketh_ledger_canister = principal \"qcg3w-tyaaa-aaaaa-aaaea-cai\";
    erc20_bridge_address = \"$ERC20_BRIDGE_ADDRESS\";
    erc20_swap_fee = $ERC20_SWAP_FEE;
  })"

  dfx deploy --mode=$INSTALL_MODE --yes --network="$NETWORK" --argument="$fly_init_args" fly

}

deploy_marketplace() {
  INSTALL_MODE="$1"
  NETWORK="$2"
  MARKETPLACE_PRINCIPAL="$3"
  DEFERRED_PRINCIPAL="$4"
  FLY_PRINCIPAL="$5"
  ADMINS="$6"

  echo "deploying marketplace canister $MARKETPLACE_PRINCIPAL"

  marketplace_init_args="(record {
    deferred_canister = principal \"$DEFERRED_PRINCIPAL\";
    fly_canister = principal \"$FLY_PRINCIPAL\";
    xrc_canister = principal \"uf6dk-hyaaa-aaaaq-qaaaq-cai\";
    admins = vec { $(for admin in $ADMINS; do echo "principal \"$admin\";"; done) };
    icp_ledger_canister = principal \"ryjl3-tyaaa-aaaaa-aaaba-cai\";
  })"

  dfx deploy --mode=$INSTALL_MODE --yes --network="$NETWORK" --argument="$marketplace_init_args" marketplace
}
