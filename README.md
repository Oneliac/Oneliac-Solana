# Oneliac Solana Contract

Solana smart contract for privacy-preserving healthcare verification using zero-knowledge proofs.

## Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Install Anchor
cargo install --git https://github.com/coral-xyz/anchor avm --locked
avm install latest
avm use latest
```

## Build

```bash
anchor build
```

## Test

```bash
anchor test
```

## Deploy

```bash
# Set cluster
solana config set --url devnet

# Deploy
anchor deploy
```

## Program Functions

| Function | Description |
|----------|-------------|
| `initialize` | Initialize the healthcare registry |
| `verify_eligibility` | Verify patient eligibility with ZK proof |
| `pin_medical_data` | Pin encrypted data to IPFS |
| `submit_model_update` | Submit federated learning update |

## Accounts

- `HealthcareRegistry` - Main registry state
- `VerificationRecord` - Individual verification records
- `IpfsPinRecord` - IPFS pinning records
- `FederatedLearningState` - FL training state

## Architecture

This contract is called by the Python API agents via Solana RPC.
