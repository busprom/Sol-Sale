pub mod utils;
pub mod processor;
pub mod error;
pub mod instruction;
pub mod types;
pub mod token;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

solana_program::declare_id!("");

pub const TOKEN_PROGRAM_ID: &str = "";
pub const PROFIT_ID: &str = "";
pub const METADATA_PROGRAM_ID: &str = "";

pub const SETTINGS_SEED: &str = "";
pub const PREFIX: &str = "";
pub const STORAGE: &str = "";

pub const LOTTERY_SEED: &str = "";
pub const LOTTERY_TOKEN: &str = "";

pub const LOTTERY_NFT: &str = "";

pub const SHARER: &str = "";
pub const SHARE_SEED: &str = "";