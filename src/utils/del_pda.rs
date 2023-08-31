use borsh::BorshDeserialize;
use solana_program::{
	pubkey::Pubkey, msg,
	entrypoint::ProgramResult,
	account_info::AccountInfo
};
use crate::{
  types::{
    lottery::Lottery,
    nft_lottery::NftLottery
  },
  error::NftError,
  LOTTERY_SEED, LOTTERY_NFT, SHARE_SEED
};

pub fn process_del_pda<'a>(
  program_id: &Pubkey,
  payer: &AccountInfo<'a>,
  pda_account: &AccountInfo<'a>,
  profit_id: &AccountInfo<'a>
) -> ProgramResult {
  if !payer.is_signer { return Err(NftError::WrongOwnerNFR.into()); }

  let (calc_profit, _) = Pubkey::find_program_address(
    &[SHARE_SEED.as_bytes(), program_id.as_ref(), program_id.as_ref()], &program_id
  );
  if calc_profit != *profit_id.key { return Err(NftError::WrongOwnerNFR.into()); }

  let (calc_lottery, _) = Pubkey::find_program_address(
    &[LOTTERY_SEED.as_bytes(), program_id.as_ref(), payer.key.as_ref()], &program_id
  );

  if calc_lottery == *pda_account.key {
    let lottery = Lottery::try_from_slice(&pda_account.data.borrow())?;
    if lottery.owner != *payer.key { return Err(NftError::NotDrawn.into()); }

    let mut wins: u64 = 0;
    for i in 0..lottery.lots.len() {
      wins += lottery.lots[i].wins;
    }

    if wins != lottery.create_box { return Err(NftError::NotDrawn.into()); }

    msg!("Close lottery account");
    let dest_starting_lamports = profit_id.lamports();
    **profit_id.lamports.borrow_mut() = dest_starting_lamports.checked_add(pda_account.lamports()).unwrap();
    **pda_account.lamports.borrow_mut() = 0;

    return Ok(());
  }

  let (calc_lottery, _) = Pubkey::find_program_address(
    &[LOTTERY_NFT.as_bytes(), program_id.as_ref(), payer.key.as_ref()], &program_id
  );

  if calc_lottery == *pda_account.key {
    let lottery = NftLottery::try_from_slice(&pda_account.data.borrow())?;
    if lottery.owner != *payer.key { return Err(NftError::NotDrawn.into()); }
    
    let mut wins: u64 = 0;
    for i in 0..lottery.lots.len() {
      wins += lottery.lots[i].wins * lottery.lots[i].collect.len() as u64;
    }

    if wins != lottery.create_box { return Err(NftError::NotDrawn.into()); }

    msg!("Close lottery account");
    let dest_starting_lamports = profit_id.lamports();
    **profit_id.lamports.borrow_mut() = dest_starting_lamports.checked_add(pda_account.lamports()).unwrap();
    **pda_account.lamports.borrow_mut() = 0;

    return Ok(());
  }
  
  return Err(NftError::WrongOwnerNFR.into());
}