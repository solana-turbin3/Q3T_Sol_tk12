use anchor_lang::prelude::*;

#[account]
pub struct Marketplace {
    pub admin: Pubkey,
    pub fee: u16,
    pub bump: u8,
    pub treasury_bump: u8,
    pub rewards_bump: u8,
    pub name: String, // set this to 32 as max length
}

impl Space for Marketplace {
    // Why `4 +`? In Rust, when you store a String, the length of the string is stored as a 32-bit unsigned integer (u32), which takes up 4 bytes.
    const INIT_SPACE: usize = 8 + 32 + (1 * 3) + (4 + 32);
}
