use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_pack::{IsInitialized, Sealed},
    // pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Unit {
    pub id: u32,
    pub unit_type: UnitType,
    pub health: u32,
    pub attack: u32,
    pub movement_range: u8,
    pub visibility_range: u8, // for the "fog of war"
    pub remaining_actions: u8, // e.g. builder can move and build in the same turn, but number of total builds is limited per his lifetime
    pub position: Position,
    pub is_alive: bool, // not sure if needed, but unit is not necessary dead, should be removed also when remaining_actions == 0 (settler, builder)
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum UnitType {
    Settler,
    Builder,
    Warrior,
    Swordsman,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct UnitSeq {
    pub next_unit_id: u32, // to be used as a unique id for each new unit
    // @todo: add game session identifier ?
}

// @TODO: merge player state with balances etc?
// #[derive(BorshSerialize, BorshDeserialize)]
// pub struct PlayerState {
//     pub is_initialized: bool,
//     pub units: Vec<Unit>,
// }

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