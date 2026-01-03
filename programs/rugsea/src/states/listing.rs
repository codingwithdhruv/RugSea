use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Listing {
    pub seller: Pubkey,
    pub nft_mint: Pubkey,
    pub nft_account: Pubkey,
    pub escrow: Pubkey,
    pub price: u64,
    pub bump: u8,
    pub is_active: bool,
}
impl Listing {
    pub const INIT_SPACE: usize = 8 + 32 * 4 + 8 + 1 + 1 + 8;
}