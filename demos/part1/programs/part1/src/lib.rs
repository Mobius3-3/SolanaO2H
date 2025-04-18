use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("2FtFkSFRV2GXQt4mhHHJxUMdZGHa7TqaKEkGUW9eYcoi");

#[program]
pub mod crud {
    use super::*;

    pub fn create(ctx: Context<Create>) -> Result<()> {
        Ok(())
    }

    pub fn update(ctx: Context<Update>, new_value1: u64) -> Result<()> {
        ctx.accounts.my_storage.value1 = new_value1;
        Ok(())
    }

    pub fn delete(ctx: Context<Delete>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init,
              payer = signer,
              space=size_of::<MyStorage>() + 8,
              seeds = [],
              bump)]
    pub my_storage: Account<'info, MyStorage>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut, seeds = [], bump)]
    pub my_storage: Account<'info, MyStorage>,
}

#[derive(Accounts)]
pub struct Delete<'info> {
    #[account(mut, close = signer)]
    pub my_storage: Account<'info, MyStorage>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

#[account]
pub struct MyStorage {
    value1: u64,
}
