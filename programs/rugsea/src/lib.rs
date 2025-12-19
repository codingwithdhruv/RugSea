use anchor_lang::prelude::*;

declare_id!("HZQUX8ZpU5rtEpDA4R47hL9KJrUFsGXgYGEHekYyeDEN");

#[program]
pub mod rugsea {
    use super::*;

    pub fn create_listing(ctx: Context<CreateListing>, price: u64) -> Result<()> {
        let _ctx = ctx;
        let _price = price;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateListing<'info> {
    pub seller: Signer<'info>,
}

#[account]
pub struct Listing {
    pub seller: Pubkey,
    pub nft_mint: Pubkey,
    pub price: u64,
}
