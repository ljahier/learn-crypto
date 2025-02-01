# Bitcoin Wallet CLI

A minimalist command-line Bitcoin (for now) **wallet**, written in Rust.

## Installation

### Clone and Build

```sh
git clone https://github.com/ljahier/learn-crypto.git
cd learn-crypto
cargo build --release
```

The executable is located at `./target/release/wallet`.

## Commands

Generate a seed phrase and save it (default: `wallet.seed`)

```sh
./wallet generate-seed
```

Generate a private key from the seed

```sh
./wallet generate-private
```

_(Or specify a custom seed file with `--from file_path`.)_

Generate a public key from the private key

```sh
./wallet generate-public
```

Generate a Bitcoin address from the public key

```sh
./wallet generate-address
```

Use a custom file  
Example with a specific seed file:

```sh
./wallet generate-private --from my_seed.txt
```

## What I Learned

### How a Bitcoin Wallet Works

Thanks to the official [Bitcoin Developer Guide](https://developer.bitcoin.org/devguide/wallets.html), that help me to understood how a wallet works. How to stores a private key, which generates a public key, which then creates a Bitcoin address.

How Transactions are signed with the private key to prove ownership of the funds.

## Next Steps

- Support for other blockchains like Ethereum or Solana.
- Adding transaction signing and sending functionality.
