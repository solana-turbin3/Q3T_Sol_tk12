use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{MasterEditionAccount, Metadata, MetadataAccount},
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::state::{Listing, Marketplace};

#[derive(Accounts)]
pub struct List<'info> {
    #[account(mut)]
    maker: Signer<'info>,
    #[account()]
    marketplace: Account<'info, Marketplace>,
}
