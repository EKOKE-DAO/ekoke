#!/bin/bash

OWNER="$1"

if [ -z "$OWNER" ]; then
  echo "Owner principal is required"
  exit 255
fi

dfx canister call --ic \
    deferred_minter \
    remove_agency \
    "( \
        principal \"$OWNER\" \
    )"
