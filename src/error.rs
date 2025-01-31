use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum CommentProgramError {
  /// Invalid Instruction
  #[error("Invalid Instruction")]
  InvalidInstruction,
  
  #[error("Token mint is not compatible")]
  InvalidMint,

  #[error("Given pda is not same with the derived pda")]
  NotListed,
}

impl From<CommentProgramError> for ProgramError {
  fn from(e: CommentProgramError) -> Self {
    ProgramError::Custom(e as u32)
  }
}
