use crate::errors::*;
use anchor_lang::prelude::*;

#[account]
pub struct Game {
    pub player: Pubkey,
    pub npc: Pubkey,
    pub turn: u32,
    pub map: [u8; 400],
}

#[account]
pub struct Player {
    pub game: Pubkey,
    pub player: Pubkey,
    pub points: u32,
    pub cities: Vec<City>,
    pub tiles: Vec<Tile>,
    pub units: Vec<Unit>,
    pub resources: Resources,
    pub next_city_id: u32,
    pub next_unit_id: u32,
}

#[account]
pub struct Npc {
    pub game: Pubkey,
    pub cities: Vec<City>,
    pub units: Vec<Unit>,
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
    pub fn new(
        unit_id: u32,
        player: Pubkey,
        game: Pubkey,
        unit_type: UnitType,
        x: u8,
        y: u8,
    ) -> Self {
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

    pub fn perform_attack(&mut self, defender: &mut Unit) -> Result<()> {
        // Check if the attacker is alive and of attacking type
        if !self.is_alive
            || !matches!(
                self.unit_type,
                UnitType::Warrior | UnitType::Archer | UnitType::Swordsman
            )
        {
            return err!(UnitError::InvalidAttack);
        }

        // Check if defender is of neutral type (Settler or Builder)
        if matches!(defender.unit_type, UnitType::Settler | UnitType::Builder) {
            defender.is_alive = false;
            defender.health = 0;
            msg!("Defender is dead");
            // set movement range to 0 so that the attacker cannot move or attack anymore
            self.movement_range = 0;
            return Ok(());
        }
        // Calculate given damage and taken damage by a formula:
        // damage = 30 * e^((difference between combat strengths) / 25) * random_factor
        let e: f32 = std::f32::consts::E;
        // get the unix timestamp modulo 10 to get a number in the range [0, 9]
        let clock = Clock::get()?;
        let random_factor = clock.unix_timestamp % 10;

        // map this to a range of [0.9, ~1.1007]
        let multiplier: f32 = 0.9 + ((random_factor as f32) * 0.0223);
        // @todo: do we really need the multiplier for the taken damage?
        let taken_damage_multiplier: f32 = 1.0 / multiplier;
        let given_damage = (30.0
            * e.powf((self.attack as f32 - defender.attack as f32) / 25.0)
            * multiplier) as u8;
        let taken_damage = (30.0
            * e.powf((defender.attack as f32 - self.attack as f32) / 25.0)
            * taken_damage_multiplier) as u8;
        msg!("Given damage: {}", given_damage);
        msg!("Taken damage: {}", taken_damage);
        // Deduct defender's health by the given damage
        if given_damage >= defender.health {
            defender.is_alive = false;
            defender.health = 0;
            msg!("Defender is dead");
        } else {
            defender.health -= given_damage;
            msg!("Defender HP after attack: {}", defender.health);
        }

        // Deduct attacker's health by the taken damage
        if taken_damage >= self.health {
            self.is_alive = false;
            self.health = 0;
            msg!("Attacker is dead");
        } else {
            self.health -= taken_damage;
            msg!("Attacker HP after attack: {}", self.health);
        }
        // after the attack unit cannot move or attack anymore
        self.movement_range = 0;

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pub x: u8,
    pub y: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum TileType {
    TimberCamp,
    StoneQuarry,
    CornField,
}

impl Tile {
    pub fn new(tile_type: TileType, x: u8, y: u8) -> Self {
        Self { tile_type, x, y }
    }
}
