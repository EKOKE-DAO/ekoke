#!/bin/bash

./scripts/dfx_tests.sh
RC=$?

if [ $RC -ne 0 ]; then
    echo "dfx_tests.sh failed ($RC)"
    exit $RC
fi

./scripts/pocket_ic_tests.sh
