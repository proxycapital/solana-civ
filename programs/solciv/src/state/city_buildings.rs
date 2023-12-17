use crate::state::{TechnologyType, UnitType};
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct City {
    pub city_id: u32,
    pub name: String,
    pub player: Pubkey,
    pub game: Pubkey,
    pub x: u8,
    pub y: u8,
    pub health: u32,
    pub wall_health: u32,
    pub attack: u32,
    pub population: u32,
    pub gold_yield: u32,
    pub food_yield: u32,
    pub production_yield: u32,
    pub science_yield: u32,
    pub buildings: Vec<BuildingType>,
    pub production_queue: Vec<ProductionItem>,
    pub accumulated_production: u32,
    pub accumulated_food: i32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum ProductionItem {
    Unit(UnitType),
    Building(BuildingType),
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum BuildingType {
    Barracks,
    Wall,
    WallMedieval,
    WallRenaissance,
    WallIndustrial,
    Library,
    School,
    University,
    Observatory,
    Forge,
    Factory,
    EnergyPlant,
    Market,
    Bank,
    StockExchange,
    Granary,
    Mill,
    Bakery,
    Supermarket,
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
            wall_health: 0,
            attack: 0,
            population: 1,
            gold_yield: 2,
            food_yield: 2,
            production_yield: 2,
            science_yield: 1,
            buildings: vec![],
            production_queue: vec![],
            accumulated_production: 0,
            accumulated_food: 0,
        }
    }

    pub fn construct_building(&mut self, building_type: BuildingType) -> Result<()> {
        match building_type {
            BuildingType::Barracks => self.attack += 2,
            BuildingType::Wall => {
                self.attack += 5;
                self.wall_health = 50;
            }
            BuildingType::WallMedieval => {
                self.attack += 5;
                self.wall_health = 100;
            }
            BuildingType::WallRenaissance => {
                self.attack += 10;
                self.wall_health = 150;
            }
            BuildingType::WallIndustrial => {
                self.attack += 10;
                self.wall_health = 200;
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
    /// returns `(base_production_cost, base_gold_cost)`
    pub fn get_base_stats(building_type: BuildingType) -> (u32, u32) {
        match building_type {
            BuildingType::Barracks => (6, 100),
            BuildingType::Wall => (10, 100),
            BuildingType::WallMedieval => (16, 200),
            BuildingType::WallRenaissance => (20, 250),
            BuildingType::WallIndustrial => (28, 300),
            BuildingType::Library => (10, 100),
            BuildingType::School => (20, 150),
            BuildingType::University => (30, 200),
            BuildingType::Observatory => (40, 300),
            BuildingType::Forge => (10, 100),
            BuildingType::Factory => (20, 200),
            BuildingType::EnergyPlant => (30, 300),
            BuildingType::Market => (10, 100),
            BuildingType::Bank => (20, 200),
            BuildingType::StockExchange => (30, 300),
            BuildingType::Granary => (10, 100),
            BuildingType::Mill => (20, 200),
            BuildingType::Bakery => (30, 300),
            BuildingType::Supermarket => (40, 400),
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
