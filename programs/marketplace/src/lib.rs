mod list;
mod buy;
mod cancel;

use anchor_lang::prelude::*;
use list::*;
use buy::*;
use cancel::*;

declare_id!("DuFfA9WLCUakkgUWvNQQJH7NAnBSEuUBK5tZ5r1CSrD5");

#[program]
pub mod marketplace {
    use super::*;

    pub fn listnft(
        ctx: Context<ListNFT>,
        expected_amount: u64
    ) -> Result<()> {
        list::list(
            ctx,
            expected_amount,
        )
    }

    pub fn buynft(
        ctx: Context<BuyNft>,
        sale_lamports: u64
    ) -> Result<()> {
        buy::buy(
            ctx,
            sale_lamports,
        )
    }

    pub fn cancellisting(
        ctx: Context<CancelListing>,
    ) -> Result<()> {
        cancel::cancel(
            ctx,
        )
    }

    
}
