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
    pub player: Pubkey,
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
    pub name: String,
    pub player: Pubkey,
    pub game: Pubkey,
    pub x: u8,
    pub y: u8,
    pub health: u32,
    pub attack: u32,
    pub population: u32,
    pub gold_yield: u32,
    pub food_yield: u32,
    pub production_yield: u32,
    pub science_yield: u32,
    pub buildings: Vec<BuildingType>,
    pub production_queue: Vec<ProductionItem>,
    pub accumulated_production: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum ProductionItem {
    Unit(UnitType),
    Building(BuildingType),
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum BuildingType {
    Barracks,        // defense + units
    Wall,            // defense
    WallMedieval,    // defense
    WallRenaissance, // defense
    WallIndustrial,  // defense
    Library,         // science
    School,          // science
    University,      // science
    Observatory,     // science
    Forge,           // production
    Factory,         // production
    EnergyPlant,     // prooduction
    Market,          // gold
    Bank,            // gold
    StockExchange,   // gold
    Granary,         // food
    Mill,            // food
    Bakery,          // food
    Supermarket,     // food
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
    pub base_production_cost: u32,
    pub base_gold_cost: u32,
    pub base_resource_cost: u32,
    pub is_ranged: bool,
    pub is_alive: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq)]
pub enum UnitType {
    Settler,
    Builder,
    Warrior,
    Archer,
    Swordsman,
    Crossbowman,
    Musketman,
    Rifleman,
    Tank,
}

impl Player {
    pub fn update_resources(&mut self, gold: u32, food: u32, wood: u32, stone: u32, iron: u32) -> Result<()> {
        self.resources.gold = self.resources.gold.checked_add(gold).unwrap_or(u32::MAX);
        self.resources.food = self.resources.food.checked_add(food).unwrap_or(u32::MAX);
        self.resources.wood = self.resources.wood.checked_add(wood).unwrap_or(u32::MAX);
        self.resources.stone = self.resources.stone.checked_add(stone).unwrap_or(u32::MAX);
        self.resources.iron = self.resources.iron.checked_add(iron).unwrap_or(u32::MAX);
        Ok(())
    }
}

impl City {
    pub fn new(city_id: u32, player: Pubkey, game: Pubkey, x: u8, y: u8, name: String) -> Self {
        Self {
            city_id,
            name,
            player,
            game,
            x,
            y,
            health: 100,
            attack: 0,
            population: 1,
            gold_yield: 2,
            food_yield: 2,
            production_yield: 2,
            science_yield: 1,
            buildings: vec![],
            production_queue: vec![],
            accumulated_production: 0,
        }
    }

    pub fn construct_building(&mut self, building_type: BuildingType) -> Result<()> {
        match building_type {
            BuildingType::Barracks => self.attack += 2,
            BuildingType::Wall => {
                self.attack += 2;
                self.health += 25;
            }
            BuildingType::WallMedieval => {
                self.attack += 4;
                self.health += 25;
            }
            BuildingType::WallRenaissance => {
                self.attack += 4;
                self.health += 25;
            }
            BuildingType::WallIndustrial => {
                self.attack += 4;
                self.health += 25;
            }
            BuildingType::Library => self.science_yield += 2,
            BuildingType::School => self.science_yield += 3,
            BuildingType::University => self.science_yield += 4,
            BuildingType::Observatory => self.science_yield += 5,
            BuildingType::Forge => self.production_yield += 2,
            BuildingType::Factory => self.production_yield += 3,
            BuildingType::EnergyPlant => self.production_yield += 4,
            BuildingType::Market => self.gold_yield += 2,
            BuildingType::Bank => self.gold_yield += 3,
            BuildingType::StockExchange => self.gold_yield += 4,
            BuildingType::Granary => self.food_yield += 2,
            BuildingType::Mill => self.food_yield += 2,
            BuildingType::Bakery => self.food_yield += 3,
            BuildingType::Supermarket => self.food_yield += 4,
            _ => (),
        }
        self.buildings.push(building_type);

        Ok(())
    }
}

impl BuildingType {
    /// returns `(base_production_cost, base_gold_cost, required_building_type, required_technology_type)`
    pub fn get_base_stats(building_type: BuildingType) -> (u32, u32) {
        match building_type {
            // BuildingType::Barracks => (20, 20),
            // BuildingType::Wall => (20, 20),
            // BuildingType::WallMedieval => (20, 20),
            // BuildingType::WallRenaissance => (20, 20),
            // BuildingType::WallIndustrial => (20, 20),
            // BuildingType::Library => (20, 20),
            // BuildingType::School => (20, 20),
            // BuildingType::University => (20, 20),
            // BuildingType::Observatory => (20, 20),
            // BuildingType::Forge => (20, 20),
            // BuildingType::Factory => (20, 20),
            // BuildingType::EnergyPlant => (20, 20),
            // BuildingType::Market => (20, 20),
            // BuildingType::Bank => (20, 20),
            // BuildingType::StockExchange => (20, 20),
            // BuildingType::Granary => (20, 20),
            // BuildingType::Mill => (20, 20),
            // BuildingType::Bakery => (20, 20),
            // BuildingType::Supermarket => (20, 20),
            BuildingType::Barracks => (4, 4),
            BuildingType::Wall => (4, 4),
            BuildingType::WallMedieval => (4, 4),
            BuildingType::WallRenaissance => (4, 4),
            BuildingType::WallIndustrial => (4, 4),
            BuildingType::Library => (4, 4),
            BuildingType::School => (4, 4),
            BuildingType::University => (4, 4),
            BuildingType::Observatory => (4, 4),
            BuildingType::Forge => (4, 4),
            BuildingType::Factory => (4, 4),
            BuildingType::EnergyPlant => (4, 4),
            BuildingType::Market => (4, 4),
            BuildingType::Bank => (4, 4),
            BuildingType::StockExchange => (4, 4),
            BuildingType::Granary => (4, 4),
            BuildingType::Mill => (4, 4),
            BuildingType::Bakery => (4, 4),
            BuildingType::Supermarket => (4, 4),
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
        let (
            is_ranged,
            health,
            attack,
            movement_range,
            remaining_actions,
            base_production_cost,
            base_gold_cost,
            base_resource_cost,
        ) = Self::get_base_stats(unit_type);

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
            base_production_cost,
            base_gold_cost,
            base_resource_cost,
            is_ranged,
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
    /// `(is_ranged, health, attack, movement_range, remaining_actions, base_production_cost, base_gold_cost, base_resource_cost)`.
    pub fn get_base_stats(unit_type: UnitType) -> (bool, u8, u8, u8, u8, u32, u32, u32) {
        match unit_type {
            // UnitType::Settler => (false, 100, 0, 2, 1, 20, 0, 40),
            // UnitType::Builder => (false, 100, 0, 2, 1, 20, 200, 0),
            // UnitType::Warrior => (false, 100, 8, 2, 0, 20, 240, 0),
            // UnitType::Archer => (true, 100, 6, 2, 0, 20, 240, 0),
            // UnitType::Swordsman => (false, 100, 14, 2, 0, 30, 240, 10),
            // UnitType::Crossbowman => (true, 100, 24, 2, 0, 40, 300, 0),
            // UnitType::Musketman => (true, 100, 32, 2, 0, 50, 360, 0),
            // UnitType::Rifleman => (true, 100, 40, 3, 0, 60, 420, 0),
            // UnitType::Tank => (true, 100, 50, 4, 0, 80, 500, 0),
            UnitType::Settler => (false, 100, 0, 2, 1, 4, 0, 40),
            UnitType::Builder => (false, 100, 0, 2, 1, 2, 200, 0),
            UnitType::Warrior => (false, 100, 8, 2, 0, 2, 240, 0),
            UnitType::Archer => (true, 100, 6, 2, 0, 20, 240, 0),
            UnitType::Swordsman => (false, 100, 14, 2, 0, 30, 240, 10),
            UnitType::Crossbowman => (true, 100, 24, 2, 0, 40, 300, 0),
            UnitType::Musketman => (true, 100, 32, 2, 0, 50, 360, 0),
            UnitType::Rifleman => (true, 100, 40, 3, 0, 60, 420, 0),
            UnitType::Tank => (true, 100, 50, 4, 0, 80, 500, 0),
        }
    }

    pub fn get_base_cost(unit_type: UnitType) -> u32 {
        Unit::get_base_stats(unit_type).5
    }

    pub fn get_gold_cost(unit_type: UnitType) -> u32 {
        Unit::get_base_stats(unit_type).6
    }

    pub fn get_resource_cost(unit_type: UnitType) -> u32 {
        Unit::get_base_stats(unit_type).7
    }

    fn can_attack(&self) -> bool {
        // only 2 units cannot attack: Settler and Builder
        !matches!(self.unit_type, UnitType::Settler | UnitType::Builder)
    }

    pub fn attack_unit(&mut self, defender: &mut Unit) -> Result<()> {
        // Check if the attacker is alive and of attacking type
        if !self.is_alive || !self.can_attack() {
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

    pub fn attack_city(&mut self, city: &mut City) -> Result<()> {
        if !self.is_alive || !self.can_attack() {
            return err!(UnitError::InvalidAttack);
        }

        // @todo: consider more complicated defense flow based on Wall types etc, as well as taken_damage for the attacker (in case of Wall in the city)
        let city_defense = city.attack;

        // Similar damage calculations as attack_unit
        let e: f32 = std::f32::consts::E;
        let clock = Clock::get()?;
        let random_factor = clock.unix_timestamp % 10;
        let multiplier: f32 = 0.9 + ((random_factor as f32) * 0.0223);
        let given_damage =
            (30.0 * e.powf((self.attack as f32 - city_defense as f32) / 25.0) * multiplier) as u8;

        // @todo: add wall mechanics for the taken_damage
        msg!("Given damage to city: {}", given_damage);

        if u32::from(given_damage) >= city.health {
            city.health = 0;
            msg!("City has been destroyed");
        } else {
            city.health -= u32::from(given_damage);
            msg!("City HP after attack: {}", city.health);
        }

        // After the attack, the unit cannot move or attack anymore.
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
    LumberMill,
    StoneQuarry,
    Farm,
    IronMine,
}

impl Tile {
    pub fn new(tile_type: TileType, x: u8, y: u8) -> Self {
        Self { tile_type, x, y }
    }
}
