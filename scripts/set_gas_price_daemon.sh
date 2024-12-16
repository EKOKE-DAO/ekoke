#!/bin/bash

if [ -z "$ETHERSCAN_APIKEY" ]; then
    echo "ETHERSCAN_APIKEY is required"
    exit 255
fi

OP="$1"

INTERVAL=${INTERVAL:-600} # 10 minutes

if [ "$OP" == "stop" ]; then
    if [ -z "$PIDFILE" ]; then
        echo "PIDFILE is required"
        exit 255
    fi

    if [ ! -f "$PIDFILE" ]; then
        echo "PIDFILE does not exist"
        exit 255
    fi

    PID=$(cat "$PIDFILE")
    kill "$PID"
    exit 0
fi

if [ "$OP" == "start" ]; then
    echo "Starting daemon"

    # run with screen
    screen -S "deferred-gas-station" -d -m $0
    exit 0
fi

# run in background

if [ ! -z "$PIDFILE" ]; then
    echo "$$" > "$PIDFILE"
fi

while [ 1 ]; do
    safe_gas_price_gwei=$(curl "https://api.etherscan.io/v2/api?chainid=1&module=gastracker&action=gasoracle&apikey=$ETHERSCAN_APIKEY" | jq -r '.result.ProposeGasPrice')
    gas_price_wei=$(echo "$safe_gas_price_gwei 1000000000" | awk '{printf "%.0f", $1 * $2}')

    echo "Safe gas price: $safe_gas_price_gwei Gwei ($gas_price_wei Wei)"
    echo "Setting gas price to $gas_price_wei Wei"

    dfx canister call --ic \
        2f5ik-ciaaa-aaaal-aruna-cai \
        gas_station_set_gas_price \
        "( \
            $gas_price_wei \
        )"
    echo "Gas price set"

    echo "Waiting $INTERVAL seconds"
    sleep $INTERVAL
done
