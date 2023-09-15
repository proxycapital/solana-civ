use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_pack::{IsInitialized, Sealed},
    // pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct GameAccountState {
    pub is_initialized: bool,
    pub map: [u8; 400],
}

impl GameAccountState {
    pub const DISCRIMINATOR: &'static str = "game";

    pub fn get_account_size() -> usize {
        return 1 + 400;
    }
}

impl Sealed for GameAccountState {}

impl IsInitialized for GameAccountState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Balances {
    pub is_initialized: bool,
    pub gold: u64,
    pub food: u64,
    pub lumber: u64,
}

impl Balances {
    pub const DISCRIMINATOR: &'static str = "balances";
    pub const SIZE: usize = 1 + 8 + 8 + 8;
}

impl Sealed for Balances {}