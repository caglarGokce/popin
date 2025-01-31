use crate::instruction::CommentProgramInstruction;
use crate::state::{CreateComment,  Comment};
use borsh::{BorshDeserialize, BorshSerialize};
use std::str::FromStr;
use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint::ProgramResult,
  pubkey::Pubkey, 
  system_instruction,program::invoke
};


pub struct Processor;
impl Processor {
  pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
  ) -> ProgramResult {
    let instruction: CommentProgramInstruction = CommentProgramInstruction::unpack(instruction_data)?;

    match instruction {

      CommentProgramInstruction::SendComment {data} => {
        Self::send_comment(accounts,program_id,data)
      }
      CommentProgramInstruction::Delete => {
        Self::delete_comment(accounts)
      }
      CommentProgramInstruction::DeleteAuth => {
        Self::delete_comment_by_authority(accounts)
      }
    }
  }


    pub fn  send_comment(
      accounts: &[AccountInfo],
      program_id:&Pubkey,
      data:CreateComment
    ) -> ProgramResult {
  
    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();
  
    let commenter: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let comment_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    if !commenter.is_signer{panic!()}

    let create_ix = system_instruction::create_account(
      commenter.key,
      comment_account.key,
      data.lamports, 
      data.space,
      program_id);

    invoke(&create_ix,&[commenter.clone(),comment_account.clone()])?;


    let comment = Comment{
      commenter:commenter.key.to_bytes(),
      comment:data.comment
    };
    
    comment.serialize(&mut &mut comment_account.data.borrow_mut()[..])?;

  
      Ok(())
    }

    pub fn  delete_comment(
      accounts: &[AccountInfo],
    ) -> ProgramResult {
  
    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let commenter: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let comment_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    if !commenter.is_signer{panic!()}

    let comment: Comment = Comment::try_from_slice(&comment_account.data.borrow())?;

    let commenter_from_bytes = Pubkey::new_from_array(comment.commenter);

    if &commenter_from_bytes != commenter.key {panic!()}

    let value = **comment_account.lamports.borrow_mut();

    **comment_account.lamports.borrow_mut()-= value;
    **commenter.lamports.borrow_mut()+= value;


      Ok(())
    }

    pub fn  delete_comment_by_authority(
      accounts: &[AccountInfo],
    ) -> ProgramResult {
  
    let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

    let authority: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let commenter: &AccountInfo<'_> = next_account_info(accounts_iter)?;
    let comment_account: &AccountInfo<'_> = next_account_info(accounts_iter)?;

    let authority_address = Pubkey::from_str("Frfz5jf4mR7QFNqrYKAMKCjRbCGycX1by6r26UmHHCoL").unwrap();
    if !authority.is_signer{panic!()}

    if authority.key != &authority_address {panic!()}

    let comment: Comment = Comment::try_from_slice(&comment_account.data.borrow())?;

    let commenter_from_bytes = Pubkey::new_from_array(comment.commenter);

    if &commenter_from_bytes != commenter.key {panic!()}

    let value = **comment_account.lamports.borrow_mut();

    **comment_account.lamports.borrow_mut()-= value;
    **commenter.lamports.borrow_mut()+= value;

      Ok(())
    }


}

