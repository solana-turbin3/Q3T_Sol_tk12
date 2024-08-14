use anchor_lang::prelude::*;

declare_id!("4LvtRMpRiK2AukE2iJersc7Wn3oNYZMKpSRFNuWcoZTU");

pub mod error;
pub mod instructions;
pub mod state;

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
