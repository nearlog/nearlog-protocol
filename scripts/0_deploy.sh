#!/bin/bash
export MAIN_ACCOUNT=nearlog.testnet
export NEAR_ENV=testnet
export CONTRACT_ID=main.$MAIN_ACCOUNT
export WETH_TOKEN_ID=weth.fakes.testnet
export WNEAR_TOKEN_ID=wrap.testnet
export ONE_YOCTO=0.000000000000000000000001
export GAS=200000000000000
export DECIMAL_18=000000000000000000
export DECIMAL_24=00000000000000000000000

echo "################### CREATE ACCOUNT ###################"

near create-account $CONTRACT_ID --masterAccount $MAIN_ACCOUNT --initialBalance 10

echo "################### CREATE CONTRACT ###################"
near deploy $CONTRACT_ID --accountId $MAIN_ACCOUNT --wasmFile ../res/nearlog_protocol.wasm


######################### B2: Init Nearland contract #########################

echo "################### INIT CONTRACT ###################"
near call $CONTRACT_ID --accountId=$CONTRACT_ID new

######################### B3: Deposit storage #########################

# Deposit CONTRACT_ID 
near call $CONTRACT_ID --accountId=$MAIN_ACCOUNT storage_deposit '' --amount=0.1

# Deposit WETH_TOKEN_ID
near call $WETH_TOKEN_ID --accountId=$CONTRACT_ID storage_deposit '' --amount=0.1

# Deposit WNEAR_TOKEN_ID
near call $WNEAR_TOKEN_ID --accountId=$CONTRACT_ID storage_deposit '' --amount=0.1

