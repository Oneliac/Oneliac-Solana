# AGENTS.md - Oneliac Solana

## Commands
- Build: `anchor build`
- Test all: `anchor test`
- Test single: `anchor test --skip-build -- --grep "test name"`
- Deploy: `anchor deploy` (requires `solana config set --url devnet`)

## Architecture
Anchor-based Solana program for privacy-preserving healthcare verification using ZK proofs.
- **Program**: `programs/zk_healthcare/` - main smart contract
- **Accounts**: `HealthcareRegistry`, `VerificationRecord`, `IpfsPinRecord`, `FederatedLearningState`
- **Instructions**: `initialize`, `verify_eligibility`, `pin_medical_data`, `submit_model_update`
- Called by Python API agents via Solana RPC

## Code Style
- Rust 2021 edition with Anchor 0.30.0
- Use `#[account]` for account structs, `#[derive(Accounts)]` for instruction contexts
- Custom errors via `#[error_code]` enum with `#[msg(...)]` annotations
- Events via `#[event]` structs and `emit!()` macro
- Validate inputs with `require!()` macro
- Use `msg!()` for logging, `keccak::hash()` for hashing
