#!/bin/bash

IC_COMMIT_HASH="85bd56a70e55b2cea75cae6405ae11243e5fdad8"

ICRC_INDEX_DID="https://raw.githubusercontent.com/dfinity/ic/${IC_COMMIT_HASH}/rs/rosetta-api/icrc1/index/index.did"
ICRC_LEDGER_DID="https://raw.githubusercontent.com/dfinity/ic/${IC_COMMIT_HASH}/rs/rosetta-api/icrc1/ledger/ledger.did"

ICRC_INDEX_WASM="https://download.dfinity.systems/ic/${IC_COMMIT_HASH}/canisters/ic-icrc1-index.wasm.gz"
ICRC_LEDGER_WASM="https://download.dfinity.systems/ic/${IC_COMMIT_HASH}/canisters/ic-icrc1-ledger.wasm.gz"

set -e

wget -O docs/did/icrc1-index.did $ICRC_INDEX_DID
wget -O docs/did/icrc1-ledger.did $ICRC_LEDGER_DID

wget -O ./assets/wasm/ekoke-icrc-index.wasm.gz $ICRC_INDEX_WASM
wget -O ./assets/wasm/ekoke-icrc-ledger.wasm.gz $ICRC_LEDGER_WASM
