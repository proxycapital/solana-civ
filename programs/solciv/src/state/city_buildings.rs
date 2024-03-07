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
    pub housing: u32,
    pub controlled_tiles: Vec<TileCoordinate>,
    pub level: u32,
    pub growth_points: u32,
    pub on_coast: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileCoordinate {
    pub x: u8,
    pub y: u8,
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
    ResidentialComplex,
    Lighthouse,
    Shipyard,
    SeaPort,
}

pub struct NewCityParams {
    pub city_id: u32,
    pub player: Pubkey,
    pub game: Pubkey,
    pub x: u8,
    pub y: u8,
    pub name: String,
    pub health: u32,
    pub controlled_tiles: Vec<TileCoordinate>,
    pub on_coast: bool,
}

impl City {
    pub fn new(params: NewCityParams) -> Self {
        Self {
            city_id: params.city_id,
            name: params.name,
            player: params.player,
            game: params.game,
            x: params.x,
            y: params.y,
            health: params.health,
            controlled_tiles: params.controlled_tiles,
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
            housing: 4,
            level: 0,
            growth_points: 0,
            on_coast: params.on_coast,
        }
    }

    pub fn controls_tile(&self, tile_x: u8, tile_y: u8) -> bool {
        self.controlled_tiles
            .iter()
            .any(|tile| tile.x == tile_x && tile.y == tile_y)
    }

    pub fn construct_building(&mut self, building_type: BuildingType) -> Result<()> {
        match building_type {
            BuildingType::Barracks => {
                self.attack += 2;
                self.housing += 1;
            }
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
            BuildingType::University => {
                self.science_yield += 4;
                self.housing += 1;
            }
            BuildingType::Observatory => self.science_yield += 5,
            BuildingType::Forge => self.production_yield += 2,
            BuildingType::Factory => self.production_yield += 3,
            BuildingType::EnergyPlant => self.production_yield += 4,
            BuildingType::Market => self.gold_yield += 2,
            BuildingType::Bank => self.gold_yield += 3,
            BuildingType::StockExchange => self.gold_yield += 4,
            BuildingType::Granary => {
                self.food_yield += 2;
                self.housing += 2;
            }
            BuildingType::Mill => self.food_yield += 2,
            BuildingType::Bakery => self.food_yield += 3,
            BuildingType::Supermarket => self.food_yield += 4,
            BuildingType::ResidentialComplex => self.housing += 5,
            BuildingType::Lighthouse => {
                self.food_yield += 1;
                self.gold_yield += 1;
            }
            BuildingType::Shipyard => {
                self.production_yield += 2;
                self.gold_yield += 1;
            }
            BuildingType::SeaPort => {
                self.gold_yield += 2;
                self.housing += 1;
            }
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
            BuildingType::ResidentialComplex => (40, 600),
            BuildingType::Lighthouse => (10, 100),
            BuildingType::Shipyard => (16, 200),
            BuildingType::SeaPort => (20, 250),
        }
    }

    pub fn get_gold_cost(building_type: BuildingType) -> u32 {
        BuildingType::get_base_stats(building_type).1
    }

    pub fn can_construct(&self, researched_technologies: &[TechnologyType], on_coast: bool) -> bool {
        match self {
            BuildingType::Lighthouse => {
                researched_technologies.contains(&TechnologyType::MaritimeNavigation) && on_coast
            }
            BuildingType::Shipyard => {
                researched_technologies.contains(&TechnologyType::AdvancedShipbuilding) && on_coast
            }
            BuildingType::SeaPort => {
                researched_technologies.contains(&TechnologyType::OceanicTrade) && on_coast
            }
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
            BuildingType::ResidentialComplex => {
                researched_technologies.contains(&TechnologyType::Urbanization)
            }
        }
    }
}
