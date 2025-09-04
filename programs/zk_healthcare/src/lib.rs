use anchor_lang::prelude::*;

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
}

#[account]
pub struct HealthcareRegistry {
    pub authority: Pubkey,
    pub nist_compliant: bool,
    pub total_verifications: u64,
    pub ipfs_pin_count: u64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 128)]
    pub registry: Account<'info, HealthcareRegistry>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
