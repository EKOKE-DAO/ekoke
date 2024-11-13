#!/bin/bash

CMD="$1"

case $CMD in
  start)
    echo "Starting canisters..."
    ;;
  stop)
    echo "Stopping canisters..."
    ;;
  *)
    echo "Usage: $0 {start|stop}"
    exit 1
    ;;
esac

set -e

dfx identity use ekoketoken

dfx canister --ic $CMD v5vof-zqaaa-aaaal-ai5cq-cai
dfx canister --ic $CMD un25n-wyaaa-aaaal-ams5a-cai
dfx canister --ic $CMD uk33z-3aaaa-aaaal-ams5q-cai
dfx canister --ic $CMD v2uir-uiaaa-aaaal-ai5ca-cai
dfx canister --ic $CMD vtxdn-caaaa-aaaal-ai5dq-cai
dfx canister --ic $CMD vuwfz-pyaaa-aaaal-ai5da-cai
