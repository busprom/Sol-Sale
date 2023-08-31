use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
  msg,
  account_info::AccountInfo,
  entrypoint::ProgramResult, 
  program::{invoke},
  pubkey::Pubkey,
  system_instruction,
  program_error::ProgramError
};
use crate::{
  STORAGE, SHARE_SEED,
  types::sell::Sell,
  token::transfer_token_seed::process_transfer_token_seed
};
use spl_token::state::Account;
use solana_program::program_pack::Pack;

pub fn process_sell_withdrawal<'a>(
  program_id: &Pubkey,
  mint: &AccountInfo<'a>,
  seller: &AccountInfo<'a>,
  buyer: &AccountInfo<'a>,
  token_transfer_account: &AccountInfo<'a>,
  vault: &AccountInfo<'a>,
  storage: &AccountInfo<'a>,
  applicant: &AccountInfo<'a>,
  profit_id: &AccountInfo<'a>,
  token_program: &AccountInfo<'a>,
  spl_token_program: &AccountInfo<'a>,
  rent_program: &AccountInfo<'a>,
  system_program: &AccountInfo<'a>
) -> ProgramResult {
  // проверку на апликанта сделать!!!
  if !buyer.is_signer { return Err(ProgramError::MissingRequiredSignature); }

  // проверка профит айди
  let (calc_profit, _) = Pubkey::find_program_address(
    &[SHARE_SEED.as_bytes(), program_id.as_ref(), program_id.as_ref()], &program_id
  );
  if calc_profit != *profit_id.key { return Err(ProgramError::InvalidArgument); }
  
  let mut settings = Sell::try_from_slice(&storage.data.borrow())?;
  if settings.vault != *vault.key { return Err(ProgramError::InvalidArgument); }
  if settings.seller != *seller.key { return Err(ProgramError::InvalidArgument); }

  let (calc_vault, vault_seed) = Pubkey::find_program_address(
    &[STORAGE.as_bytes(), program_id.as_ref(), mint.key.as_ref()], &program_id
  );
  if calc_vault != *vault.key { return Err(ProgramError::InvalidArgument); }
  let vault_signer_seeds = &[STORAGE.as_bytes(), program_id.as_ref(), mint.key.as_ref(), &[vault_seed]];

  let (calc_storage, _) = Pubkey::find_program_address(
    &[STORAGE.as_bytes(), program_id.as_ref(), vault.key.as_ref()], &program_id 
  );
  if calc_storage != *storage.key { return Err(ProgramError::InvalidArgument); }
  

  if settings.auction == 0 { // SELL TOKEN
    
    if settings.seller != *buyer.key {
      msg!("Trafsfer fee");
      invoke(
        &system_instruction::transfer(buyer.key, profit_id.key, (settings.price as f32 * 0.01) as u64),
        &[buyer.clone(), profit_id.clone(), system_program.clone()]
      )?;

      msg!("Payment for token");
      invoke(
        &system_instruction::transfer(buyer.key, seller.key, settings.price),
        &[buyer.clone(), seller.clone(), system_program.clone()]
      )?;
    }
    
    msg!("Transfer token");
    process_transfer_token_seed(
      buyer,
      buyer,
      mint,
      vault,
      token_transfer_account,
      profit_id,
      token_program,
      rent_program,
      system_program,
      spl_token_program,
      vault_signer_seeds
    )?;
    
    let dest_starting_lamports = profit_id.lamports();
    **profit_id.lamports.borrow_mut() = dest_starting_lamports
      .checked_add(storage.lamports())
      .unwrap();
    **storage.lamports.borrow_mut() = 0;

    msg!("Process withdrawal done");
    Ok(())
  }
  else { //AUCTION
    // вывод токена или денег продавцом (владельцем)
    if settings.seller == *buyer.key {// запрос владельца токена
      // если не было ставок, просто возвращаем токен
      // если были ставки, проверить чтобы совпадал token_transfer_account с аппликантом
      msg!("Transfer token to auction owner and close auction");
      process_transfer_token_seed(
        buyer,
        applicant,
        mint,
        vault,
        token_transfer_account,
        profit_id,
        token_program,
        rent_program,
        system_program,
        spl_token_program,
        vault_signer_seeds
      )?;

      let spl_token_account_data = &token_transfer_account.try_borrow_data()?;
      let spl_token_account = Account::unpack(&spl_token_account_data)?;

      // проверяем токен аккаунт
      if settings.applicant != *buyer.key {
        if spl_token_account.owner != settings.applicant {
          return Err(ProgramError::InvalidArgument);
        }
      }
      else {
        if spl_token_account.owner != settings.seller {
          return Err(ProgramError::InvalidArgument);
        }
      }
      
      if settings.applicant != *buyer.key { // значит были ставки и на аккаунте есть деньги
        // переводим ставку за токен продавцу, т.к. seller - это продавец
        let am = settings.price - (settings.price as f32 * 0.01) as u64;
        let dest_starting_lamports = seller.lamports();
        **seller.lamports.borrow_mut() = dest_starting_lamports.checked_add(am).unwrap();
        **storage.lamports.borrow_mut() = storage.lamports() - am;
      }
      
      msg!("Close storage");
      let dest_starting_lamports = profit_id.lamports();
      **profit_id.lamports.borrow_mut() = dest_starting_lamports
        .checked_add(storage.lamports())
        .unwrap();
      **storage.lamports.borrow_mut() = 0;

      msg!("Auction done");
      Ok(())
    }
    else {// значит запрос претендента

      if settings.applicant == *buyer.key {
        msg!("Cancel bet");
        let dest_starting_lamports = applicant.lamports();
        let ret = settings.price - 1000000;

        **applicant.lamports.borrow_mut() = dest_starting_lamports
          .checked_add(ret)
          .unwrap();
        **storage.lamports.borrow_mut() = storage.lamports() - ret;

        settings.price = settings.start_price;
        settings.applicant = settings.seller;
        settings.bets = 0;

        let _ = settings.serialize(&mut &mut storage.data.borrow_mut()[..]);

        msg!("Auction restart");

        Ok(())
      }
      else {
        msg!("Start new bet");

        let price: u64;
        if settings.seller != settings.applicant {
          price = settings.price + (settings.price / 100);
        }// сразу повышаем цену чтобы покупатель оплатил + 1%;
        else {
          price = settings.price;
        }
        invoke(// переводим на сторадж текущую стоимость токена
          &system_instruction::transfer(buyer.key, storage.key, price),
          &[buyer.clone(), storage.clone(), system_program.clone()]
        )?;

        if settings.seller != settings.applicant {// возвращаем предыдущему участнику sol
          let dest_starting_lamports = applicant.lamports();
          let ret = settings.price - 1000000;

          **applicant.lamports.borrow_mut() = dest_starting_lamports
            .checked_add(ret)
            .unwrap();
          **storage.lamports.borrow_mut() = storage.lamports() - ret;
        }

        settings.price = price;
        settings.applicant = *buyer.key;
        settings.bets = settings.bets + 1;

        let _ = settings.serialize(&mut &mut storage.data.borrow_mut()[..]);

        msg!("New bet done");
        Ok(())
      }

    }
    
  }
}