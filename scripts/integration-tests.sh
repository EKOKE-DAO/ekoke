#!/bin/bash

cargo test --test integration_tests $@
RC=$?

killall pocket-ic || true
rm -rf /tmp/.tmp* || true

exit $RC
