use anchor_lang::prelude::*;

declare_id!("BXfuFD8G6YJ9vcHU5ubm4QgDG5Uc4taJYrUjXBavFH7H");

mod error;
mod instructions;
mod state;

pub use instructions::*;

#[program]
pub mod nft_staking {
    use super::*;

    pub fn init_config(
        ctx: Context<InitializeConfig>,
        points_per_stake: u8,
        max_stake: u8,
        freeze_period: u32,
    ) -> Result<()> {
        ctx.accounts
            .init_config(points_per_stake, max_stake, freeze_period, &ctx.bumps)
    }

    pub fn init_user_account(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.init_user_account(&ctx.bumps)
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        ctx.accounts.unstake()
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        ctx.accounts.claim()
    }
}
