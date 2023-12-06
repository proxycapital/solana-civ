#![allow(clippy::result_large_err)]

mod consts;
mod errors;
mod instructions;
mod state;

use crate::instructions::*;
use crate::state::{ProductionItem, TechnologyType};
use anchor_lang::prelude::*;

declare_id!("8RTGhsKoX81TVW8zwZwR483WKSWjdW4eYcVkHsgsgtCr");

#[program]
pub mod solciv {
    use super::*;

    pub fn initialize_game(ctx: Context<InitializeGame>, map: [u8; 400]) -> Result<()> {
        instructions::initialize_game(ctx, map)
    }

    pub fn initialize_player(ctx: Context<InitializePlayer>) -> Result<()> {
        instructions::initialize_player(ctx)
    }

    pub fn initialize_npc(ctx: Context<InitializeNpc>) -> Result<()> {
        instructions::initialize_npc(ctx)
    }

    pub fn move_unit(ctx: Context<MoveUnit>, unit_id: u32, x: u8, y: u8) -> Result<()> {
        instructions::move_unit(ctx, unit_id, x, y)
    }

    pub fn heal_unit(ctx: Context<HealUnit>, unit_id: u32) -> Result<()> {
        instructions::heal_unit(ctx, unit_id)
    }

    pub fn found_city(
        ctx: Context<FoundCity>,
        x: u8,
        y: u8,
        unit_id: u32,
        name: String,
    ) -> Result<()> {
        instructions::found_city(ctx, x, y, unit_id, name)
    }

    pub fn add_to_production_queue(
        ctx: Context<AddToProductionQueue>,
        city_id: u32,
        item: ProductionItem,
    ) -> Result<()> {
        instructions::add_to_production_queue(ctx, city_id, item)
    }

    pub fn remove_from_production_queue(
        ctx: Context<RemoveFromProductionQueue>,
        city_id: u32,
        index: u8,
    ) -> Result<()> {
        instructions::remove_from_production_queue(ctx, city_id, index)
    }

    pub fn purchase_with_gold(
        ctx: Context<PurchaseWithGold>,
        city_id: u32,
        item: ProductionItem,
    ) -> Result<()> {
        instructions::purchase_with_gold(ctx, city_id, item)
    }

    pub fn start_research(
        ctx: Context<StartResearch>,
        technology_type: TechnologyType,
    ) -> Result<()> {
        instructions::start_research(ctx, technology_type)
    }

    pub fn upgrade_tile(ctx: Context<UpgradeTile>, x: u8, y: u8, unit_id: u32) -> Result<()> {
        instructions::upgrade_tile(ctx, x, y, unit_id)
    }

    pub fn attack_unit(ctx: Context<AttackUnit>, attacker_id: u32, defender_id: u32) -> Result<()> {
        instructions::attack_unit(ctx, attacker_id, defender_id)
    }

    pub fn attack_city(ctx: Context<AttackCity>, attacker_id: u32, city_id: u32) -> Result<()> {
        instructions::attack_city(ctx, attacker_id, city_id)
    }

    pub fn mint_gems(ctx: Context<MintGems>) -> Result<()> {
        instructions::mint_gems(ctx)
    }

    pub fn end_turn(ctx: Context<EndTurn>) -> Result<()> {
        instructions::end_turn(ctx)
    }

    pub fn close_game(ctx: Context<Close>) -> Result<()> {
        instructions::close_game(ctx)
    }

    pub fn repair_wall(ctx: Context<RepairWall>, city_id: u32) -> Result<()> {
        instructions::repair_wall(ctx, city_id)
    }
}
