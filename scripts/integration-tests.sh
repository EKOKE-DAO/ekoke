#!/bin/bash

POCKET_IC_VERSION=5.0.0

SHOULD_REINSTALL_POCKET_IC=0
if [ -f "./integration-tests/pocket-ic" ]; then
  VERSION=$(./integration-tests/pocket-ic --version)
  if [ "$VERSION" != "pocket-ic-server $POCKET_IC_VERSION" ]; then
    SHOULD_REINSTALL_POCKET_IC=1
  fi
else
  SHOULD_REINSTALL_POCKET_IC=1
fi

if [ $SHOULD_REINSTALL_POCKET_IC -eq 1 ]; then
  echo "Reinstalling pocket-ic"
  wget -O pocket-ic.gz https://github.com/dfinity/pocketic/releases/download/$POCKET_IC_VERSION/pocket-ic-x86_64-linux.gz
  gzip -d pocket-ic.gz
  chmod +x pocket-ic
  mv pocket-ic ./integration-tests/pocket-ic
fi

cargo test --test integration_tests $@
RC=$?

killall pocket-ic || true
rm -rf /tmp/.tmp* || true

exit $RC
