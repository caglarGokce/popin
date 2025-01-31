use crate::{error::CommentProgramError::InvalidInstruction, state::CreateComment};
use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

#[derive(Debug, PartialEq)]
pub enum CommentProgramInstruction {

  SendComment{data:CreateComment},
  Delete,
  DeleteAuth,
}

impl CommentProgramInstruction {
  pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {

    let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
    Ok(match tag {
      0 => Self::SendComment{
        data:CreateComment::try_from_slice(&rest)?,
      },
      1 => Self::Delete,
      2 => Self::DeleteAuth,

      _ => return Err(InvalidInstruction.into()),
    })
  }
}
