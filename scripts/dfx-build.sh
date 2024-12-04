#!/bin/bash

# build rust canisters
dfx extension install sns
dfx sns download --wasms-dir assets/wasm/
dfx build
