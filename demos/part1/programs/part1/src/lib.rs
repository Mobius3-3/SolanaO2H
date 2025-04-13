use anchor_lang::prelude::*;

declare_id!("2FtFkSFRV2GXQt4mhHHJxUMdZGHa7TqaKEkGUW9eYcoi");

#[program]
pub mod part1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[account]
pub struct MyStorage {
    value1: u8,
}
