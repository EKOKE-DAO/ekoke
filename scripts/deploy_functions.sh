#!/bin/bash

set -e

deploy_deferred() {\
  INSTALL_MODE="$1"
  NETWORK="$2"
  DEFERRED_PRINCIPAL="$3"
  EKOKE_PRINCIPAL="$4"
  MARKETPLACE_PRINCIPAL="$5"
  ADMIN_PRINCIPAL="$6"

  echo "deploying deferred canister $DEFERRED_PRINCIPAL"

  deferred_init_args="(record {
    ekoke_canister = principal \"$EKOKE_PRINCIPAL\";
    marketplace_canister = principal \"$MARKETPLACE_PRINCIPAL\";
    custodians = vec { principal \"$ADMIN_PRINCIPAL\" };
  })"

  dfx deploy --mode=$INSTALL_MODE --yes --network="$NETWORK" --argument="$deferred_init_args" deferred

}

deploy_ekoke() {
  INSTALL_MODE="$1"
  NETWORK="$2"
  EKOKE_PRINCIPAL="$3"
  ADMINS="$4"
  TOTAL_SUPPLY="$5"
  INITIAL_BALANCES="$6"
  DEFERRED_PRINCIPAL="$7"
  MARKETPLACE_PRINCIPAL="$8"
  SWAP_ACCOUNT="$9"
  MINTING_ACCOUNT="${10}"
  ERC20_BRIDGE_ADDRESS="${11}"
  ERC20_SWAP_FEE="${12}"
  ERC20_NETWORK="${13}"

  echo "deploying ekoke canister $EKOKE_PRINCIPAL"

  ekoke_init_args="(record {
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
    cketh_minter_canister = principal \"sv3dd-oaaaa-aaaar-qacoa-cai\";
    cketh_ledger_canister = principal \"ss2fx-dyaaa-aaaar-qacoq-cai\";
    erc20_bridge_address = \"$ERC20_BRIDGE_ADDRESS\";
    erc20_gas_price = $ERC20_SWAP_FEE;
    erc20_network = variant { $ERC20_NETWORK };
  })"

  dfx deploy --mode=$INSTALL_MODE --yes --network="$NETWORK" --argument="$ekoke_init_args" ekoke

}

deploy_marketplace() {
  INSTALL_MODE="$1"
  NETWORK="$2"
  MARKETPLACE_PRINCIPAL="$3"
  DEFERRED_PRINCIPAL="$4"
  EKOKE_PRINCIPAL="$5"
  ADMINS="$6"

  echo "deploying marketplace canister $MARKETPLACE_PRINCIPAL"

  marketplace_init_args="(record {
    deferred_canister = principal \"$DEFERRED_PRINCIPAL\";
    ekoke_canister = principal \"$EKOKE_PRINCIPAL\";
    xrc_canister = principal \"uf6dk-hyaaa-aaaaq-qaaaq-cai\";
    admins = vec { $(for admin in $ADMINS; do echo "principal \"$admin\";"; done) };
    icp_ledger_canister = principal \"ryjl3-tyaaa-aaaaa-aaaba-cai\";
  })"

  dfx deploy --mode=$INSTALL_MODE --yes --network="$NETWORK" --argument="$marketplace_init_args" marketplace
}
