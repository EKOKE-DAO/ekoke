#!/bin/bash

PRINCIPAL="$1"

if [ -z "$PRINCIPAL" ]; then
  echo "Principal is required"
  exit 255
fi

dfx canister call --ic deferred_minter admin_set_role "(principal \"$PRINCIPAL\", variant { GasStation } )"
