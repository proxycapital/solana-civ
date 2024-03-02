use crate::consts::*;
use crate::errors::*;
use crate::state::{City, TechnologyType};
use crate::utils::*;
use anchor_lang::prelude::*;

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
    pub level: u8,
    pub experience: u8,
    pub movement_range: u8,
    pub remaining_actions: u8,
    pub base_production_cost: u32,
    pub base_gold_cost: u32,
    pub base_resource_cost: u32,
    pub maintenance_cost: i32,
    pub is_ranged: bool,
    pub is_alive: bool,
    pub is_naval: bool,
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
    Horseman,
    Galley,
    Frigate,
    Battleship,
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
            is_naval,
            health,
            attack,
            movement_range,
            remaining_actions,
            base_production_cost,
            base_gold_cost,
            base_resource_cost,
            maintenance_cost,
            experience,
            level,
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
            experience,
            level,
            movement_range,
            remaining_actions,
            base_production_cost,
            base_gold_cost,
            base_resource_cost,
            maintenance_cost,
            is_ranged,
            is_alive: true,
            is_naval,
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
    /// A tuple containing values representing the base stats of the unit in the following order:
    /// `(is_ranged, is_naval, health, attack, movement_range, remaining_actions, base_production_cost, base_gold_cost, base_resource_cost, maintenance_cost, experience, level)`.
    pub fn get_base_stats(
        unit_type: UnitType,
    ) -> (bool, bool, u8, u8, u8, u8, u32, u32, u32, i32, u8, u8) {
        match unit_type {
            UnitType::Settler => (false, false, 100, 0, 2, 1, 20, 100, 60, 0, 0, 0),
            UnitType::Builder => (false, false, 100, 0, 2, 1, 20, 100, 0, 0, 0, 0),
            UnitType::Warrior => (false, false, 100, 8, 2, 0, 20, 200, 0, 0, 0, 0),
            UnitType::Archer => (true, false, 100, 10, 2, 0, 20, 200, 0, 1, 0, 0),
            UnitType::Swordsman => (false, false, 100, 14, 2, 0, 30, 240, 10, 1, 0, 0),
            UnitType::Horseman => (false, false, 100, 14, 3, 0, 30, 280, 10, 2, 0, 0),
            UnitType::Crossbowman => (true, false, 100, 24, 2, 0, 40, 240, 0, 2, 0, 0),
            UnitType::Musketman => (true, false, 100, 32, 2, 0, 50, 360, 0, 2, 0, 0),
            UnitType::Rifleman => (true, false, 100, 40, 3, 0, 60, 420, 0, 4, 0, 0),
            UnitType::Tank => (true, false, 100, 50, 4, 0, 80, 500, 0, 7, 0, 0),
            UnitType::Galley => (false, true, 100, 10, 3, 0, 25, 150, 0, 0, 0, 0),
            UnitType::Frigate => (false, true, 100, 14, 4, 0, 40, 280, 0, 1, 0, 0),
            UnitType::Battleship => (true,  true, 100, 24, 5, 0, 80, 420, 0, 3, 0, 0),
        }
    }

    pub fn get_base_cost(unit_type: UnitType) -> u32 {
        Unit::get_base_stats(unit_type).6
    }

    pub fn get_gold_cost(unit_type: UnitType) -> u32 {
        Unit::get_base_stats(unit_type).7
    }

    pub fn get_resource_cost(unit_type: UnitType) -> u32 {
        Unit::get_base_stats(unit_type).8
    }

    pub fn get_base_movement_range(unit_type: UnitType) -> u8 {
        Unit::get_base_stats(unit_type).4
    }

    pub fn get_maintenance_cost(unit_type: UnitType) -> i32 {
        Unit::get_base_stats(unit_type).9
    }

    pub fn get_expereince(unit_type: UnitType) -> u8 {
        Unit::get_base_stats(unit_type).10
    }

    fn can_attack(&self) -> bool {
        // only 2 units cannot attack: Settler and Builder
        !matches!(self.unit_type, UnitType::Settler | UnitType::Builder)
    }

    fn apply_damage(&mut self, damage: u8) {
        if damage >= self.health {
            self.is_alive = false;
            self.health = 0;
        } else {
            self.health -= damage;
        }
    }

    fn update_experience(&mut self, is_killer: bool) {
        let exp_gain = if is_killer {
            2 * EXP_PER_ATTACK
        } else {
            EXP_PER_ATTACK
        };
        self.experience = get_new_exp(self.level, self.experience, exp_gain);
    }

    pub fn attack_unit(
        &mut self,
        defender: &mut Unit,
        defender_behind_the_wall: Option<bool>,
    ) -> Result<()> {
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
        let mut given_damage_raw =
            30.0 * e.powf((self.attack as f32 - defender.attack as f32) / 25.0) * multiplier
                - 10.0 * (100.0 - self.health as f32) / 100.0;

        if defender_behind_the_wall.is_some() {
            // decrease given damage by 2 if defender unit behind the wall
            given_damage_raw /= 2.0;
        }

        let taken_damage_raw = 30.0
            * e.powf((defender.attack as f32 - self.attack as f32) / 25.0)
            * taken_damage_multiplier
            - 10.0 * (100.0 - defender.health as f32) / 100.0;

        let given_damage = (given_damage_raw.max(0.0).min(255.0)) as u8;
        let taken_damage = (taken_damage_raw.max(0.0).min(255.0)) as u8;

        // Apply damage to defender
        defender.apply_damage(given_damage);

        // Apply damage to self (attacker)
        self.apply_damage(taken_damage);

        // Check if both units are supposed to die
        if !self.is_alive && !defender.is_alive {
            if given_damage >= taken_damage {
                // Attacker dealt more damage, so it survives with 1 HP
                // Attacker wins also in case of a tie
                self.is_alive = true;
                self.health = 1;
                // Update experience considering attacker won
                self.update_experience(true);
            } else {
                // Defender dealt more damage, so it survives with 1 HP
                defender.is_alive = true;
                defender.health = 1;
                // Update experience considering defender won
                defender.update_experience(true);
            }
        } else {
            // If not both are dead, update experiences normally
            self.update_experience(!defender.is_alive);
            defender.update_experience(!self.is_alive);
        }

        // After the attack unit cannot move or attack anymore
        self.movement_range = 0;

        Ok(())
    }

    pub fn attack_city(&mut self, city: &mut City) -> Result<()> {
        if !self.is_alive || !self.can_attack() {
            return err!(UnitError::InvalidAttack);
        }

        // by default city don't have any defence
        let mut city_defense = 0;
        if city.wall_health != 0 {
            city_defense = city.attack;
        }

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

        if city.wall_health > 0 {
            // damage for wall decreased by 2
            let given_wall_damage = given_damage / 2;
            // handle damage for city wall
            if city.wall_health < given_wall_damage as u32 {
                let city_damage = u32::from(given_wall_damage) - city.wall_health;
                city.wall_health = 0;
                city.health -= city_damage;
                msg!("City HP after attack: {}", city.health);
                msg!("City Wall destroyed");
            } else {
                city.wall_health -= u32::from(given_wall_damage);
                msg!("City Wall HP after attack: {}", city.wall_health);
            }
        } else {
            msg!("Given damage to the city: {}", given_damage);
            msg!("Taken damage from the city: {}", taken_damage);

            // handle damage for city health
            if u32::from(given_damage) >= city.health {
                city.health = 0;
                msg!("City has been destroyed");
            } else {
                city.health -= u32::from(given_damage);
                msg!("City HP after attack: {}", city.health);
            }
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
            UnitType::Horseman => {
                researched_technologies.contains(&TechnologyType::HorsebackRiding)
            }
            UnitType::Swordsman => researched_technologies.contains(&TechnologyType::IronWorking),
            UnitType::Crossbowman => {
                researched_technologies.contains(&TechnologyType::MedievalWarfare)
            }
            UnitType::Musketman => researched_technologies.contains(&TechnologyType::Gunpowder),
            UnitType::Rifleman => researched_technologies.contains(&TechnologyType::Ballistics),
            UnitType::Tank => researched_technologies.contains(&TechnologyType::TanksAndArmor),
            UnitType::Galley => researched_technologies.contains(&TechnologyType::MaritimeNavigation),
            UnitType::Frigate => researched_technologies.contains(&TechnologyType::AdvancedShipbuilding),
            UnitType::Battleship => researched_technologies.contains(&TechnologyType::OceanicTrade),
        }
    }
}
