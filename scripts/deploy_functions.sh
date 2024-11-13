#!/bin/bash

set -e

deploy_deferred() {\
  INSTALL_MODE="$1"
  NETWORK="$2"
  EKOKE_REWARD_POOL_PRINCIPAL="$3"
  MARKETPLACE_PRINCIPAL="$4"
  EKOKE_LIQUIDITY_POOL_PRINCIPAL="$5"
  ADMIN_PRINCIPAL="$6"

  echo "deploying deferred canister"

  deferred_init_args="(record {
    allowed_currencies = vec { \"USD\"; \"EUR\"; \"GBP\"; \"CHF\"; \"JPY\"; \"CNY\"; };
    ekoke_reward_pool_canister = principal \"$EKOKE_REWARD_POOL_PRINCIPAL\";
    icp_ledger_canister = principal \"ryjl3-tyaaa-aaaaa-aaaba-cai\";
    liquidity_pool_canister = principal \"$EKOKE_LIQUIDITY_POOL_PRINCIPAL\";
    marketplace_canister = principal \"$MARKETPLACE_PRINCIPAL\";
    custodians = vec { principal \"$ADMIN_PRINCIPAL\" };
  })"

  dfx deploy --mode=$INSTALL_MODE --yes --network="$NETWORK" --argument="$deferred_init_args" deferred

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

deploy_ekoke_erc20_swap_frontend() {
  INSTALL_MODE="$1"
  NETWORK="$2"

  echo "deploying ekoke-erc20-swap-frontend canister"

  dfx deploy --mode=$INSTALL_MODE --yes --network="$NETWORK" ekoke-erc20-swap-frontend
}

deploy_ekoke_reward_pool() {
  INSTALL_MODE="$1"
  NETWORK="$2"
  EKOKE_REWARD_POOL_PRINCIPAL="$3"
  ADMINS="$4"
  EKOKE_LEDGER_PRINCIPAL="$5"
  DEFERRED_PRINCIPAL="$6"
  MARKETPLACE_PRINCIPAL="$7"

  echo "deploying ekoke-reward-pool canister $EKOKE_REWARD_POOL_PRINCIPAL"

  ekoke_init_args="(record {
    deferred_canister = principal \"$DEFERRED_PRINCIPAL\";
    marketplace_canister = principal \"$MARKETPLACE_PRINCIPAL\";
    admins = vec { $(for admin in $ADMINS; do echo "principal \"$admin\";"; done) };
    ledger_canister = principal \"$EKOKE_LEDGER_PRINCIPAL\";
  })"

  dfx deploy --mode=$INSTALL_MODE --yes --network="$NETWORK" --argument="$ekoke_init_args" ekoke-reward-pool

}

deploy_ekoke_icrc_index() {
  INSTALL_MODE="$1"
  NETWORK="$2"
  LEDGER_PRINCIPAL="$3"

  echo "deploying ekoke-icrc-index canister"

  ekoke_icrc_index_init_args="(record {
    ledger_id = principal \"$LEDGER_PRINCIPAL\";
  })"

  dfx deploy --mode=$INSTALL_MODE --yes --network="$NETWORK" --argument="$ekoke_icrc_index_init_args" ekoke-icrc-index

}

deploy_ekoke_icrc_ledger() {
  INSTALL_MODE="$1"
  NETWORK="$2"
  INDEX_PRINCIPAL="$3"
  OWNER="$4"

  echo "deploying ekoke-icrc-ledger canister"

  ekoke_icrc_leger_init_args="(variant {
    Init = record {
      minting_account = record { owner = principal \"$OWNER\"; };
      transfer_fee = 1000;
      decimals = opt 8;
      token_name = \"Ekoke\";
      token_symbol = \"EKOKE\";
      metadata = vec {};
      initial_balances = vec {};
      feature_flags = opt record {
        icrc2 = true;
      };
      archive_options = record {
        trigger_threshold = 10_000_000;
          num_blocks_to_archive = 1_000_000;
          controller_id = principal \"$INDEX_PRINCIPAL\";
      };
    }
  })"

  dfx deploy --mode=$INSTALL_MODE --yes --network="$NETWORK" --argument="$ekoke_icrc_leger_init_args" ekoke-icrc-ledger
}

deploy_ekoke_liquidity_pool() {
  INSTALL_MODE="$1"
  NETWORK="$2"
  EKOKE_LIQUIDITY_POOL_PRINCIPAL="$3"
  DEFERRED_PRINCIPAL="$4"
  ADMINS="$5"

  echo "deploying ekoke-liquidity-pool canister $EKOKE_LIQUIDITY_POOL_PRINCIPAL"

  ekoke_liquidity_pool_init_args="(record {
    admins = vec { $(for admin in $ADMINS; do echo "principal \"$admin\";"; done) };
    deferred_canister = principal \"$DEFERRED_PRINCIPAL\";
    icp_ledger_canister = principal \"ryjl3-tyaaa-aaaaa-aaaba-cai\";
  })"

  dfx deploy --mode=$INSTALL_MODE --yes --network="$NETWORK" --argument="$ekoke_liquidity_pool_init_args" ekoke-liquidity-pool

}

deploy_marketplace() {
  INSTALL_MODE="$1"
  NETWORK="$2"
  MARKETPLACE_PRINCIPAL="$3"
  DEFERRED_PRINCIPAL="$4"
  EKOKE_REWARD_POOL_PRINCIPAL="$5"
  ADMINS="$6"
  EKOKE_LIQUIDITY_POOL_PRINCIPAL="$7"

  echo "deploying marketplace canister $MARKETPLACE_PRINCIPAL"

  marketplace_init_args="(record {
    deferred_canister = principal \"$DEFERRED_PRINCIPAL\";
    ekoke_reward_pool_canister = principal \"$EKOKE_REWARD_POOL_PRINCIPAL\";
    ekoke_liquidity_pool_canister = principal \"$EKOKE_LIQUIDITY_POOL_PRINCIPAL\";
    xrc_canister = principal \"uf6dk-hyaaa-aaaaq-qaaaq-cai\";
    admins = vec { $(for admin in $ADMINS; do echo "principal \"$admin\";"; done) };
    icp_ledger_canister = principal \"ryjl3-tyaaa-aaaaa-aaaba-cai\";
  })"

  dfx deploy --mode=$INSTALL_MODE --yes --network="$NETWORK" --argument="$marketplace_init_args" marketplace
}

get_arg() {
  read -p "$1: " arg
  if [ -z "$arg" ]; then
    echo "$2"
  else
    echo "$arg"
  fi
}
