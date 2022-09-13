# Nearlog Protocol Contract

Nearlog protolcol is a native Near open-source option market that allows traders to buy and sell options on cryptocurrencies against a pool of liquidity.

## Getting started

```
cd existing_repo
git remote add origin https://github.com/nearlog/nearlog-protocol.git
git branch -M main
git push -uf origin main
```

## Build

```
./build.sh
```

## Deploy

```
./scripts/0_deploy.sh
```

## Actions

### Register account by paying for storage

This has to be done one per account.

```bash
near call $CONTRACT_ID --accountId=$ACCOUNT_ID --gas=$GAS --amount=0.1 storage_deposit '{}'
```

### Deposit liquidity

For example, let's deposit `5` WETH.

```bash
near call $WETH_TOKEN_ID --accountId=$ACCOUNT_ID --gas=$GAS --amount=$ONE_YOCTO ft_transfer_call '{
  "receiver_id": "'$CONTRACT_ID'",
  "amount": "5000000000000000000",
  "msg": ""
}'
```

### View account information

```bash
near view $CONTRACT_ID get_account '{"account_id": "'$ACCOUNT_ID'"}'
```

### View a given asset

```bash
near view $CONTRACT_ID get_asset '{"token_id": "'$WETH_TOKEN_ID'"}'
```

### Create a call option

```
near call $ORACLE_ID --accountId=$MAIN_ACCOUNT oracle_call '{  "receiver_id": "'$CONTRACT_ID'",
  "asset_ids": [
    "'$WETH_TOKEN_ID'",
    "'$USDT_TOKEN_ID'",
  ],
  "msg": "{\"Execute\": {\"actions\": [{\"Create\":{\"option_type\": \"call\", \"strike\": \"20000\", \"expiration\": \"1663078814\", \"amount\": \"1\", \"token_id\":\"'$WETH_TOKEN_ID'\"}}]}}"
}' --amount=$ONE_YOCTO --gas=$GAS
```

### Create a put option

```
near call $ORACLE_ID --accountId=$MAIN_ACCOUNT oracle_call '{  "receiver_id": "'$CONTRACT_ID'",
  "asset_ids": [
    "'$WETH_TOKEN_ID'",
    "'$USDT_TOKEN_ID'",
  ],
  "msg": "{\"Execute\": {\"actions\": [{\"Create\":{\"option_type\": \"put\", \"strike\": \"20000\", \"expiration\": \"1663078814\", \"amount\": \"1\", \"token_id\":\"'$WETH_TOKEN_ID'\"}}]}}"
}' --amount=$ONE_YOCTO --gas=$GAS
```

### Exercise an option

```
near call $ORACLE_ID --accountId=$MAIN_ACCOUNT oracle_call '{  "receiver_id": "'$CONTRACT_ID'",
  "asset_ids": [
    "'$WETH_TOKEN_ID'",
    "'$USDT_TOKEN_ID'",
  ],
  "msg": "{\"Execute\": {\"actions\": [{\"Exercise\":{\"option_id\": \"1111\", \"token_id\":\"'$WETH_TOKEN_ID'\"}}]}}"
}' --amount=$ONE_YOCTO --gas=$GAS
```
