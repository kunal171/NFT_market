use anchor_spl::token;

use {
    anchor_lang::{
        prelude::*,
    },
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{Mint, Token, TokenAccount},
    },
};


pub fn list<'info>(
    ctx: Context<ListNFT>,
    expected_amount: u64
) -> Result<()> {

    // msg!("Initiating transfer of {} lamports...", sale_lamports);
    // msg!("Purchaser (sending lamports): {}", &ctx.accounts.buyer_authority.key());
    // msg!("Seller (receiving lamports): {}", &ctx.accounts.owner_authority.key());
    // system_program::transfer(
    //     CpiContext::new(
    //         ctx.accounts.system_program.to_account_info(),
    //         system_program::Transfer {
    //             from: ctx.accounts.buyer_authority.to_account_info(),
    //             to: ctx.accounts.owner_authority.to_account_info(),
    //         }
    //     ),
    //     sale_lamports
    // )?;
    let nft_mint =  &ctx.accounts.mint;
    let seller_token_account = &ctx.accounts.seller_token_account;
    let seller_wallet = &ctx.accounts.seller_wallet;
    let escrow = &mut ctx.accounts.escrow;
    let escrow_ata =  &ctx.accounts.escrow_token_account;
    let token_program = &ctx.accounts.token_program;
    let system_program = &ctx.accounts.system_program;
    let ata_program = &ctx.accounts.associated_token_program;
    let rent = &ctx.accounts.rent;
    //*Assign values to the Escrow */
    escrow.is_initialized = true;
    escrow.seller_pubkey = seller_wallet.key();
    escrow.token_account_pubkey = seller_token_account.key();
    escrow.mint_key = nft_mint.key();
    escrow.expected_amount = expected_amount;
    escrow.bump = *ctx.bumps.get("marketplace").unwrap();

    // let token_account_state = token::state::Account::unpack(
    //     &**seller_token_account.data.borrow()
    // ).unwrap();
    // // check if the token account have balance
    // if token_account_state.amount != (1 as u64){
    //     msg!("invalid NFT data ** ..");
    //     return Err(ProgramError::InvalidAccountData);
    // }
    msg!("Escrow Token Address: {}", &ctx.accounts.escrow_token_account.key());    

    msg!("Transferring NFT...");
    msg!("Owner Token Address: {}", &ctx.accounts.seller_token_account.key());    
    msg!("Escrow Token Address: {}", &ctx.accounts.escrow_token_account.key());    
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.seller_token_account.to_account_info(),
                to: ctx.accounts.escrow_token_account.to_account_info(),
                authority: ctx.accounts.seller_wallet.to_account_info(),
            }
        ),
        1
    )?;
    msg!("NFT transferred successfully.");

    Ok(())
}

#[account]
pub struct Escrow {
    pub is_initialized: bool,
    pub seller_pubkey: Pubkey,
    pub token_account_pubkey: Pubkey,
    pub mint_key: Pubkey,
    pub expected_amount: u64,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct ListNFT<'info> {
    #[account(mut)]
    pub mint: Account<'info, token::Mint>,
    #[account(mut)]
    pub seller_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub seller_wallet: Signer<'info>,
    #[account(mut)]
    pub escrow_token_account: Account<'info, TokenAccount>,
    #[account(init,
        payer = seller_token_account,
        space = 8 + 1 + 32 + 32 + 32 + 8 + 4 + 220, seeds = [b"marketplace", seller_wallet.key().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
