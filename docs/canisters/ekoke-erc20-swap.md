# EKOKE ERC20 Swap Canister

- [EKOKE ERC20 Swap Canister](#ekoke-erc20-swap-canister)
  - [Introduction](#introduction)
  - [ERC20 Smart Contract](#erc20-smart-contract)
    - [swap (erc20)](#swap-erc20)
    - [transcribeSwap (erc20)](#transcribeswap-erc20)
  - [API](#api)
    - [swap (icrc)](#swap-icrc)
    - [swap\_fee](#swap_fee)
  - [Swap implementation](#swap-implementation)
    - [Swap ICRC into ERC20](#swap-icrc-into-erc20)
    - [Swap ERC20 into ICRC](#swap-erc20-into-icrc)
  - [ckETH / ETH Swap](#cketh--eth-swap)

## Introduction

- Ethereum [0x]()
- Sepolia: [0x30eBEE43A1f7Ba89C78Eb4Adde3ada425DAA473d](https://sepolia.etherscan.io/address/0x30eBEE43A1f7Ba89C78Eb4Adde3ada425DAA473d)

Decimals: 8
Symbol: **EKOKE**

The EKOKE ICRC-2 token has a 1:1 token on Ethereum implemented as an ERC20 token, with the same name.
The purpose of this token on the Ethereum blockchain is to make EKOKE more accessible to web3 users.

## ERC20 Smart Contract

The smart contract which implements the EKOKE ERC20 has two additional methods to allow bridging between IC and Ethereum blockchains.

### swap (erc20)

The swap method allows an Ethereum user to withdraw to convert ERC20 tokens to ICRC tokens.
The swap method takes as argument the **Principal** to withdraw the tokens to and the amount to withdraw.
Once called this method burns the user tokens and emits an event `EkokeSwapped` which is intercepted by the ekoke-erc20-swap canister.

Once intercepted the canister will withdraw ICRC tokens to the provided principal for the specified amount.

### transcribeSwap (erc20)

This method can be called only by the ekoke-erc20-swap canister and it is used once a deposit has been made on the canister, which means that a user has requested to convert his ICRC tokens into ERC20 tokens.

This method takes as argument the ETH address to mint tokens to and the amount. Once called the ERC20 tokens will be minted to the provided ethereum address.

## API

The full API is documented in the [DID](../../src/ekoke_erc20_swap/ekoke-erc20-swap.did).

### swap (icrc)

This method can be called by a user who wants to swap a certain amount of ICRC tokens into ERC20 tokens.

The user provides an ethereum address to swap the tokens to and these will be swapped into the ERC20 token, following the process described below.

### swap_fee

Get the swap fee to pay in ckETH to allow the swap.

## Swap implementation

### Swap ICRC into ERC20

1. Alice wants to swap 100 EKOKE from **ICRC** into **ERC20**
2. Alice calls `swap_fee` on the **erc20-swap canister**
3. The **erc20-swap canister** checks the current gas price and returns the fee Alice must allow the canister to pay for her
4. Alice gives **ckEth** allowance to the **erc20-swap canister** for at least the fee value
5. Alice gives **EKOKE** allowance to the **erc20-swap canister** for the amount she wants to swap
6. Alice calls **swap** on the **erc20-swap canister** providing the Ethereum address she wants to receive the ERC20 tokens to and the amount she will to swap
7. At this point the **erc20-swap canister** verifies
   1. Alice has enough allowance for ckETH to cover the fee costs
   2. Alice has the ekoke balance she wants to swap
   3. Alice has given allowance for the balance she wants to swap
8. **erc20-swap canister** calls `transcribeSwap` on the ERC20 smart contract
9. **erc20-swap canister** transfers the amount from Alice account to its account
10. the ERC20 smart contract mints amount of ekoke tokens to the provided ethereum account
11. Alice can now see her value being transferred from IC to Ethereum.

### Swap ERC20 into ICRC

1. Alice wants to swap 100 EKOKE from **ERC20** back into **ICRC**
2. Alice calls `swap` on the ERC20 smart contract, providing her IC principal and the amount she wants to swap
3. the ERC20 sc validates her balance
4. the ERC20 sc burns the token amount she has provided from her account
5. the ERC20 sm emits the `EkokeSwapped` event passing the principal and the amount as arguments
6. **erc20-swap canister** catches the events and transfers the amount from its account to the account specified in the event

## ckETH / ETH Swap

Since the conversion from ckETH to ETH is rather complex and requires time, it cannot be done directly during the swap, but it must be scheduled as an async task.

The task will have to:

1. Check the ckETH amount on the canister balance using `icrc1_balance`. The balance will have to be 0.03 ckETH at least <https://github.com/dfinity/ic/tree/master/rs/ethereum/cketh/minter#cost-of-all-cketh-transactions>.
2. Approve the ckETH minter canister to spend the current ckETH amount on the ckETH ledger canister
3. Call `withdraw_eth` on the ckETH minter canister providing the amount of eth we want to withdraw

For more details: <https://github.com/dfinity/ic/blob/master/rs/ethereum/cketh/minter/README.adoc#withdrawal-cketh-to-eth>.
