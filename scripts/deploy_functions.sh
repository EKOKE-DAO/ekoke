#!/bin/bash

set -e

deploy_deferred() {\
  INSTALL_MODE="$1"
  NETWORK="$2"
  DEFERRED_PRINCIPAL="$3"
  EKOKE_LEDGER_PRINCIPAL="$4"
  MARKETPLACE_PRINCIPAL="$5"
  ADMIN_PRINCIPAL="$6"

  echo "deploying deferred canister $DEFERRED_PRINCIPAL"

  deferred_init_args="(record {
    ekoke_ledger_canister = principal \"$EKOKE_LEDGER_PRINCIPAL\";
    marketplace_canister = principal \"$MARKETPLACE_PRINCIPAL\";
    custodians = vec { principal \"$ADMIN_PRINCIPAL\" };
  })"

  dfx deploy --mode=$INSTALL_MODE --yes --network="$NETWORK" --argument="$deferred_init_args" deferred

}

deploy_ekoke_archive() {
  INSTALL_MODE="$1"
  NETWORK="$2"
  EKOKE_ARCHIVE_PRINCIPAL="$3"
  EKOKE_LEDGER_PRINCIPAL="$4"
  EKOKE_INDEX_PRINCIPAL="$5"

  echo "deploying ekoke-archive canister $EKOKE_ARCHIVE_PRINCIPAL"

  ekoke_archive_init_args="(record {
    index_id = principal \"$EKOKE_INDEX_PRINCIPAL\";
    ledger_id = principal \"$EKOKE_LEDGER_PRINCIPAL\";
  })"

  dfx deploy --mode=$INSTALL_MODE --yes --network="$NETWORK" --argument="$ekoke_archive_init_args" ekoke-archive
}

deploy_ekoke_erc20_swap() {
  INSTALL_MODE="$1"
  NETWORK="$2"
  EKOKE_ERC20_SWAP_PRINCIPAL="$3"
  ADMINS="$4"
  EKOKE_LEDGER_PRINCIPAL="$5"
  ERC20_BRIDGE_ADDRESS="$6"
  ERC20_SWAP_FEE="$7"
  ERC20_NETWORK="$8"

  echo "deploying ekoke-erc20-swap canister $EKOKE_ERC20_SWAP_PRINCIPAL"

  ekoke_erc20_swap_init_args="(record {
    admins = vec { $(for admin in $ADMINS; do echo "principal \"$admin\";"; done) };
    ledger_id = principal \"$EKOKE_LEDGER_PRINCIPAL\";
    cketh_minter_canister = principal \"sv3dd-oaaaa-aaaar-qacoa-cai\";
    cketh_ledger_canister = principal \"ss2fx-dyaaa-aaaar-qacoq-cai\";
    erc20_bridge_address = \"$ERC20_BRIDGE_ADDRESS\";
    erc20_gas_price = $ERC20_SWAP_FEE;
    erc20_network = variant { $ERC20_NETWORK };
  })"

  dfx deploy --mode=$INSTALL_MODE --yes --network="$NETWORK" --argument="$ekoke_erc20_swap_init_args" ekoke-erc20-swap
}

deploy_ekoke_index() {
  INSTALL_MODE="$1"
  NETWORK="$2"
  EKOKE_INDEX_PRINCIPAL="$3"
  EKOKE_LEDGER_PRINCIPAL="$4"
  EKOKE_ARCHIVE_PRINCIPAL="$5"

  echo "deploying ekoke-index canister $EKOKE_INDEX_PRINCIPAL"

  ekoke_index_init_args="(record {
    archive_id = principal \"$EKOKE_ARCHIVE_PRINCIPAL\";
    ledger_id = principal \"$EKOKE_LEDGER_PRINCIPAL\";
  })"

  dfx deploy --mode=$INSTALL_MODE --yes --network="$NETWORK" --argument="$ekoke_index_init_args" ekoke-index
}

deploy_ekoke_ledger() {
  INSTALL_MODE="$1"
  NETWORK="$2"
  EKOKE_LEDGER_PRINCIPAL="$3"
  ADMINS="$4"
  TOTAL_SUPPLY="$5"
  INITIAL_BALANCES="$6"
  DEFERRED_PRINCIPAL="$7"
  MARKETPLACE_PRINCIPAL="$8"
  SWAP_ACCOUNT="$9"
  MINTING_ACCOUNT="${10}"
  EKOKE_ARCHIVE_PRINCIPAL="${11}"

  echo "deploying ekoke-ledger canister $EKOKE_LEDGER_PRINCIPAL"

  ekoke_init_args="(record {
    deferred_canister = principal \"$DEFERRED_PRINCIPAL\";
    marketplace_canister = principal \"$MARKETPLACE_PRINCIPAL\";
    swap_account = $SWAP_ACCOUNT;
    admins = vec { $(for admin in $ADMINS; do echo "principal \"$admin\";"; done) };
    total_supply = $TOTAL_SUPPLY;
    initial_balances = $INITIAL_BALANCES;
    minting_account = $MINTING_ACCOUNT;
    archive_canister = principal \"$EKOKE_ARCHIVE_PRINCIPAL\";
    xrc_canister = principal \"uf6dk-hyaaa-aaaaq-qaaaq-cai\";
    ckbtc_canister = principal \"mxzaz-hqaaa-aaaar-qaada-cai\";
    icp_ledger_canister = principal \"ryjl3-tyaaa-aaaaa-aaaba-cai\";
  })"

  dfx deploy --mode=$INSTALL_MODE --yes --network="$NETWORK" --argument="$ekoke_init_args" ekoke-ledger

}

deploy_marketplace() {
  INSTALL_MODE="$1"
  NETWORK="$2"
  MARKETPLACE_PRINCIPAL="$3"
  DEFERRED_PRINCIPAL="$4"
  EKOKE_LEDGER_PRINCIPAL="$5"
  ADMINS="$6"

  echo "deploying marketplace canister $MARKETPLACE_PRINCIPAL"

  marketplace_init_args="(record {
    deferred_canister = principal \"$DEFERRED_PRINCIPAL\";
    ekoke_ledger_canister = principal \"$EKOKE_LEDGER_PRINCIPAL\";
    xrc_canister = principal \"uf6dk-hyaaa-aaaaq-qaaaq-cai\";
    admins = vec { $(for admin in $ADMINS; do echo "principal \"$admin\";"; done) };
    icp_ledger_canister = principal \"ryjl3-tyaaa-aaaaa-aaaba-cai\";
  })"

  dfx deploy --mode=$INSTALL_MODE --yes --network="$NETWORK" --argument="$marketplace_init_args" marketplace
}
