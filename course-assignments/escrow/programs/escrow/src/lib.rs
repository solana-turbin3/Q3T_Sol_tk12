use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};
declare_id!("2YXFYtCXGdznhfqY9uM263QWmG3NEybNANk9n1viXEg3");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
        // Create the escrow state account
        ctx.accounts.escrow.set_inner(Escrow {
            seed,
            maker: ctx.accounts.maker.key(),
            mint_a: ctx.accounts.mint_a.key(),
            mint_b: ctx.accounts.mint_b.key(),
            receive,
            bump: ctx.bumps.escrow,
        });

        // Create a TransferChecked instruction to deposit the maker's funds into the vault.
        let transfer_accounts = TransferChecked {
            from: ctx.accounts.maker_ata_a.to_account_info(),
            mint: ctx.accounts.mint_a.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.maker.to_account_info(),
        };

        // Create a CPI context for the transfer instruction.
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_accounts,
        );

        // Execute the transfer_checked instruction.
        transfer_checked(cpi_ctx, deposit, ctx.accounts.mint_a.decimals)
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        // Create the signer seeds for the escrow account in order to sign on behalf of the vault.
        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"escrow",
            ctx.accounts.maker.to_account_info().key.as_ref(),
            &ctx.accounts.escrow.seed.to_le_bytes()[..],
            &[ctx.accounts.escrow.bump],
        ]];

        // Create a TransferChecked instruction to transfer the maker's funds from the vault to the maker's ATA.
        let xfer_accounts = TransferChecked {
            from: ctx.accounts.vault.to_account_info(),
            mint: ctx.accounts.mint_a.to_account_info(),
            to: ctx.accounts.maker_ata_a.to_account_info(),
            authority: ctx.accounts.escrow.to_account_info(),
        };

        // Create a CPI context with signer for the transfer instruction.
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            xfer_accounts,
            &signer_seeds,
        );

        // Execute the transfer_checked instruction.
        transfer_checked(
            cpi_ctx,
            ctx.accounts.vault.amount,
            ctx.accounts.mint_a.decimals,
        )?;

        // Create a CloseAccount instruction to close the vault account.
        let close_accounts = CloseAccount {
            account: ctx.accounts.vault.to_account_info(),
            destination: ctx.accounts.maker.to_account_info(),
            authority: ctx.accounts.escrow.to_account_info(),
        };

        // Create a CPI context with signer for the close instruction.
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            close_accounts,
            &signer_seeds,
        );

        // Execute the close_account instruction.
        close_account(cpi_ctx)
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        // Create a TransferChecked instruction to transfer the taker's funds from the taker's ATA to the maker's ATA.
        let transfer_accounts = TransferChecked {
            from: ctx.accounts.taker_ata_b.to_account_info(),
            mint: ctx.accounts.mint_b.to_account_info(),
            to: ctx.accounts.maker_ata_b.to_account_info(),
            authority: ctx.accounts.taker.to_account_info(),
        };

        // Create a CPI context for the transfer instruction.
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_accounts,
        );

        // Execute the transfer_checked instruction.
        transfer_checked(
            cpi_ctx,
            ctx.accounts.escrow.receive,
            ctx.accounts.mint_b.decimals,
        )?;

        // Create the signer seeds for the escrow account in order to sign on behalf of the vault.
        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"escrow",
            ctx.accounts.maker.to_account_info().key.as_ref(),
            &ctx.accounts.escrow.seed.to_le_bytes()[..],
            &[ctx.accounts.escrow.bump],
        ]];

        // Create a TransferChecked instruction to transfer the maker's funds from the vault to the taker's ATA.
        let accounts = TransferChecked {
            from: ctx.accounts.vault.to_account_info(),
            mint: ctx.accounts.mint_a.to_account_info(),
            to: ctx.accounts.taker_ata_a.to_account_info(),
            authority: ctx.accounts.escrow.to_account_info(),
        };

        // Create a CPI context with signer for the transfer instruction.
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            accounts,
            &signer_seeds,
        );

        // Execute the transfer_checked instruction.
        transfer_checked(
            cpi_ctx,
            ctx.accounts.vault.amount,
            ctx.accounts.mint_a.decimals,
        )?;

        // Create a CloseAccount instruction to close the vault account.
        let accounts = CloseAccount {
            account: ctx.accounts.vault.to_account_info(),
            destination: ctx.accounts.taker.to_account_info(),
            authority: ctx.accounts.escrow.to_account_info(),
        };

        // Create a CPI context with signer for the close instruction.
        let ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            accounts,
            &signer_seeds,
        );

        // Execute the close_account instruction.
        close_account(ctx)
    }
}

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub seed: u64,
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub receive: u64,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub maker: SystemAccount<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_ata_a: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_ata_b: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_ata_b: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        close = maker,
        has_one = maker,
        has_one = mint_a,
        has_one = mint_b,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump
    )]
    escrow: Account<'info, Escrow>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    maker: Signer<'info>,
    mint_a: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    maker_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        close = maker,
        has_one = mint_a,
        has_one = maker,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump
    )]
    escrow: Account<'info, Escrow>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    associated_token_program: Program<'info, AssociatedToken>,
    token_program: Interface<'info, TokenInterface>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mint::token_program = token_program
    )]
    pub mint_a: InterfaceAccount<'info, Mint>,
    #[account(
        mint::token_program = token_program
    )]
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer = maker,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

// Below is how Dean live coded the lib.rs
// use anchor_lang::prelude::*;

// mod contexts;
// use contexts::*;
// mod state;
// use state::*;

// declare_id!("2YXFYtCXGdznhfqY9uM263QWmG3NEybNANk9n1viXEg3");

// #[program]
// pub mod escrow {
//     use super::*;
//     // these arguments must be made in this order (ie: ctx, seed, amount etc) or things will break
//     pub fn make(ctx: Context<Make>, seed: u64, amount: u64, receive: u64) -> Result<()> {
//         // initialize and save the data that the escrow needs to manage and regulate the terms
//         ctx.accounts.save_escrow(seed, receive, ctx.bumps.escrow)?;
//         // the user that is making the offer deposits the agreed amount to the escrow and it is put into the vault
//         ctx.accounts.deposit_to_vault(amount)
//     }

//     pub fn take(ctx: Context<Take>) -> Result<()> {
//         // transfer from taker to the maker
//         ctx.accounts.transfer_to_maker()?;
//         // once the transfer above is complete, send what is in the vault to the taker and close the account and recover the rent
//         ctx.accounts.withdraw_and_close()
//     }

//     pub fn refund(ctx: Context<Refund>) -> Result<()> {
//         ctx.accounts.withdraw_and_close()
//     }
// }
