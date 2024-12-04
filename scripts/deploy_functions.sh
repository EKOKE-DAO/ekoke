#!/bin/bash

set -e

deploy_deferred_data() {
  INSTALL_MODE="$1"
  NETWORK="$2"
  MINTER_ID="$3"

  echo "deploying deferred data canister"

  init_args="(record {
    log_settings = record {
      enable_console = false;
      in_memory_records = 128;
      max_record_length = 1024;
      log_filter = \"info\";
    };
    minter = principal \"$MINTER_ID\";
  })"

  dfx deploy --mode=$INSTALL_MODE --yes --network="$NETWORK" --argument="$init_args" deferred_data

}

deploy_deferred_minter() {
  INSTALL_MODE="$1"
  NETWORK="$2"
  CHAIN_ID="$3"
  DEFERRED_ERC721="$4"
  ECDSA_KEY="$5"
  DEFERRED_DATA_PRINCIPAL="$6"
  ADMIN_PRINCIPAL="$7"
  EVM_RPC_PRINCIPAL="$8"
  REWARD_POOL="$9"

  echo "deploying deferred minter canister"

  init_args="(record {
    allowed_currencies = vec { \"USD\"; };
    chain_id = $CHAIN_ID;
    deferred_erc721 = \"$DEFERRED_ERC721\";
    deferred_data = principal \"$DEFERRED_DATA_PRINCIPAL\";
    ecdsa_key = variant { $ECDSA_KEY };
    evm_rpc = principal \"$EVM_RPC_PRINCIPAL\";
    custodians = vec { principal \"$ADMIN_PRINCIPAL\" };
    reward_pool = \"$REWARD_POOL\";
    log_settings = record {
      enable_console = false;
      in_memory_records = 128;
      max_record_length = 1024;
      log_filter = \"info\";
    };
  })"

  dfx deploy --mode=$INSTALL_MODE --yes --network="$NETWORK" --argument="$init_args" deferred_minter

}

get_arg() {
  read -p "$1: " arg
  if [ -z "$arg" ]; then
    echo "$2"
  else
    echo "$arg"
  fi
}
