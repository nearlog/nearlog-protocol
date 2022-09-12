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

# screen drum diamond addict lift element silk hammer lemon shoulder luggage rapid

echo "################### CREATE ACCOUNT ###################"

near create-account $CONTRACT_ID --masterAccount $MAIN_ACCOUNT --initialBalance 10

echo "################### CREATE CONTRACT ###################"
near deploy $CONTRACT_ID --accountId $MAIN_ACCOUNT --wasmFile ../res/nearlog_protocol.wasm


######################### B2: Init Nearland contract #########################

echo "################### INIT CONTRACT ###################"
near call $CONTRACT_ID --accountId=$CONTRACT_ID new '{
  "config" : {
    "owner_id": "'$MAIN_ACCOUNT'"
  }
}'


######################### B3: Deposit storage #########################

# Deposit CONTRACT_ID 
near call $CONTRACT_ID --accountId=$MAIN_ACCOUNT storage_deposit '' --amount=0.1

# Deposit WETH_TOKEN_ID
near call $WETH_TOKEN_ID --accountId=$CONTRACT_ID storage_deposit '' --amount=0.1
near call $WETH_TOKEN_ID --accountId=$MAIN_ACCOUNT storage_deposit '' --amount=0.1

# Deposit WNEAR_TOKEN_ID
near call $WNEAR_TOKEN_ID --accountId=$CONTRACT_ID storage_deposit '' --amount=0.1
near call $WNEAR_TOKEN_ID --accountId=$MAIN_ACCOUNT storage_deposit '' --amount=0.1


# Mint WETH_TOKEN_ID
near call $WETH_TOKEN_ID --accountId=$MAIN_ACCOUNT mint '{
 "account_id": "'$MAIN_ACCOUNT'",
 "amount": "100'$DECIMAL_18'"
}'

###################### B4: Add asset #####################

near call $CONTRACT_ID --accountId=$MAIN_ACCOUNT add_asset '{
  "token_id": "'$WETH_TOKEN_ID'",
  "asset_config": {
    "reserve_ratio": 2500,
    "max_utilization_rate": 8000,
    "collateralization_ratio": 5000,
    "extra_decimals": 0,
    "can_deposit": true,
    "can_withdraw": true
  }
}' --amount=$ONE_YOCTO --gas=$GAS