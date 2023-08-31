use solana_program::{
  msg, program::{invoke},
  account_info::AccountInfo,
  entrypoint::ProgramResult
};
use crate::{
  token::create_token_account::process_create_token_account
};

pub fn process_transfer_token<'a>(
  payer: &AccountInfo<'a>,
  mint: &AccountInfo<'a>,
  from_token_accaunt: &AccountInfo<'a>,
  to_token_accaunt: &AccountInfo<'a>,
  profit_id: &AccountInfo<'a>,
  token_program: &AccountInfo<'a>,
  rent_program: &AccountInfo<'a>,
  system_program: &AccountInfo<'a>,
  spl_token_program: &AccountInfo<'a>
) -> ProgramResult {
  
  msg!("Create ATA");
  process_create_token_account(
    payer,
    payer,
    mint,
    to_token_accaunt,
    token_program,
    rent_program,
    system_program,
    spl_token_program
  )?;
  
  msg!("Transfer token");
  invoke(
    &spl_token::instruction::transfer(
      token_program.key,
      from_token_accaunt.key,
      to_token_accaunt.key,
      from_token_accaunt.key,
      &[from_token_accaunt.key],
      1
    ).unwrap(),
    &[token_program.clone(), from_token_accaunt.clone(), to_token_accaunt.clone(), from_token_accaunt.clone()]
  )?;

  msg!("Close token account");
  invoke(
    &spl_token::instruction::close_account(
      token_program.key,
      from_token_accaunt.key,
      profit_id.key,
      from_token_accaunt.key,
      &[from_token_accaunt.key]
    ).unwrap(),
    &[from_token_accaunt.clone(), profit_id.clone(), from_token_accaunt.clone()]
  )?;

  Ok(())
}