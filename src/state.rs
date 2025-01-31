use borsh::{BorshDeserialize, BorshSerialize};


#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Comment{
    pub commenter:[u8;32],
    pub comment:String

}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct CreateComment{
    pub space:u64,
    pub lamports:u64,
    pub comment:String
}