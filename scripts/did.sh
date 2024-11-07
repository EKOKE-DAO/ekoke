#!/bin/bash

set -e

account() {
  OWNER="$1"
  SUBACCOUNT="$2"

  if [ -z "$SUBACCOUNT" ]; then
    echo "record { owner = principal \"$OWNER\"; }"
  else
    echo "record { owner = principal \"$OWNER\"; subaccount = opt vec $SUBACCOUNT; }"
  fi
}
