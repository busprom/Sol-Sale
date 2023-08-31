use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
  pubkey::Pubkey, msg,
  entrypoint::ProgramResult,
  account_info::{AccountInfo, next_account_info},
  sysvar::{rent::Rent, Sysvar},
  program::invoke_signed,
  system_instruction
};
use crate::{
  error::NftError, LOTTERY_NFT, LOTTERY_SEED,
  token::mint_token::process_mint_token,
  types::{
    nft_lottery::{NftLottery, BoxData, SaveToken},
    lottery::Lottery,
    metadata::{CreateMetadataArgs, Metadata, Data}
  },
  SHARE_SEED
};
use solana_program::program_pack::Pack;

pub fn process_save_token<'a>(
  program_id: &Pubkey,
  accounts: &[AccountInfo],
  data: SaveToken
) -> ProgramResult {
  let account_info_iter = &mut accounts.iter();
  let payer = next_account_info(account_info_iter)?;
  let burned_token = next_account_info(account_info_iter)?;
  let burned_token_storage = next_account_info(account_info_iter)?;
  let mint = next_account_info(account_info_iter)?;
  let mint_account = next_account_info(account_info_iter)?;
  let metadata_account = next_account_info(account_info_iter)?;
  let metadata_program = next_account_info(account_info_iter)?;
  let profit_id = next_account_info(account_info_iter)?;
  let storage = next_account_info(account_info_iter)?;
  let mint_storage = next_account_info(account_info_iter)?;
  let token_program = next_account_info(account_info_iter)?;
  let spl_token_program = next_account_info(account_info_iter)?;
  let rent_program = next_account_info(account_info_iter)?;
  let system_program = next_account_info(account_info_iter)?;

  if !payer.is_signer { return Err(NftError::AdminRequired.into()); }

  // значит токен использован уже
  if burned_token_storage.data_is_empty() { return Err(NftError::WrongOwnerNFR.into()); }

  // проверка что действительно токена нет
  let mint_data = spl_token::state::Mint::unpack(&burned_token.data.borrow())?;
  if mint_data.supply > 0 { return Err(NftError::WrongSettingsPDA.into()); }
  drop(mint_data);

  // все проверки отсюда
  let mut burned_box_data = BoxData::try_from_slice(&burned_token_storage.data.borrow())?;
  if burned_box_data.seed != data.box_data.seed { return Err(NftError::WrongSettingsPDA.into()); }
  if burned_box_data.owner != data.box_data.owner { return Err(NftError::WrongSettingsPDA.into()); }

  // проверяем потеряный токен на соответствие
  let (calc_burnet_storage, _) = Pubkey::find_program_address(
    &[burned_box_data.seed.as_bytes(), program_id.as_ref(), burned_token.key.as_ref()], &program_id
  );
  if calc_burnet_storage != *burned_token_storage.key { return Err(NftError::WrongSettingsPDA.into()); }

  // проверяем сторадж лоттереи
  let (calc_storage, _) = Pubkey::find_program_address(
    &[burned_box_data.seed.as_bytes(), program_id.as_ref(), data.box_data.owner.as_ref()], &program_id
  );
  if calc_storage != *storage.key { return Err(NftError::WrongSettingsPDA.into()); }

  let (calc_profit, _) = Pubkey::find_program_address(
    &[SHARE_SEED.as_bytes(), program_id.as_ref(), program_id.as_ref()], &program_id
  );
  if calc_profit != *profit_id.key { return Err(NftError::WrongOwnerNFR.into()); }
  
  let mut meta = CreateMetadataArgs {
    metadata: Metadata {
      instruction: 0,
      data: Data {
        name: '_'.to_string(),
        symbol: '_'.to_string(),
        uri: '_'.to_string(),
        seller_fee_basis_points: 0,
        creators: None
      },
      is_mutable: 0
    },
    kind: '_'.to_string(),
    metaplex: data.metaplex
  };

  if burned_box_data.seed == LOTTERY_SEED {
    let lottery_data = Lottery::try_from_slice(&storage.data.borrow())?;
    if lottery_data.owner != burned_box_data.owner { return Err(NftError::WrongSettingsPDA.into()); }
    if *storage.key != burned_box_data.storage { return Err(NftError::WrongSettingsPDA.into()); }
    meta.kind = "lottery_box".to_string();
    meta.metadata.data.name = "SOL in Box".to_string();
    meta.metadata.data.symbol = "BOX".to_string();
    meta.metadata.data.uri = lottery_data.url + &"meta/".to_string() + &*storage.key.to_string();
  }

  if burned_box_data.seed == LOTTERY_NFT {
    let lottery_data = NftLottery::try_from_slice(&storage.data.borrow())?;
    if lottery_data.owner != burned_box_data.owner { return Err(NftError::WrongSettingsPDA.into()); }
    if *storage.key != burned_box_data.storage { return Err(NftError::WrongSettingsPDA.into()); }
    meta.kind = "nft_box".to_string();
    meta.metadata.data.name = "NFT in Box".to_string();

    if burned_box_data.is_box == "BOX".to_string() {
      meta.metadata.data.symbol = "BOX".to_string();
      meta.metadata.data.uri = lottery_data.clone().url + &"meta/".to_string() + &*storage.key.to_string();
    }

    if burned_box_data.is_box == "NFT".to_string() {
      meta.metadata.data.symbol = "NIB".to_string();
      let i = burned_box_data.index as usize;
      meta.metadata.data.uri = lottery_data.url.to_string() + &"meta/".to_string() + &lottery_data.pictures[i].to_string();
    }

  }

  // значит поддельный сид
  if meta.kind == "_".to_string() { return Err(NftError::WrongSettingsPDA.into()); }

  msg!("Create box info account");
  let (calc_mint_storage, mint_seed) = Pubkey::find_program_address(
    &[burned_box_data.seed.as_bytes(), program_id.as_ref(), mint.key.as_ref()], &program_id
  );
  if calc_mint_storage != *mint_storage.key { return Err(NftError::WrongSettingsPDA.into()); }
  let mint_storage_signer_seeds = &[burned_box_data.seed.as_bytes(), program_id.as_ref(), mint.key.as_ref(), &[mint_seed]];
  
  let rent = &Rent::from_account_info(rent_program)?;
  let space = burned_box_data.try_to_vec()?.len();
  let lamports = rent.minimum_balance(space);
  invoke_signed(
    &system_instruction::create_account(
      payer.key,
      mint_storage.key,
      lamports * 4,
      space as u64,
      &program_id
    ),
    &[payer.clone(), mint_storage.clone(), system_program.clone()],
    &[mint_storage_signer_seeds],
  )?;
  burned_box_data.token = *mint.key;
  let _ = burned_box_data.serialize(&mut &mut mint_storage.data.borrow_mut()[..]);

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
    meta.clone()
  )?;
  
  msg!("Close mint account");
  let dest_starting_lamports = profit_id.lamports();
  **profit_id.lamports.borrow_mut() = dest_starting_lamports.checked_add(burned_token_storage.lamports()).unwrap();
  **burned_token_storage.lamports.borrow_mut() = 0;

  Ok(())
}