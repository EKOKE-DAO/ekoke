#!/bin/bash

# build frontend
cd ./src/ekoke_erc20_swap_frontend/
yarn
mkdir -p node_modules/web3/dist/
wget -O node_modules/web3/dist/web3.min.js "https://cdn.jsdelivr.net/npm/web3@latest/dist/web3.min.js"
yarn build
cd -

# build rust canisters
dfx sns download --wasms-dir assets/wasm/
dfx build
