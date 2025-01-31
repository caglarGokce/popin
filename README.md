# Popin program

## Overview
This is a Solana Program that manages writing and deleting data on-chain to solana accounts.

## Prerequisites
Before running or deploying the program, ensure you have the following installed:

- Rust & Cargo: [Install Rust](https://www.rust-lang.org/tools/install)
- Solana CLI: [Install Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- BPF SDK (Included in Solana CLI)

## Building the Program
To compile the program for Solana:
```sh
cargo build-bpf
```
This will generate the compiled `.so` file in the `target/deploy` directory.

## Deploying the Program
1. Start a Solana test validator (optional but recommended for testing):
   ```sh
   solana-test-validator
   ```
   Open a new terminal and set up the test validator:
   ```sh
   solana config set --url localhost
   solana airdrop 2
   ```

2. Deploy the program using the Solana CLI:
   ```sh
   solana program deploy target/deploy/<program_name>.so
   ```
   This will return the program's public key, which you will need for interaction.



## Troubleshooting
- Ensure Solana CLI is correctly configured: `solana config get`
- If deployment fails, check logs using `solana logs`
- Verify the correct program ID is being used

## License
This project is licensed under the MIT License.

