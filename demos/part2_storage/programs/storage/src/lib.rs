use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("7jP8p6SaboVP11VGqs46yjvLLanmnJso9NhRGPoZBNo");

#[program]
pub mod storage {
    use super::*;

    pub fn create_user(ctx: Context<CreateUser>, key: String, name: String, address: String) -> Result<()> {
        msg!("Received key: {}", key.as_str());

        // try out different ways to store string
        ctx.accounts.user_account.set_string(&name); 
        ctx.accounts.user_account.address = address;

        let user_name = ctx.accounts.user_account.get_string();
        msg!("user name is: {}", user_name?);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(key: String)] // <-- makes `key` available in constraints
pub struct CreateUser<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + 4 + 20 + 20,
        seeds = [&key.as_bytes().as_ref()], // to_le_bytes()
        bump
    )]
    pub user_account: Account<'info, User>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct User {
    name: Vec<u8>,
    address: String,
}

impl User {
    pub fn set_string(&mut self, input: &str) {
        self.name = input.as_bytes().to_vec();
    }

    pub fn get_string(&self) -> Result<String> {
        String::from_utf8(self.name.clone())
            .map_err(|_| MyError::InvalidUtf8.into())
    }
}

#[error_code]
pub enum MyError {
    #[msg("Invalid UTF-8 data.")]
    InvalidUtf8,
}

