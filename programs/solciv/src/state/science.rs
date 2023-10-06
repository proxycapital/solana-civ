use anchor_lang::prelude::*;

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