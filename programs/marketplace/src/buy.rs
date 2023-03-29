use {
    anchor_lang::{
        prelude::*,
        system_program,
    },
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{Mint, Token, TokenAccount},
    },
};

use super::*;
use anchor_spl::token;

use crate::list::Escrow;

use crate::list;

pub fn buy(
    ctx: Context<BuyNft>,
    sale_lamports: u64
) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow;

    if escrow.expected_amount != sale_lamports {
        return Err(error!(BuyError::AmountMismatch));
    }
    let seller_pubkey = escrow.seller_pubkey.key();
    
    msg!("Initiating transfer of {} lamports...", sale_lamports);
    msg!("Purchaser (sending lamports): {}", &ctx.accounts.buyer_authority.key());
    msg!("Seller (receiving lamports): {}", seller_pubkey);
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.buyer_authority.to_account_info(),
                to: ctx.accounts.seller_account.to_account_info(),
            }
        ),
        sale_lamports
    )?;
    

    let escrow_signer_seeds = [
            "marketplace".as_bytes(),
            seller_pubkey.as_ref(),//* Wallet Key for the Signer
            &[escrow.bump], //* Escrow bump
    ];

    msg!("Lamports transferred successfully.");

    msg!("Transferring NFT...");
    msg!("Escrow Token Address: {}", &ctx.accounts.escrow_token_account.key());    
    msg!("Buyer Token Address: {}", &ctx.accounts.buyer_token_account.key());    
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.escrow_token_account.to_account_info(),
                to: ctx.accounts.buyer_token_account.to_account_info(),
                authority: ctx.accounts.escrow.to_account_info(),
            }
        ),
        1
    )?;
    msg!("NFT transferred successfully.");
    
    msg!("Sale completed successfully!");


    Ok(())
}


#[derive(Accounts)]
pub struct BuyNft<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub escrow_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub seller_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub escrow: Account<'info, Escrow>,
    /// CHECK: We're about to create this with Anchor
    #[account(mut)]
    pub buyer_token_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub buyer_authority: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[error_code]
pub enum BuyError {
    #[msg("AmountMismatch")]//301
    AmountMismatch,
}  