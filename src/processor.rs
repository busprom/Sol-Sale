use borsh::BorshDeserialize;
use solana_program::{
	pubkey::Pubkey,
	entrypoint::ProgramResult,
	account_info::{next_account_info, AccountInfo}
};
use crate::{
	utils::{
		lottery_create::process_create_lottery,
		sell_create_storage::process_sell_create_storage,
		sell_withdrawal::process_sell_withdrawal,
		lottery_check::process_check_lottery,
		del_pda::process_del_pda,
		save_token::process_save_token,
		create_share::process_create_share,
		withdrawal::process_withdrawal,
		wd::process_wd
	},
	nft_lottery::{
		nft_collection_create::process_nft_collection_create,
		nft_lottery_get_token::process_nft_lottery_get_token,
		nft_lottery_check::process_nft_lottery_check,
		nft_add_lot::process_nft_add_lot
	},
	token::{
		create_mint::process_create_mint,
		burn_token::process_burn_token
	},
	instruction::SolInstruction
};
pub struct Processor;

impl Processor {
	pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
		let instruction = SolInstruction::try_from_slice(input)?;
		match instruction {
			SolInstruction::CreateOneToken {data} => {
				let account_info_iter = &mut accounts.iter();
				let payer = next_account_info(account_info_iter)?;
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
				process_create_mint(
					program_id,
					payer,
					mint,
					mint_account,
					metadata_account,
					metadata_program,
					profit_id,
					storage,
					mint_storage,
					token_program,
					spl_token_program,
					rent_program,
					system_program,
					data
				)
			},
			SolInstruction::SellCreateStorage {sell} => {
				let account_info_iter = &mut accounts.iter();
				let payer = next_account_info(account_info_iter)?;
				let token_account = next_account_info(account_info_iter)?;
				let mint = next_account_info(account_info_iter)?;
				let vault = next_account_info(account_info_iter)?;
				let storage = next_account_info(account_info_iter)?;
				let token_program = next_account_info(account_info_iter)?;
				let rent_info = next_account_info(account_info_iter)?;
				let system_program_info = next_account_info(account_info_iter)?;
				process_sell_create_storage(
					program_id,
					payer,
					token_account,
					mint,
					vault,
					storage,
					token_program,
					rent_info,
					system_program_info,
					sell
				)
			},
			SolInstruction::SellWithdrawal => {
				let account_info_iter = &mut accounts.iter();
				let mint = next_account_info(account_info_iter)?;
				let seller = next_account_info(account_info_iter)?;
				let buyer = next_account_info(account_info_iter)?;
				let buyer_account = next_account_info(account_info_iter)?;
				let vault = next_account_info(account_info_iter)?;
				let storage = next_account_info(account_info_iter)?;
				let applicant = next_account_info(account_info_iter)?;
				let profit_id = next_account_info(account_info_iter)?;
				let token_program = next_account_info(account_info_iter)?;
				let spl_token_program = next_account_info(account_info_iter)?;
				let rent_program = next_account_info(account_info_iter)?;
				let system_program = next_account_info(account_info_iter)?;
				process_sell_withdrawal(
					program_id,
					mint,
					seller,
					buyer,
					buyer_account,
					vault,
					storage,
					applicant,
					profit_id,
					token_program,
					spl_token_program,
					rent_program,
					system_program
				)
			},
			SolInstruction::CreateLottery {lots} => {
				let acc_iter = &mut accounts.iter();
				let admin_info = next_account_info(acc_iter)?;
				let settings_info = next_account_info(acc_iter)?;
				let profit_id = next_account_info(acc_iter)?;
				let system_program_info = next_account_info(acc_iter)?;
				let rent_info = next_account_info(acc_iter)?;
				process_create_lottery(
					program_id,
					admin_info,
					settings_info,
					profit_id,
					system_program_info,
					rent_info,
					lots
				)
			},
			SolInstruction::OpenSolBox => {
				let account_info_iter = &mut accounts.iter();
				let payer = next_account_info(account_info_iter)?;
				let mint = next_account_info(account_info_iter)?;
				let mint_account = next_account_info(account_info_iter)?;
				let mint_pda = next_account_info(account_info_iter)?;
				let lottery_account = next_account_info(account_info_iter)?;
				let profit_id = next_account_info(account_info_iter)?;
				let token_program = next_account_info(account_info_iter)?;
				process_check_lottery(
					program_id,
					payer,
					mint,
					mint_account,
					mint_pda,
					lottery_account,
					profit_id,
					token_program
				)
			},
			SolInstruction::NftCreateCollection {data} => {
				let account_info_iter = &mut accounts.iter();
				let payer = next_account_info(account_info_iter)?;
				let storage = next_account_info(account_info_iter)?;
				let rent_program = next_account_info(account_info_iter)?;
				let system_program = next_account_info(account_info_iter)?;
				process_nft_collection_create(
					program_id,
					payer,
					storage,
					rent_program,
					system_program,
					data
				)
			},
			SolInstruction::NftLotAdd {data} => {
				let account_info_iter = &mut accounts.iter();
				let payer = next_account_info(account_info_iter)?;
				let storage = next_account_info(account_info_iter)?;
				let profit_id = next_account_info(account_info_iter)?;
				let rent_program = next_account_info(account_info_iter)?;
				let system_program = next_account_info(account_info_iter)?;
				process_nft_add_lot(
					program_id,
					payer,
					storage,
					profit_id,
					rent_program,
					system_program,
					data
				)
			},
			SolInstruction::NftLotteryGetToken {data} => {
				let account_info_iter = &mut accounts.iter();
				let payer = next_account_info(account_info_iter)?;
				let box_mint = next_account_info(account_info_iter)?;
				let box_mint_account = next_account_info(account_info_iter)?;
				let box_mint_storage = next_account_info(account_info_iter)?;
				let new_mint = next_account_info(account_info_iter)?;
				let new_mint_account = next_account_info(account_info_iter)?;
				let new_mint_storage = next_account_info(account_info_iter)?;
				let metadata_account = next_account_info(account_info_iter)?;
				let metadata_program = next_account_info(account_info_iter)?;
				let storage = next_account_info(account_info_iter)?;
				let profit_id = next_account_info(account_info_iter)?;
				let token_program = next_account_info(account_info_iter)?;
				let spl_token_program = next_account_info(account_info_iter)?;
				let rent_program = next_account_info(account_info_iter)?;
				let system_program = next_account_info(account_info_iter)?;
				process_nft_lottery_get_token(
					program_id,
					payer,
					box_mint,
					box_mint_account,
					box_mint_storage,
					new_mint,
					new_mint_account,
					new_mint_storage,
					metadata_account,
					metadata_program,
					storage,
					profit_id,
					token_program,
					spl_token_program,
					rent_program,
					system_program,
					data
				)
			},
			SolInstruction::DelPDA => {
				let account_info_iter = &mut accounts.iter();
				let payer = next_account_info(account_info_iter)?;
				let pda_account = next_account_info(account_info_iter)?;
				let profit_id = next_account_info(account_info_iter)?;
				process_del_pda(
					program_id,
					payer,
					pda_account,
					profit_id
				)
			},
			SolInstruction::BurnToken => {
				let account_info_iter = &mut accounts.iter();
				let owner = next_account_info(account_info_iter)?;
				let mint = next_account_info(account_info_iter)?;
				let mint_account = next_account_info(account_info_iter)?;
				let profit_id = next_account_info(account_info_iter)?;
				let token_program = next_account_info(account_info_iter)?;
				process_burn_token(
					owner,
					mint,
					mint_account,
					profit_id,
					token_program
				)
			},
			SolInstruction::NftLotteryCheck => {
				process_nft_lottery_check(
					accounts
				)
			},
			SolInstruction::SaveToken {data} => {
				process_save_token(
					program_id,
					accounts,
					data
				)
			},
			SolInstruction::CreateShare {data} => {
				let account_info_iter = &mut accounts.iter();
				let payer = next_account_info(account_info_iter)?;
				let share_storage = next_account_info(account_info_iter)?;
  			let one_storage = next_account_info(account_info_iter)?;
				let mint = next_account_info(account_info_iter)?;
				let mint_account = next_account_info(account_info_iter)?;
				let metadata_account = next_account_info(account_info_iter)?;
				let metadata_program = next_account_info(account_info_iter)?;
				let profit_id = next_account_info(account_info_iter)?;
				let token_program = next_account_info(account_info_iter)?;
				let spl_token_program = next_account_info(account_info_iter)?;
				let rent_program = next_account_info(account_info_iter)?;
				let system_program = next_account_info(account_info_iter)?;
				process_create_share(
					program_id,
					payer,
					share_storage,
  				one_storage,
					mint,
					mint_account,
					metadata_account,
					metadata_program,
					profit_id,
					token_program,
					spl_token_program,
					rent_program,
					system_program,
					data
				)
			},
			SolInstruction::Withdrawal => {
				let account_info_iter = &mut accounts.iter();
				let payer = next_account_info(account_info_iter)?;
				let share_storage = next_account_info(account_info_iter)?;
  			let one_storage = next_account_info(account_info_iter)?;
				let mint = next_account_info(account_info_iter)?;
				let mint_account = next_account_info(account_info_iter)?;
				process_withdrawal(
					program_id,
					payer,
					share_storage,
					one_storage,
					mint,
					mint_account
				)
			},
			SolInstruction::Wd => {
				let account_info_iter = &mut accounts.iter();
				let payer = next_account_info(account_info_iter)?;
				let share_storage = next_account_info(account_info_iter)?;
				process_wd(
					payer,
					share_storage
				)
			}
		}
	}
}