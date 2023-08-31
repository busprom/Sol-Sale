use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    pubkey::Pubkey
};

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct CreateMetadataArgs {
    pub metadata: Metadata,
    pub kind: String,
    pub metaplex: u8
}

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct Metadata {
    pub instruction: u8,
    pub data: Data,
    pub is_mutable: u8
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Data {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: u16,
    pub creators: Option<Vec<Creator>>
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Creator {
    pub address: Pubkey,
    pub verified: bool,
    pub share: u8,
}