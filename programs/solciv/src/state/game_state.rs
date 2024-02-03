use crate::consts::STORAGE_CAPACITY;
use crate::errors::*;
use crate::state::{City, Resources, TechnologyType, Tile, Unit};
use anchor_lang::prelude::*;

#[account]
pub struct Game {
    pub player: Pubkey,
    pub npc: Pubkey,
    pub turn: u32,
    pub defeat: bool,
    pub victory: bool,
    pub map: [Terrain; 400],
    pub difficulty_level: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Terrain {
    pub terrain: u8,
    pub discovered: bool,
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

impl Player {
    pub fn update_resources(
        &mut self,
        gold: i32,
        wood: u32,
        stone: u32,
        iron: u32,
        horses: u32,
    ) -> Result<()> {
        self.resources.gold = self.resources.gold.checked_add(gold).unwrap_or({
            if gold > 0 {
                i32::MAX
            } else {
                i32::MIN
            }
        });

        let add_resource = |current: u32, addition: u32| -> u32 {
            std::cmp::min(current.saturating_add(addition), STORAGE_CAPACITY.into())
        };

        self.resources.wood = add_resource(self.resources.wood, wood);
        self.resources.stone = add_resource(self.resources.stone, stone);
        self.resources.iron = add_resource(self.resources.iron, iron);
        self.resources.horses = add_resource(self.resources.horses, horses);

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
            if self.research_accumulated_points >= TechnologyType::get_cost(technology) {
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
            TechnologyType::AnimalHusbandry
            | TechnologyType::Writing
            | TechnologyType::Agriculture => return true,
            TechnologyType::Archery => TechnologyType::AnimalHusbandry,
            TechnologyType::HorsebackRiding => TechnologyType::Archery,
            TechnologyType::IronWorking => TechnologyType::HorsebackRiding,
            TechnologyType::MedievalWarfare => TechnologyType::IronWorking,
            TechnologyType::Gunpowder => TechnologyType::MedievalWarfare,
            TechnologyType::Ballistics => TechnologyType::Gunpowder,
            TechnologyType::TanksAndArmor => TechnologyType::Ballistics,
            TechnologyType::Education => TechnologyType::Writing,
            TechnologyType::Economics => TechnologyType::Education,
            TechnologyType::Academia => TechnologyType::Economics,
            TechnologyType::Astronomy => TechnologyType::Academia,
            TechnologyType::Capitalism => TechnologyType::Astronomy,
            TechnologyType::Construction => TechnologyType::Agriculture,
            TechnologyType::Industrialization => TechnologyType::Construction,
            TechnologyType::ElectricalPower => TechnologyType::Industrialization,
            TechnologyType::ModernFarming => TechnologyType::ElectricalPower,
            TechnologyType::Urbanization => TechnologyType::ModernFarming,
        };
        self.has_researched(&prev_tech)
    }
}
