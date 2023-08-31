use borsh::{BorshDeserialize, BorshSerialize};
use crate::{
	types::{
		metadata::CreateMetadataArgs,
		lottery::Lottery,
		sell::Sell,
		nft_lottery::{NftStorage, Lot, SaveToken},
		share::ShareData
	}
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum SolInstruction {
	DelPDA,
	BurnToken,
	NftLotAdd {data: Lot},
	Withdrawal
}