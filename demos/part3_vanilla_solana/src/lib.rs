use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hello from vanilla Rust Solana program!");

    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    msg!("Account key: {}", account.key);

    Ok(())
}
