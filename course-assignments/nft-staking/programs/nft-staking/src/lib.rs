use anchor_lang::prelude::*;

declare_id!("AtnsmbdumGmkvn9j4fBy9pSk2XyKEv6WsqWHQXg2BUvH");

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
