use anchor_lang::prelude::*;
// this is compatible with spl_token propgram AND spl_token 2022 program
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::Escrow;

#[derive(Accounts)]
// this allows us to get access to the seed in lib.rs pub fn make()
#[instruction[seed: u64]]
pub struct Make<'info> {
    #[account(mut)]
    maker: Signer<'info>,
    #[account(
        mint::token_program = token_program,
    )]
    mint_a: InterfaceAccount<'info, Mint>,
    #[account(
        mint::token_program = token_program,
    )]
    mint_b: InterfaceAccount<'info, Mint>,
    #[account(mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    maker_ata_a: InterfaceAccount<'info, TokenAccount>,
    // stores the data for the escrow
    #[account(
        init,
        payer = maker,
        space = 8 + Escrow::INIT_SPACE,
        // we use seeds and bump because this is going to be a PDA
        seeds = [b"escrow",maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    escrow: Account<'info, Escrow>,
    // ata owned by the escrow
    #[account(
        init_if_needed,
        // we always need a payer when an account needs to be initialized
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    vault: InterfaceAccount<'info, TokenAccount>,
    // calculates ata's
    associated_token_program: Program<'info, AssociatedToken>,
    // used for opening mints and token transfers
    token_program: Interface<'info, TokenInterface>,
    // handles the escrow state
    system_program: Program<'info, System>,
}

impl<'info> Make<'info> {
    pub fn save_escrow(&mut self, seed: u64, receive: u64, bump: u8) -> Result<()> {
        self.escrow.set_inner(Escrow {
            seed,
            maker: self.maker.key(),
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            receive,
            bump,
        });
        Ok(())
    }

    pub fn deposit_to_vault(&mut self, amount: u64) -> Result<()> {
        let accounts = TransferChecked {
            from: self.maker_ata_a.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let ctx = CpiContext::new(self.token_program.to_account_info(), accounts);

        transfer_checked(ctx, amount, self.mint_a.decimals);

        Ok(())
    }
}
