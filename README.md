# merkle-distributor

A program for distributing tokens efficiently via uploading a [Merkle root](https://en.wikipedia.org/wiki/Merkle_tree).

## Claiming Airdrop via CLI

To claim via CLI instead of using `https://jito.network/airdrop`, run the following commands.

1. Build the cli (must have rust + cargo installed):

```bash
cargo b -r
```

1. Run `claim` with the proper args. Be sure to replace `<YOUR KEYPAIR>` with the _full path_ of your keypair file. This will transfer tokens from the program account to a the associated token account owned by your keypair, creating it if it doesn't exist.

```bash
./target/release/cli --rpc-url https://api.mainnet-beta.solana.com --keypair-path <YOUR KEYPAIR> --airdrop-version 0 --mint <YOUR MINT> --program-id ADis3cccJHS6dmj8MPCG1NGGuXBN2ZSaaXFwKKU7UUgF claim --merkle-tree-path merkle_tree.json
```
