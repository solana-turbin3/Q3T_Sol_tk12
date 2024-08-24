use anchor_lang::prelude::*;

declare_id!("Gqk1QZfgp7PEbUfXT84xRHgcYHCbXDG8DRs4Ejo5UJNT");

pub mod errors;
pub use errors::*;

#[program]
pub mod nft_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn list(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn delist(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn purchase(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
