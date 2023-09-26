mod state;
mod instructions;
mod errors;

use anchor_lang::prelude::*;
use crate::instructions::*;

declare_id!("GoiXQMoEhhLM8MSbfUFhHz4punJqXNHEQh6ysegmuHJz");

#[program]
pub mod solciv {
    use super::*;

    pub fn initialize_game(ctx: Context<InitializeGame>, map: [u8; 400]) -> Result<()> {
        instructions::initialize_game(ctx, map)
    }

    pub fn initialize_player(ctx: Context<InitializePlayer>) -> Result<()> {
        instructions::initialize_player(ctx)
    }

    pub fn move_unit(
        ctx: Context<MoveUnit>,
        unit_id: u32,
        x: u8,
        y: u8,
    ) -> Result<()> {
        instructions::move_unit(ctx, unit_id, x, y)
    }

    pub fn found_city(ctx: Context<FoundCity>, x: u8, y: u8, unit_id: u32) -> Result<()> {
        instructions::found_city(ctx, x, y, unit_id)
    }

    pub fn end_turn(ctx: Context<EndTurn>) -> Result<()> {
        instructions::end_turn(ctx)
    }
}