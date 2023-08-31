use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum NftError {
    #[error("Admin signature is required")]
    AdminRequired,

    #[error("Wrong counter PDA for this user")]
    WrongCounterPDA,

    #[error("Wrong settings PDA")]
    WrongSettingsPDA,

    #[error("Wrong owner NFT")]
    WrongOwnerNFR,

    #[error("Not enough lampports")]
    WrongLamports,

    #[error("Not enough tokens")]
    WrongTokens,

    #[error("The token was used")]
    WasUsed,

    #[error("The lottery is done")]
    LotteryDone,

    #[error("The collection didn't win")]
    DontWin,

    #[error("The lottery already exists")]
    LotteryExists,

    #[error("All the boxes were manufactured")]
    AllBoxesMinted,

    #[error("Not all lots are drawn")]
    NotDrawn

}

impl From<NftError> for ProgramError {
    fn from(e: NftError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
