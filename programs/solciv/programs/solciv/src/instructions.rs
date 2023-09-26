use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;

const MAX_UNITS: u8 = 20;
const MAX_CITIES: u8 = 20;
const MAX_BUILDINGS: u8 = 20;
const MAX_UPGRADED_TILES: u8 = 100;

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
  // @todo: consider implementing helper methods for initializing the resources, units or other default things
  ctx.accounts.player_account.resources = Resources {
      gold: 0,
      food: 10,
      wood: 5,
      stone: 0,
      iron: 0,
  };
  ctx.accounts.player_account.units = vec![
      Unit::new(0, ctx.accounts.player.key().clone(), ctx.accounts.game.key().clone(), UnitType::Settler, 2, 2),
      Unit::new(1, ctx.accounts.player.key().clone(), ctx.accounts.game.key().clone(), UnitType::Builder, 3, 2),
      Unit::new(2, ctx.accounts.player.key().clone(), ctx.accounts.game.key().clone(), UnitType::Warrior, 2, 3),
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
  // Validate if the unit with `unit_id` is a settler and is at `x` and `y`.
  let unit_idx = ctx.accounts.player_account.units.iter().position(|u| u.unit_id == unit_id).ok_or(UnitError::UnitNotFound)?;
  let unit = &ctx.accounts.player_account.units[unit_idx];
  if unit.unit_type != UnitType::Settler {
      return err!(UnitError::InvalidUnitType);
  }
  if (unit.x, unit.y) != (x, y) {
      return err!(UnitError::UnitWrongPosition);
  }
  
  // Check if there is already a city at `x` and `y`.
  let is_occupied = ctx.accounts.player_account.cities.iter().any(|city| city.x == x && city.y == y);
  if is_occupied {
      return err!(BuildingError::TileOccupied);
  }

  // Initialize the new City.
  let new_city = City::new(ctx.accounts.player_account.next_city_id, ctx.accounts.player_account.player, ctx.accounts.game.key(), x, y);
  ctx.accounts.player_account.cities.push(new_city);
  
  // Remove the settler unit used to found the city.
  ctx.accounts.player_account.units.remove(unit_idx);
  
  // Update the next_city_id in the player account.
  ctx.accounts.player_account.next_city_id = ctx.accounts.player_account.next_city_id.checked_add(1).unwrap();
  
  msg!("Founded new city!");
  
  Ok(())
}

pub fn upgrade_tile(ctx: Context<UpgradeTile>, x: u8, y: u8, unit_id: u32) -> Result<()> {
    // Validate if the unit with `unit_id` is a Builder and is at `x` and `y`.
    let unit_idx = ctx.accounts.player_account.units.iter().position(|u| u.unit_id == unit_id).ok_or(UnitError::UnitNotFound)?;
    let unit = &ctx.accounts.player_account.units[unit_idx];
    if unit.unit_type != UnitType::Builder {
        return err!(UnitError::InvalidUnitType);
    }
    if (unit.x, unit.y) != (x, y) {
        return err!(UnitError::UnitWrongPosition);
    }

    // Check if the tile type is upgradeable and the tile is not occupied by a City or another Tile.
    let map_idx = (y as usize) * 20 + x as usize;
    match ctx.accounts.game.map[map_idx] {
        2 | 5 | 6 => {}, // allowable tile types
        _ => return err!(TileError::NotUpgradeable),
    }
    
    if ctx.accounts.player_account.cities.iter().any(|city| city.x == x && city.y == y) 
        || ctx.accounts.player_account.tiles.iter().any(|tile| tile.x == x && tile.y == y) {
        return err!(TileError::TileOccupied);
    }
    
    // Initialize the new Tile and push it to player_account tiles vector.
    let tile_type = match ctx.accounts.game.map[map_idx] {
        2 => TileType::TimberCamp,
        5 => TileType::StoneQuarry,
        6 => TileType::CornField,
        // we've already checked the tile type above, if there was no match, we would have returned an error NotUpgradeable
        _ => unreachable!(), 
    };
    
    let new_tile = Tile::new(tile_type, x, y);
    ctx.accounts.player_account.tiles.push(new_tile);
    
    // Reduce remaining_actions of the Builder and remove it if remaining_actions hit 0.
    ctx.accounts.player_account.units[unit_idx].remaining_actions -= 1;
    if ctx.accounts.player_account.units[unit_idx].remaining_actions == 0 {
        ctx.accounts.player_account.units.remove(unit_idx);
    }
    
    msg!("Tile upgraded!");
    
    Ok(())
}

pub fn end_turn(ctx: Context<EndTurn>) -> Result<()> {
  // Iterate over all units of the player and reset the movement_range to 2
  for unit in &mut ctx.accounts.player_account.units.iter_mut() {
      if unit.is_alive {
          unit.movement_range = 2;
      }
  }
  /*
  
      // Adjust player's resources based on tile type.
    match tile_type {
        TileType::TimberCamp => ctx.accounts.player_account.resources.wood += 1,
        TileType::StoneQuarry => ctx.accounts.player_account.resources.stone += 1,
        TileType::CornField => ctx.accounts.player_account.resources.food += 1,
        // ... other cases
    }
  
   */
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
            std::mem::size_of::<Tile>() * MAX_UPGRADED_TILES as usize +
            std::mem::size_of::<BuildingType>() * MAX_BUILDINGS as usize +
            std::mem::size_of::<Resources>() + 8)
    ]
    pub player_account: Box<Account<'info, Player>>,

    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FoundCity<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub player_account: Account<'info, Player>,
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

#[derive(Accounts)]
pub struct MoveUnit<'info> {
    #[account(mut)]
    pub player_account: Account<'info, Player>,
    #[account(mut)]
    pub player: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpgradeTile<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub player_account: Account<'info, Player>,
    #[account(mut)]
    pub player: Signer<'info>,
}

#[derive(Accounts)]
pub struct EndTurn<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub player_account: Account<'info, Player>,
    #[account(mut)]
    pub player: Signer<'info>,
}