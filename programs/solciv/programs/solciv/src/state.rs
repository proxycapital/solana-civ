use anchor_lang::prelude::*;

#[account]
pub struct Game {
    pub player: Pubkey,
    pub turn: u32,
    pub map: [u8; 400],
}

#[account]
pub struct Player {
    pub game: Pubkey,
    pub player: Pubkey,
    pub points: u32,
    pub cities: Vec<City>,
    pub units: Vec<Unit>,
    pub resources: Resources,
    pub next_city_id: u32,
    pub next_unit_id: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone)]
pub struct Resources {
    pub gold: u32,
    pub food: u32,
    pub wood: u32,
    pub stone: u32,
    pub iron: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct City {
    pub city_id: u32,
    pub player: Pubkey,
    pub game: Pubkey,
    pub x: u8,
    pub y: u8,
    pub health: u32,
    pub defence: u32,
    pub population: u32,
    pub gold_yield: u32,
    pub food_yield: u32,
    pub production_yield: u32,
    pub science_yield: u32,
    pub buildings: Vec<BuildingType>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum BuildingType {
    Barracks,
    Wall,
    Market,
    Library,
    School,
    University,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone)]
pub struct Unit {
    pub unit_id: u32,
    pub player: Pubkey,
    pub game: Pubkey,
    pub unit_type: UnitType,
    pub x: u8,
    pub y: u8,
    pub attack: u8,
    pub health: u8,
    pub movement_range: u8,
    pub remaining_actions: u8,
    pub is_alive: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq)]
pub enum UnitType {
    Settler,
    Builder,
    Warrior,
    Archer,
    Swordsman,
}

impl City {
  pub fn new(city_id: u32, player: Pubkey, game: Pubkey, x: u8, y: u8) -> Self {
      Self {
          city_id,
          player,
          game,
          x,
          y,
          health: 100,
          defence: 0,
          population: 1,
          gold_yield: 2,
          food_yield: 2,
          production_yield: 2,
          science_yield: 1,
          buildings: vec![],
      }
  }
}


impl Unit {
  pub fn new(unit_id: u32, player: Pubkey, game: Pubkey, unit_type: UnitType, x: u8, y: u8) -> Self {
      let (health, attack, movement_range, remaining_actions) = Self::get_base_stats(unit_type);
      
      Self {
          unit_id,
          player,
          game,
          unit_type,
          x,
          y,
          attack,
          health,
          movement_range,
          remaining_actions,
          is_alive: true,
      }
  }

  /// Returns the base stats of a given `UnitType`.
  /// 
  /// ### Arguments
  /// 
  /// * `unit_type` - A `UnitType` enum variant representing the type of unit.
  /// 
  /// ### Returns
  /// 
  /// A tuple containing four `u8` values representing the base stats of the unit in the following order:
  /// `(health, attack, movement_range, remaining_actions)`.
  fn get_base_stats(unit_type: UnitType) -> (u8, u8, u8, u8) {
      match unit_type {
          UnitType::Settler => (100, 0, 2, 1),
          UnitType::Builder => (100, 0, 2, 1),
          UnitType::Warrior => (100, 20, 2, 0),
          UnitType::Archer => (100, 30, 2, 0),
          UnitType::Swordsman => (100, 50, 2, 0),
      }
  }
}