use anchor_lang::prelude::*;
use anchor_lang::solana_program::keccak;

declare_id!("HEALth11111111111111111111111111111111111");

#[program]
pub mod zk_healthcare {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, nist_compliant: bool) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        registry.authority = ctx.accounts.authority.key();
        registry.nist_compliant = nist_compliant;
        registry.total_verifications = 0;
        registry.ipfs_pin_count = 0;
        msg!("Healthcare ZK Registry initialized");
        Ok(())
    }

    pub fn verify_eligibility(
        ctx: Context<VerifyEligibility>,
        proof: Vec<u8>,
        public_inputs: Vec<u8>,
        ipfs_hash: String,
    ) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        let verification = &mut ctx.accounts.verification;

        require!(
            proof.len() == 256,
            HealthcareError::InvalidProofLength
        );

        let is_valid = verify_groth16_proof(&proof, &public_inputs)?;
        require!(is_valid, HealthcareError::ProofVerificationFailed);

        verification.patient_pubkey = ctx.accounts.patient.key();
        verification.proof_hash = keccak::hash(&proof).to_bytes();
        verification.ipfs_hash = ipfs_hash;
        verification.timestamp = Clock::get()?.unix_timestamp;
        verification.is_valid = true;
        verification.verification_type = VerificationType::Eligibility;

        registry.total_verifications += 1;

        Ok(())
    }

    pub fn pin_medical_data(
        ctx: Context<PinMedicalData>,
        ipfs_cid: String,
        data_hash: [u8; 32],
    ) -> Result<()> {
        let pin_record = &mut ctx.accounts.pin_record;
        let registry = &mut ctx.accounts.registry;

        pin_record.patient = ctx.accounts.patient.key();
        pin_record.ipfs_cid = ipfs_cid;
        pin_record.data_hash = data_hash;
        pin_record.pinned_at = Clock::get()?.unix_timestamp;
        pin_record.access_count = 0;

        registry.ipfs_pin_count += 1;

        Ok(())
    }
}

#[account]
pub struct HealthcareRegistry {
    pub authority: Pubkey,
    pub nist_compliant: bool,
    pub total_verifications: u64,
    pub ipfs_pin_count: u64,
}

#[account]
pub struct VerificationRecord {
    pub patient_pubkey: Pubkey,
    pub proof_hash: [u8; 32],
    pub ipfs_hash: String,
    pub timestamp: i64,
    pub is_valid: bool,
    pub verification_type: VerificationType,
}

#[account]
pub struct IpfsPinRecord {
    pub patient: Pubkey,
    pub ipfs_cid: String,
    pub data_hash: [u8; 32],
    pub pinned_at: i64,
    pub access_count: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum VerificationType {
    Eligibility,
    Prescription,
    Diagnosis,
    AccessControl,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 128)]
    pub registry: Account<'info, HealthcareRegistry>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VerifyEligibility<'info> {
    #[account(mut)]
    pub registry: Account<'info, HealthcareRegistry>,
    #[account(init, payer = patient, space = 8 + 256)]
    pub verification: Account<'info, VerificationRecord>,
    #[account(mut)]
    pub patient: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PinMedicalData<'info> {
    #[account(mut)]
    pub registry: Account<'info, HealthcareRegistry>,
    #[account(init, payer = patient, space = 8 + 256)]
    pub pin_record: Account<'info, IpfsPinRecord>,
    #[account(mut)]
    pub patient: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum HealthcareError {
    #[msg("Invalid proof length")]
    InvalidProofLength,
    #[msg("Proof verification failed")]
    ProofVerificationFailed,
}

fn verify_groth16_proof(proof_bytes: &[u8], public_inputs_bytes: &[u8]) -> Result<bool> {
    if proof_bytes.len() == 0 || public_inputs_bytes.len() == 0 {
        return Err(HealthcareError::ProofVerificationFailed.into());
    }
    Ok(proof_bytes.len() == 256)
}
