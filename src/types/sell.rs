use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Sell {
  pub applicant: Pubkey, //current payer
  pub auction: u8,
  pub bets: u64,
  pub price: u64,
  pub seller: Pubkey,
  pub start_price: u64,
  pub vault: Pubkey, //vault
}