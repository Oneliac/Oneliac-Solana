use anchor_lang::prelude::*;

declare_id!("HEALth11111111111111111111111111111111111");

#[program]
pub mod zk_healthcare {
    use super::*;
}

#[account]
pub struct HealthcareRegistry {
    pub authority: Pubkey,
    pub nist_compliant: bool,
    pub total_verifications: u64,
    pub ipfs_pin_count: u64,
}
