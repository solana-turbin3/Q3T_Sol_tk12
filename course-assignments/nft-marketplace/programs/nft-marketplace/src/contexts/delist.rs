use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::state::{Listing, Marketplace};

#[derive(Accounts)]
pub struct Delist<'info> {
    #[account(mut)]
    maker: Signer<'info>,
    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump,
    )]
    marketplace: Box<Account<'info, Marketplace>>,
    maker_mint: Box<InterfaceAccount<'info, Mint>>,
    collection_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        associated_token::authority = maker,
        associated_token::mint = maker_mint,
    )]
    maker_ata: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init,
        payer = maker,
        seeds = [b"listing", maker.key().as_ref(), maker_mint.key().as_ref()],
        bump,
        space = Listing::INIT_SPACE,
    )]
    listing: Account<'info, Listing>,
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = maker_mint,
        associated_token::authority = listing,
    )]
    vault: Box<InterfaceAccount<'info, TokenAccount>>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
    token_program: Interface<'info, TokenInterface>,
}

impl<'info> Delist<'info> {
    pub fn withdraw_nft(&mut self) -> Result<()> {
        let seeds = &[
            &self.marketplace.key().to_bytes()[..],
            &self.maker_mint.key().to_bytes()[..],
            &[self.listing.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let accounts = TransferChecked {
            to: self.maker_ata.to_account_info(),
            from: self.vault.to_account_info(),
            mint: self.maker_ata.to_account_info(),
            authority: self.maker.to_account_info(),
        };

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        transfer_checked(ctx, 1, self.maker_mint.decimals)?;

        // Reset the state of the Listing account
        self.listing.set_inner(Listing {
            maker: Pubkey::default(),
            mint: Pubkey::default(),
            price: 0,
            bump: 0,
        });
        Ok(())

        // OR we could close accounts of vault and listing
        // let close_vault_account = CloseAccount {
        //     account: self.vault.to_account_info(),
        //     destination: self.maker.to_account_info(),
        //     authority: self.listing.to_account_info(),
        // };

        // let close_vault_ctx = CpiContext::new_with_signer(
        //     self.token_program.to_account_info(),
        //     close_vault_account,
        //     signer_seeds,
        // );

        // close_account(close_vault_ctx)?;

        // let close_listing_account = CloseAccount {
        //     account: self.listing.to_account_info(),
        //     destination: self.maker.to_account_info(),
        //     authority: self.listing.to_account_info(),
        // };

        // let close_listing_ctx = CpiContext::new_with_signer(
        //     self.token_program.to_account_info(),
        //     close_listing_account,
        //     signer_seeds,
        // );

        // close_account(close_listing_ctx)
    }
}
