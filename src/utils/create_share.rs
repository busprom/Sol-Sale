use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::{
	pubkey::Pubkey, msg,
	entrypoint::ProgramResult,
	account_info::AccountInfo,
  program::invoke_signed,
  sysvar::{clock::Clock, Sysvar, rent::Rent},
  system_instruction,
};
use crate::{
  types::{
    share::{Share, ShareStorage, ShareData},
    metadata::{CreateMetadataArgs, Data, Metadata}
  },
  token::mint_token::process_mint_token,
  error::NftError,
  SHARER, SHARE_SEED
};

pub fn process_create_share<'a>(
  program_id: &Pubkey,
  payer: &AccountInfo<'a>,
	share_storage: &AccountInfo<'a>,
  one_storage: &AccountInfo<'a>,
  mint: &AccountInfo<'a>,
  mint_account: &AccountInfo<'a>,
  metadata_account: &AccountInfo<'a>,
  metadata_program: &AccountInfo<'a>,
  profit_id: &AccountInfo<'a>,
  token_program: &AccountInfo<'a>,
  spl_token_program: &AccountInfo<'a>,
  rent_program: &AccountInfo<'a>,
  system_program: &AccountInfo<'a>,
  data: ShareData
) -> ProgramResult {
  msg!("Create share start");
  if !payer.is_signer { return Err(NftError::WrongOwnerNFR.into()); }
  if &payer.key.to_string() != SHARER { return Err(NftError::WrongOwnerNFR.into()); }
  if !one_storage.data_is_empty() { return Err(NftError::WrongOwnerNFR.into()); }

  let (calc_profit, _) = Pubkey::find_program_address(
    &[SHARE_SEED.as_bytes(), program_id.as_ref(), program_id.as_ref()], &program_id
  );
  if calc_profit != *profit_id.key { return Err(NftError::WrongOwnerNFR.into()); }

  let (calc_storage, storage_seed) = Pubkey::find_program_address(
    &[SHARE_SEED.as_bytes(), program_id.as_ref(), program_id.as_ref()], &program_id
  );
  if calc_storage != *share_storage.key { return Err(NftError::WrongSettingsPDA.into()); }
  let storage_signer_seeds = &[SHARE_SEED.as_bytes(), program_id.as_ref(), program_id.as_ref(), &[storage_seed]];

  let rent = &Rent::from_account_info(rent_program)?;

  if share_storage.data_is_empty() {
    msg!("Create share storage");
    let data = ShareStorage {
      owner: *payer.key,
      amount: 0,
      total: 0
    };

    let space = data.try_to_vec()?.len();
    let lamports = rent.minimum_balance(space);
    invoke_signed(
      &system_instruction::create_account(
        payer.key,
        share_storage.key,
        lamports,
        space as u64,
        &program_id
      ),
      &[payer.clone(), share_storage.clone(), system_program.clone()],
      &[storage_signer_seeds],
    )?;

    let _ = data.serialize(&mut &mut share_storage.data.borrow_mut()[..]);
  }

  let mut storage = ShareStorage::try_from_slice(&share_storage.data.borrow())?;
  if storage.owner.to_string() != SHARER { return Err(NftError::WrongOwnerNFR.into()); }
  if storage.amount == 1000 { return Err(NftError::WrongOwnerNFR.into()); }
  if storage.owner != *payer.key { return Err(NftError::WrongOwnerNFR.into()); }

  let (calc_one, one_seed) = Pubkey::find_program_address(
    &[SHARE_SEED.as_bytes(), program_id.as_ref(), mint.key.as_ref()], &program_id
  );
  if calc_one != *one_storage.key { return Err(NftError::WrongSettingsPDA.into()); }
  let one_storage_seeds = &[SHARE_SEED.as_bytes(), program_id.as_ref(), mint.key.as_ref(), &[one_seed]];

  let cl = Clock::get().unwrap();
  let share_data = Share {
    storage: *share_storage.key,
    mint: *mint.key,
    time_create: cl.unix_timestamp as u64,
    last_total: 0,
    total_paid: 0,
    last_paid: 0,
    last_time_paid: 0
  };

  msg!("Create share storage");
  let space = share_data.try_to_vec()?.len();
  let lamports = rent.minimum_balance(space);
  invoke_signed(
    &system_instruction::create_account(
      payer.key,
      one_storage.key,
      lamports,
      space as u64,
      &program_id
    ),
    &[payer.clone(), one_storage.clone(), system_program.clone()],
    &[one_storage_seeds],
  )?;
  let _ = share_data.serialize(&mut &mut one_storage.data.borrow_mut()[..]);
  
  process_mint_token(
    payer,
    mint,
    mint_account,
    metadata_account,
    metadata_program,
    profit_id,
    token_program,
    spl_token_program,
    rent_program,
    system_program,
    CreateMetadataArgs {
      metadata: Metadata {
        instruction: 0,
        data: Data {
          name: "Stock CM".to_string(),
          symbol: "CM".to_string(),
          uri: "https://cryptomore.me/meta/stock".to_string(),
          seller_fee_basis_points: 0,
          creators: None
        },
        is_mutable: 0
      },
      kind: "mint".to_string(),
      metaplex: data.metaplex
    }
  )?;

  storage.amount += 1;
  let _ = storage.serialize(&mut &mut share_storage.data.borrow_mut()[..]);

  Ok(())
}