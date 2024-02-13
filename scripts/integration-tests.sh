#!/bin/bash

cargo test --test integration_tests

killall pocket-ic || true
rm -rf /tmp/.tmp* || true

exit 0
