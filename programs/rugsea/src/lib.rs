use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

pub mod states;
use states::listing::Listing;

declare_id!("HZQUX8ZpU5rtEpDA4R47hL9KJrUFsGXgYGEHekYyeDEN");

#[program]
pub mod rugsea {
    use super::*;

    pub fn create_listing(ctx: Context<CreateListing>, price: u64) -> Result<()> {
        let listing = &mut ctx.accounts.listing;

        listing.seller = ctx.accounts.seller.key();
        listing.nft_mint = ctx.accounts.nft_mint.key();
        listing.nft_account = ctx.accounts.seller_nft_account.key();
        listing.escrow = ctx.accounts.escrow.key();
        listing.price = price;
        listing.bump = ctx.bumps.listing;
        listing.is_active = true;

        let cpi_accounts = anchor_spl::token::Transfer {
            from: ctx.accounts.seller_nft_account.to_account_info(),
            to: ctx.accounts.escrow.to_account_info(),
            authority: ctx.accounts.seller.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        anchor_spl::token::transfer(cpi_ctx, 1)?;

        msg!("Listing created: {} SOL for NFT {}", price / 1_000_000_000u64, listing.nft_mint);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateListing<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    
    #[account(
        init,
        payer = seller,
        space = Listing::INIT_SPACE,
        seeds = [b"listing", seller.key().as_ref(), nft_mint.key().as_ref()],
        bump,
    )]
    pub listing: Account<'info, Listing>,

    /// CHECK: NFT mint account (verified by client)
    pub nft_mint: AccountInfo<'info>,

    #[account(mut)]
    pub seller_nft_account: Account<'info, TokenAccount>,

    /// CHECK: Escrow token account (client creates first)
    #[account(mut)]
    pub escrow: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

