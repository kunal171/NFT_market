mod list;
mod buy;
mod cancel;

use anchor_lang::prelude::*;
use list::*;
use buy::*;

declare_id!("DuFfA9WLCUakkgUWvNQQJH7NAnBSEuUBK5tZ5r1CSrD5");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
