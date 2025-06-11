# CLI usage

This is a full lifecycle of the distributor executed using the CLI.

## Placeholders

- `TOKEN_MINT_ADDRESS`: The SPL token mint address
- `RPC_URL`: Solana RPC endpoint URL
- `FEE_PAYER_KEYPAIR_PATH`: Path to the fee payer's keypair JSON file
- `MINT_AUTHORITY_KEYPAIR_PATH`: Path to the mint authority's keypair JSON file
- `OWNER_ADDRESS`: Public key of the account owner
- `OWNER_KEYPAIR_PATH`: Path to the owner's keypair JSON file
- `CSV_PATH`: Path to the input CSV file for the Merkle tree
- `MERKLE_TREE_PATH`: Path to the output Merkle tree JSON file
- `DISTRIBUTOR_ADDRESS`: Distributor account address
- `DISTRIBUTOR_KEYPAIR_PATH`: Path to the distributor's keypair JSON file
- `DISTRIBUTOR_ATA_ADDRESS`: Associated token account address for the distributor
- `RECIPIENT_TOKEN_ACCOUNT_ADDRESS`: Associated token account address for the recipient
- `AMOUNT`: Amount of tokens (in base units, e.g. 1000000 for 1M tokens with 6 decimals)
- `START_VESTING_TS`: Vesting start timestamp (Unix epoch seconds)
- `END_VESTING_TS`: Vesting end timestamp (Unix epoch seconds)

## Preparation

### Create a testing SPL token

```
spl-token create-token \
  --decimals 6 \
  --url RPC_URL \
  --fee-payer FEE_PAYER_KEYPAIR_PATH \
  --mint-authority MINT_AUTHORITY_KEYPAIR_PATH
```

### Create ATA
```
spl-token create-account \
  TOKEN_MINT_ADDRESS \
  --url RPC_URL \
  --owner OWNER_ADDRESS \
  --fee-payer FEE_PAYER_KEYPAIR_PATH
```

### Mint tokens
```
spl-token mint \
  TOKEN_MINT_ADDRESS AMOUNT RECIPIENT_TOKEN_ACCOUNT_ADDRESS \
  --url RPC_URL \
  --mint-authority MINT_AUTHORITY_KEYPAIR_PATH \
  --fee-payer FEE_PAYER_KEYPAIR_PATH
```

## Create Merkle tree

```
cli --mint TOKEN_MINT_ADDRESS --rpc-url "RPC_URL" --keypair-path FEE_PAYER_KEYPAIR_PATH create-merkle-tree --csv-path CSV_PATH --merkle-tree-path MERKLE_TREE_PATH
```

## Get future distributor address

```
cli --mint TOKEN_MINT_ADDRESS --rpc-url "RPC_URL" --keypair-path FEE_PAYER_KEYPAIR_PATH distributor-pda
```

## Create distributor ATA

```
spl-token create-account \
  TOKEN_MINT_ADDRESS \
  --url RPC_URL \
  --owner DISTRIBUTOR_ADDRESS \
  --fee-payer FEE_PAYER_KEYPAIR_PATH
```

## Create distributor

```
cli --mint TOKEN_MINT_ADDRESS --rpc-url "RPC_URL" --keypair-path FEE_PAYER_KEYPAIR_PATH new-distributor --merkle-tree-path MERKLE_TREE_PATH --start-vesting-ts START_VESTING_TS --end-vesting-ts END_VESTING_TS
```

## Top up distributor

```
spl-token transfer TOKEN_MINT_ADDRESS AMOUNT DISTRIBUTOR_ATA_ADDRESS --owner OWNER_KEYPAIR_PATH -u devnet
```

## Claim unlocked

```
cli --mint TOKEN_MINT_ADDRESS --rpc-url "RPC_URL" --keypair-path OWNER_KEYPAIR_PATH claim --merkle-tree-path MERKLE_TREE_PATH --distributor DISTRIBUTOR_ADDRESS
```

## Clawback

```
cli --mint TOKEN_MINT_ADDRESS --rpc-url "RPC_URL" --keypair-path FEE_PAYER_KEYPAIR_PATH clawback --distributor DISTRIBUTOR_ADDRESS
```
