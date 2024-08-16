use anchor_lang::prelude::*;

declare_id!("2YXFYtCXGdznhfqY9uM263QWmG3NEybNANk9n1viXEg3");

mod contexts;
use contexts::*;
mod state;
use state::*;

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, amount: u64, receive: u64) -> Result<()> {
        // initialize and save the data that the escrow needs to manage and regulate the terms
        ctx.accounts.save_escrow(seed, receive, ctx.bumps.escrow)?;
        // the user that is making the offer deposits the agreed amount to the escrow and it is put into the vault
        ctx.accounts.deposit_to_vault(amount)
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        // transfer from taker to the maker
        ctx.accounts.transfer_to_maker()?;
        // once the transfer above is complete, send what is in the vault to the taker and close the account and recover the rent
        ctx.accounts.withdraw_and_close()
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.withdraw_and_close()
    }
}
