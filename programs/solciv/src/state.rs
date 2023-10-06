use crate::errors::*;
use anchor_lang::prelude::*;

#[account]
pub struct Game {
    pub player: Pubkey,
    pub npc: Pubkey,
    pub turn: u32,
    pub defeat: bool,
    pub victory: bool,
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
    pub researched_technologies: Vec<TechnologyType>,
    pub current_research: Option<TechnologyType>,
    pub research_accumulated_points: u32,
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

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub enum TechnologyType {
    Archery,
    IronWorking,
    MedievalWarfare,
    Gunpowder,
    Ballistics,
    TanksAndArmor,
    Writing,
    Education,
    Economics,
    Academia,
    Astronomy,
    Capitalism,
    Agriculture,
    Construction,
    Industrialization,
    ElectricalPower,
    ModernFarming,
}

impl Player {
    pub fn update_resources(
        &mut self,
        gold: u32,
        food: u32,
        wood: u32,
        stone: u32,
        iron: u32,
    ) -> Result<()> {
        self.resources.gold = self.resources.gold.checked_add(gold).unwrap_or(u32::MAX);
        self.resources.food = self.resources.food.checked_add(food).unwrap_or(u32::MAX);
        self.resources.wood = self.resources.wood.checked_add(wood).unwrap_or(u32::MAX);
        self.resources.stone = self.resources.stone.checked_add(stone).unwrap_or(u32::MAX);
        self.resources.iron = self.resources.iron.checked_add(iron).unwrap_or(u32::MAX);
        Ok(())
    }

    pub fn start_research(&mut self, technology: TechnologyType) -> Result<()> {
        // Ensure player isn't already researching something.
        if self.current_research.is_some() {
            return err!(ResearchError::AlreadyResearching);
        }

        // Check if the technology can be researched.
        if !self.can_research(&technology) {
            return err!(ResearchError::CannotResearch);
        }

        self.current_research = Some(technology);
        self.research_accumulated_points = 0;
        Ok(())
    }

    pub fn add_research_points(&mut self, points: u32) -> Result<()> {
        if self.current_research.is_some() {
            self.research_accumulated_points += points;
        }
        let _ = self.complete_research();
        Ok(())
    }

    pub fn complete_research(&mut self) -> Result<()> {
        if let Some(technology) = &self.current_research {
            if self.research_accumulated_points >= TechnologyType::get_cost(&technology) {
                self.researched_technologies.push(technology.clone());
                self.current_research = None;
                self.research_accumulated_points = 0;
            }
        }
        Ok(())
    }

    pub fn has_researched(&self, tech: &TechnologyType) -> bool {
        self.researched_technologies.contains(tech)
    }

    pub fn can_research(&self, tech: &TechnologyType) -> bool {
        let prev_tech = match tech {
            TechnologyType::Archery => return true,
            TechnologyType::IronWorking => TechnologyType::Archery,
            TechnologyType::MedievalWarfare => TechnologyType::IronWorking,
            TechnologyType::Gunpowder => TechnologyType::MedievalWarfare,
            TechnologyType::Ballistics => TechnologyType::Gunpowder,
            TechnologyType::TanksAndArmor => TechnologyType::Ballistics,
            TechnologyType::Writing => return true,
            TechnologyType::Education => TechnologyType::Writing,
            TechnologyType::Economics => TechnologyType::Education,
            TechnologyType::Academia => TechnologyType::Economics,
            TechnologyType::Astronomy => TechnologyType::Academia,
            TechnologyType::Capitalism => TechnologyType::Astronomy,
            TechnologyType::Agriculture => return true,
            TechnologyType::Construction => TechnologyType::Agriculture,
            TechnologyType::Industrialization => TechnologyType::Construction,
            TechnologyType::ElectricalPower => TechnologyType::Industrialization,
            TechnologyType::ModernFarming => TechnologyType::ElectricalPower,
        };
        self.has_researched(&prev_tech)
    }
}

impl TechnologyType {
    pub fn get_cost(tech_type: &TechnologyType) -> u32 {
        match tech_type {
            TechnologyType::Archery => 15,
            TechnologyType::IronWorking => 21,
            TechnologyType::MedievalWarfare => 30,
            TechnologyType::Gunpowder => 42,
            TechnologyType::Ballistics => 60,
            TechnologyType::TanksAndArmor => 80,
            TechnologyType::Writing => 5,
            TechnologyType::Education => 7,
            TechnologyType::Economics => 10,
            TechnologyType::Academia => 14,
            TechnologyType::Astronomy => 18,
            TechnologyType::Capitalism => 22,
            TechnologyType::Agriculture => 6,
            TechnologyType::Construction => 8,
            TechnologyType::Industrialization => 12,
            TechnologyType::ElectricalPower => 16,
            TechnologyType::ModernFarming => 20,
        }
    }
}

impl City {
    pub fn new(
        city_id: u32,
        player: Pubkey,
        game: Pubkey,
        x: u8,
        y: u8,
        name: String,
        health: u32,
    ) -> Self {
        Self {
            city_id,
            name,
            player,
            game,
            x,
            y,
            health,
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
                self.attack += 5;
            }
            BuildingType::WallMedieval => {
                self.attack += 5;
            }
            BuildingType::WallRenaissance => {
                self.attack += 10;
            }
            BuildingType::WallIndustrial => {
                self.attack += 10;
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

    pub fn get_gold_cost(building_type: BuildingType) -> u32 {
        BuildingType::get_base_stats(building_type).1
    }

    pub fn can_construct(&self, researched_technologies: &[TechnologyType]) -> bool {
        match self {
            BuildingType::Barracks | BuildingType::Wall => true,
            BuildingType::WallMedieval => {
                researched_technologies.contains(&TechnologyType::MedievalWarfare)
            }
            BuildingType::WallRenaissance => {
                researched_technologies.contains(&TechnologyType::Gunpowder)
            }
            BuildingType::WallIndustrial => {
                researched_technologies.contains(&TechnologyType::TanksAndArmor)
            }
            BuildingType::Library => researched_technologies.contains(&TechnologyType::Writing),
            BuildingType::School => researched_technologies.contains(&TechnologyType::Education),
            BuildingType::University => researched_technologies.contains(&TechnologyType::Academia),
            BuildingType::Observatory => {
                researched_technologies.contains(&TechnologyType::Astronomy)
            }
            BuildingType::Bank | BuildingType::Market => {
                researched_technologies.contains(&TechnologyType::Economics)
            }
            BuildingType::StockExchange => {
                researched_technologies.contains(&TechnologyType::Capitalism)
            }
            BuildingType::Forge => researched_technologies.contains(&TechnologyType::IronWorking),
            BuildingType::Granary | BuildingType::Mill => {
                researched_technologies.contains(&TechnologyType::Agriculture)
            }
            BuildingType::Bakery => researched_technologies.contains(&TechnologyType::Construction),
            BuildingType::Factory => {
                researched_technologies.contains(&TechnologyType::Industrialization)
            }
            BuildingType::EnergyPlant => {
                researched_technologies.contains(&TechnologyType::ElectricalPower)
            }
            BuildingType::Supermarket => {
                researched_technologies.contains(&TechnologyType::ModernFarming)
            }
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
            UnitType::Settler => (false, 100, 0, 2, 1, 20, 0, 40),
            UnitType::Builder => (false, 100, 0, 2, 1, 20, 200, 0),
            UnitType::Warrior => (false, 100, 8, 2, 0, 20, 240, 0),
            UnitType::Archer => (true, 100, 10, 2, 0, 20, 240, 0),
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

    pub fn get_base_movement_range(unit_type: UnitType) -> u8 {
        Unit::get_base_stats(unit_type).3
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
        let given_damage_raw =
            30.0 * e.powf((self.attack as f32 - defender.attack as f32) / 25.0) * multiplier
                - 10.0 * (100.0 - self.health as f32) / 100.0;

        let taken_damage_raw = 30.0
            * e.powf((defender.attack as f32 - self.attack as f32) / 25.0)
            * taken_damage_multiplier
            - 10.0 * (100.0 - defender.health as f32) / 100.0;

        let given_damage = (given_damage_raw.max(0.0).min(255.0)) as u8;
        let taken_damage = (taken_damage_raw.max(0.0).min(255.0)) as u8;

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
        let taken_damage_multiplier: f32 = 1.0 / multiplier;
        let given_damage =
            (15.0 * e.powf((self.attack as f32 - city_defense as f32) / 25.0) * multiplier) as u8;
        let taken_damage = (15.0
            * e.powf((city_defense as f32 - self.attack as f32) / 25.0)
            * taken_damage_multiplier) as u8;

        msg!("Given damage to the city: {}", given_damage);
        msg!("Taken damage from the city: {}", given_damage);

        if u32::from(given_damage) >= city.health {
            city.health = 0;
            msg!("City has been destroyed");
        } else {
            city.health -= u32::from(given_damage);
            msg!("City HP after attack: {}", city.health);
        }

        if taken_damage >= self.health {
            self.is_alive = false;
            self.health = 0;
            msg!("Attacker is dead");
        } else {
            self.health -= taken_damage;
            msg!("Attacker HP after attack: {}", self.health);
        }

        // After the attack, the unit cannot move or attack anymore.
        self.movement_range = 0;

        Ok(())
    }
}

impl UnitType {
    pub fn can_recruit(&self, researched_technologies: &[TechnologyType]) -> bool {
        match self {
            UnitType::Settler | UnitType::Builder | UnitType::Warrior => true, // No tech required
            UnitType::Archer => researched_technologies.contains(&TechnologyType::Archery),
            UnitType::Swordsman => researched_technologies.contains(&TechnologyType::IronWorking),
            UnitType::Crossbowman => {
                researched_technologies.contains(&TechnologyType::MedievalWarfare)
            }
            UnitType::Musketman => researched_technologies.contains(&TechnologyType::Gunpowder),
            UnitType::Rifleman => researched_technologies.contains(&TechnologyType::Ballistics),
            UnitType::Tank => researched_technologies.contains(&TechnologyType::TanksAndArmor),
        }
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
