use anchor_lang::prelude::*;

pub mod errors;
use crate::errors::{UnitError, BuildingError};

declare_id!("GoiXQMoEhhLM8MSbfUFhHz4punJqXNHEQh6ysegmuHJz");

const MAX_UNITS: u8 = 20;
const MAX_CITIES: u8 = 20;
const MAX_BUILDINGS: u8 = 20;

#[program]
pub mod solciv {
    use super::*;

    pub fn initialize_game(ctx: Context<InitializeGame>, map: [u8; 400]) -> Result<()> {
        ctx.accounts.game.player = ctx.accounts.player.key().clone();
        ctx.accounts.game.turn = 1;
        ctx.accounts.game.map = map;
    
        msg!("Game created!");
    
        Ok(())
    }

    pub fn initialize_player(ctx: Context<InitializePlayer>) -> Result<()> {
        ctx.accounts.player_account.game = ctx.accounts.game.key().clone();
        ctx.accounts.player_account.player = ctx.accounts.player.key().clone();
        ctx.accounts.player_account.points = 0;
        ctx.accounts.player_account.next_city_id = 0;
        ctx.accounts.player_account.next_unit_id = 0;
        ctx.accounts.player_account.resources = Resources {
            gold: 0,
            food: 10,
            wood: 5,
            stone: 0,
            iron: 0,
        };
        ctx.accounts.player_account.units = vec![
            Unit {
                unit_id: 0,
                player: ctx.accounts.player.key().clone(),
                game: ctx.accounts.game.key().clone(),
                unit_type: UnitType::Settler,
                x: 2,
                y: 2,
                attack: 0,
                health: 100,
                movement_range: 2,
                remaining_actions: 1,
                is_alive: true,
            },
            Unit {
                unit_id: 1,
                player: ctx.accounts.player.key().clone(),
                game: ctx.accounts.game.key().clone(),
                unit_type: UnitType::Builder,
                x: 3,
                y: 2,
                attack: 14,
                health: 100,
                movement_range: 2,
                remaining_actions: 1,
                is_alive: true,
            },
            Unit {
                unit_id: 2,
                player: ctx.accounts.player.key().clone(),
                game: ctx.accounts.game.key().clone(),
                unit_type: UnitType::Warrior,
                x: 2,
                y: 3,
                attack: 14,
                health: 100,
                movement_range: 2,
                remaining_actions: 1,
                is_alive: true,
            }
        ];
        ctx.accounts.player_account.next_unit_id = 3;
    
        msg!("Player created!");
    
        Ok(())
    }

    // #[access_control(MoveUnit::validate_unit_move(&ctx, unit_id, x, y))]
    pub fn move_unit(
        ctx: Context<MoveUnit>,
        unit_id: u32,
        x: u8,
        y: u8,
    ) -> Result<()> {
        let unit = ctx.accounts.player_account.units.iter().find(|u| u.unit_id == unit_id).ok_or(UnitError::UnitNotFound)?;
        
        // Check if the tile is within the map bounds
        if x >= 20 || y >= 20 {
            return err!(UnitError::OutOfMapBounds);
        }

        // Check if the unit has remaining movement_range points
        if unit.movement_range == 0 {
            return err!(UnitError::CannotMove);
        }

        // Check if the new position is within the movement_range
        let dist = ((unit.x as i16 - x as i16).abs() + (unit.y as i16 - y as i16).abs()) as u8;
        msg!("Initial position: ({}, {})", unit.x, unit.y);
        msg!("New position: ({}, {})", x, y);
        msg!("Distance: {}", dist);
        if dist > unit.movement_range {
            return err!(UnitError::OutOfMovementRange);
        }

        // Check if the tile is not occupied by another unit
        if ctx.accounts.player_account.units.iter().any(|u| u.x == x && u.y == y && u.unit_id != unit_id) {
            return err!(UnitError::TileOccupied);
        }

        let units = &mut ctx.accounts.player_account.units;
        
        // Find the index of the unit with the given unit_id
        let unit_idx = units.iter().position(|u| u.unit_id == unit_id).ok_or(UnitError::UnitNotFound)?;
        
        // Update the coordinates of the unit
        ctx.accounts.player_account.units[unit_idx].x = x;
        ctx.accounts.player_account.units[unit_idx].y = y;
        ctx.accounts.player_account.units[unit_idx].movement_range -= dist;
        
        Ok(())
    }

    pub fn found_city(ctx: Context<FoundCity>, x: u8, y: u8, unit_id: u32) -> Result<()> {
        // 1. Validate if the unit with `unit_id` is a settler and is at `x` and `y`.
        let unit_idx = ctx.accounts.player_account.units.iter().position(|u| u.unit_id == unit_id).ok_or(UnitError::UnitNotFound)?;
        let unit = &ctx.accounts.player_account.units[unit_idx];
        if unit.unit_type != UnitType::Settler {
            return err!(UnitError::InvalidUnitType);
        }
        if (unit.x, unit.y) != (x, y) {
            return err!(UnitError::UnitWrongPosition);
        }
        
        // 2. Check if there is already a city at `x` and `y`.
        let is_occupied = ctx.accounts.player_account.cities.iter().any(|city| city.x == x && city.y == y);
        if is_occupied {
            return err!(BuildingError::TileOccupied);
        }

        // 3. Initialize the new City.
        let new_city = City {
            city_id: ctx.accounts.player_account.next_city_id,
            player: ctx.accounts.player_account.player,
            game: ctx.accounts.game.key(),
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
        };
        ctx.accounts.player_account.cities.push(new_city);
        
        // 4. Remove the settler unit used to found the city.
        ctx.accounts.player_account.units.remove(unit_idx);
        
        // 5. Update the next_city_id in the player account.
        ctx.accounts.player_account.next_city_id = ctx.accounts.player_account.next_city_id.checked_add(1).unwrap();
        
        msg!("Founded new city!");
        
        Ok(())
    }

    pub fn end_turn(ctx: Context<EndTurn>) -> Result<()> {
        // Iterate over all units of the player and reset the movement_range to 2
        for unit in &mut ctx.accounts.player_account.units.iter_mut() {
            if unit.is_alive {
                unit.movement_range = 2;
            }
        }
        let mut gold = 0;
        let mut food = 0;
        for city in &mut ctx.accounts.player_account.cities {
            gold += city.gold_yield;
            food += city.food_yield;
        }

        ctx.accounts.player_account.resources.gold = ctx.accounts.player_account.resources.gold.checked_add(gold).unwrap();
        ctx.accounts.player_account.resources.food = ctx.accounts.player_account.resources.food.checked_add(food).unwrap();

        // units.retain(|unit| unit.is_alive);
        ctx.accounts.game.turn += 1;
        Ok(())
    }
}

#[account]
pub struct Game {
    pub player: Pubkey,
    pub turn: u32,
    pub map: [u8; 400],
}

#[derive(Accounts)]
pub struct InitializeGame<'info> {
    #[account(
        init, 
        seeds=[b"GAME", player.key().as_ref()],
        bump,
        payer = player, 
        space = std::mem::size_of::<Game>() + 8
    )]
    pub game: Account<'info, Game>,

    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Player {
    pub game: Pubkey,
    pub player: Pubkey,
    pub points: u32,
    pub cities: Vec<City>,
    pub units: Vec<Unit>,
    pub resources: Resources,
    pub next_city_id: u32,
    pub next_unit_id: u32,
}

#[derive(Accounts)]
pub struct InitializePlayer<'info> {
    pub game: Account<'info, Game>,

    #[account(
        init, 
        seeds=[
            b"PLAYER", 
            game.key().as_ref(), 
            player.key().as_ref()
        ], 
        bump, 
        payer = player, 
        space = std::mem::size_of::<Player>() +
            std::mem::size_of::<Unit>() * MAX_UNITS as usize +
            std::mem::size_of::<City>() * MAX_CITIES as usize +
            std::mem::size_of::<BuildingType>() * MAX_BUILDINGS as usize +
            std::mem::size_of::<Resources>() + 8)
    ]
    pub player_account: Box<Account<'info, Player>>,

    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
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

#[derive(Accounts)]
pub struct FoundCity<'info> {
    #[account(mut)]
    game: Account<'info, Game>,
    #[account(mut)]
    player_account: Account<'info, Player>,
    // #[account(
    //     init, 
    //     payer = player, 
    //     seeds=[b"CITY", game.key().as_ref(), player.key().as_ref(), player_account.next_city_id.to_le_bytes().as_ref()], 
    //     bump, 
    //     space = std::mem::size_of::<City>() + 8
    // )]
    // pub city: Account<'info, City>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
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

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone)]
pub struct Resources {
    pub gold: u32,
    pub food: u32,
    pub wood: u32,
    pub stone: u32,
    pub iron: u32,
}

#[derive(Accounts)]
pub struct MoveUnit<'info> {
    #[account(mut)]
    player_account: Account<'info, Player>,
    #[account(mut)]
    pub player: Signer<'info>,
}

#[derive(Accounts)]
pub struct EndTurn<'info> {
    #[account(mut)]
    game: Account<'info, Game>,
    #[account(mut)]
    player_account: Account<'info, Player>,
    #[account(mut)]
    pub player: Signer<'info>,
}