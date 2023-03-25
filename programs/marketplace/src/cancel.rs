use anchor_spl::token;

use {
    anchor_lang::{
        prelude::*,
        system_program,
    },
    anchor_spl::{
        associated_token::AssociatedToken,
        token::*,
    },
};
use crate::list::Escrow;

pub fn list(
    ctx: Context<CancelListing>
) -> Result<()> {

    let nft_mint =  &ctx.accounts.mint;
    let seller_token_account = &ctx.accounts.seller_token_account;
    let seller_wallet = &ctx.accounts.seller_wallet;
    let escrow = &mut ctx.accounts.escrow;
    let escrow_ata =  &ctx.accounts.escrow_token_account;
    let token_program = &ctx.accounts.token_program;
    let system_program = &ctx.accounts.system_program;
    let ata_program = &ctx.accounts.associated_token_program;
    let rent = &ctx.accounts.rent;

    if escrow.seller_pubkey != seller_wallet.key() {
        return Err(CancelError::AmountMismatch.into());
    }

    // let token_account_state = token::state::Account::unpack(
    //     &**seller_token_account.data.borrow()
    // ).unwrap();
    // // check if the token account have balance
    // if token_account_state.amount != (1 as u64){
    //     msg!("invalid NFT data ** ..");
    //     return Err(ProgramError::InvalidAccountData);
    // }

    msg!("Escrow Token Address: {}", &ctx.accounts.escrow_token_account.key());    
    let seller_pubkey = escrow.seller_pubkey.key();

    let escrow_signer_seeds = [
            "marketplace".as_bytes(),
            seller_pubkey.as_ref(),//* Wallet Key for the Signer
            &[escrow.bump], //* Escrow bump
    ];

    msg!("Transferring NFT...");
    msg!("Owner Token Address: {}", &ctx.accounts.seller_token_account.key());    
    msg!("Escrow Token Address: {}", &ctx.accounts.escrow_ata.key());    
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.escrow_ata.to_account_info(),
                to: ctx.accounts.seller_token_account.to_account_info(),
                authority: ctx.accounts.escrow.to_account_info(),
            }
            &[&escrow_signer_seeds],
        ),
        1
    )?;
    msg!("NFT withdraw successfully.");
        //*Assign values to the Escrow */
        escrow.is_initialized = false;
        escrow.seller_pubkey = seller_wallet.key();

    Ok(())
}

#[derive(Accounts)]
pub struct CancelListing<'info> {
    #[account(mut)]
    pub mint: Account<'info, token::Mint>,
    #[account(mut)]
    pub seller_token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub seller_wallet: Signer<'info>,
    #[account(mut)]
    pub escrow_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub escrow: Account<'info, Escrow>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, TokenAccount>,
}

#[error]
pub enum CancelError {
    #[msg("WrongListerMismatch")]//301
    WrongListerMismatch,
}  