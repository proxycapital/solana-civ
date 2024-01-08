use crate::consts::*;
use crate::errors::*;
use crate::state::*;
use crate::utils::*;
use anchor_lang::prelude::*;

pub fn move_unit(ctx: Context<MoveUnit>, unit_id: u32, x: u8, y: u8) -> Result<()> {
    let unit = ctx
        .accounts
        .player_account
        .units
        .iter()
        .find(|u| u.unit_id == unit_id)
        .ok_or(UnitError::UnitNotFound)?;
    let base_movement_range = Unit::get_base_movement_range(unit.unit_type);

    // Check if the tile is within the map bounds
    if x >= MAP_BOUND || y >= MAP_BOUND {
        return err!(UnitError::OutOfMapBounds);
    }

    // Check if the unit has remaining movement_range points
    if unit.movement_range == 0 {
        return err!(UnitError::CannotMove);
    }

    // Check if the new position is within the movement_range
    // Manhattan Distance:
    let dist = ((unit.x as i16 - x as i16).abs() + (unit.y as i16 - y as i16).abs()) as u8;
    msg!("Initial position: ({}, {})", unit.x, unit.y);
    msg!("New position: ({}, {})", x, y);
    msg!("Distance: {}", dist);
    if dist > unit.movement_range {
        return err!(UnitError::OutOfMovementRange);
    }

    // Check if the tile is not occupied by another unit
    if ctx
        .accounts
        .player_account
        .units
        .iter()
        .any(|u| u.x == x && u.y == y && u.unit_id != unit_id)
    {
        return err!(UnitError::TileOccupied);
    }

    let units = &mut ctx.accounts.player_account.units;

    // Find the index of the unit with the given unit_id
    let unit_idx = units
        .iter()
        .position(|u| u.unit_id == unit_id)
        .ok_or(UnitError::UnitNotFound)?;

    // Update the coordinates of the unit
    ctx.accounts.player_account.units[unit_idx].x = x;
    ctx.accounts.player_account.units[unit_idx].y = y;
    ctx.accounts.player_account.units[unit_idx].movement_range -= dist;

    // Mark tiles within movement range as discovered
    let start_x = x.saturating_sub(base_movement_range);
    let end_x = std::cmp::min(x + base_movement_range, 19);
    let start_y = y.saturating_sub(base_movement_range);
    let end_y = std::cmp::min(y + base_movement_range, 19);

    for j in start_y..=end_y {
        for i in start_x..=end_x {
            let dist = ((i as i16 - x as i16).abs() + (j as i16 - y as i16).abs()) as u8;
            if dist <= base_movement_range {
                let index = (j as usize) * 20 + (i as usize);
                ctx.accounts.game.map[index].discovered = true;
            }
        }
    }

    Ok(())
}

pub fn upgrade_unit(ctx: Context<UpgradeUnit>, unit_id: u32) -> Result<()> {
    let units = &mut ctx.accounts.player_account.units;
    let unit_idx = units
        .iter()
        .position(|u| u.unit_id == unit_id)
        .ok_or(UnitError::UnitNotFound)?;

    let unit_exp = units[unit_idx].experience;
    let unit_level = units[unit_idx].level;
    let unit_health = units[unit_idx].health;
    let unit_movement_range = units[unit_idx].movement_range;

    // Unit doesn't have movement range
    if unit_movement_range == 0 {
        return err!(UnitError::NoMovementPoints);
    }

    // Check if max level was reached
    if unit_level >= EXP_THRESHOLDS.len() as u8 {
        return err!(UnitError::MaxLevelReached);
    }

    // Check if unit has enough experience to level up
    if unit_exp < EXP_THRESHOLDS[unit_level as usize] {
        return err!(UnitError::NotEnoughExp);
    }

    // Level Up
    if unit_health >= 70 {
        ctx.accounts.player_account.units[unit_idx].health = 100;
    } else {
        ctx.accounts.player_account.units[unit_idx].health += 30;
    }

    ctx.accounts.player_account.units[unit_idx].attack += 2;
    ctx.accounts.player_account.units[unit_idx].level += 1;

    // After the upgrade, the unit cannot move or attack anymore
    ctx.accounts.player_account.units[unit_idx].movement_range = 0;

    Ok(())
}

fn calculate_controlled_tiles(x: u8, y: u8, existing_cities: &[City]) -> Vec<TileCoordinate> {
    let mut tiles = Vec::new();

    for dx in -2..=2 {
        for dy in -2..=2 {
            let tile_x = (x as i16 + dx) as u8;
            let tile_y = (y as i16 + dy) as u8;

            // Check map boundaries and existing city control
            if tile_x < MAP_BOUND
                && tile_y < MAP_BOUND
                && !existing_cities
                    .iter()
                    .any(|city| city.controls_tile(tile_x, tile_y))
            {
                tiles.push(TileCoordinate {
                    x: tile_x,
                    y: tile_y,
                });
            }
        }
    }

    tiles
}

pub fn found_city(ctx: Context<FoundCity>, x: u8, y: u8, unit_id: u32, name: String) -> Result<()> {
    // Validate if the unit with `unit_id` is a settler and is at `x` and `y`.
    let unit_idx = ctx
        .accounts
        .player_account
        .units
        .iter()
        .position(|u| u.unit_id == unit_id)
        .ok_or(UnitError::UnitNotFound)?;
    let unit = &ctx.accounts.player_account.units[unit_idx];
    if unit.unit_type != UnitType::Settler {
        return err!(UnitError::InvalidUnitType);
    }
    if (unit.x, unit.y) != (x, y) {
        return err!(UnitError::UnitWrongPosition);
    }

    // Check if there is already a city at `x` and `y`.
    let is_occupied = ctx
        .accounts
        .player_account
        .cities
        .iter()
        .any(|city| city.x == x && city.y == y)
        || ctx
            .accounts
            .player_account
            .tiles
            .iter()
            .any(|tile| tile.x == x && tile.y == y);
    if is_occupied {
        return err!(BuildingError::TileOccupied);
    }

    let controlled_tiles = calculate_controlled_tiles(x, y, &ctx.accounts.player_account.cities);

    // Initialize the new City.
    let params = NewCityParams {
        city_id: ctx.accounts.player_account.next_city_id,
        player: ctx.accounts.player_account.player,
        game: ctx.accounts.game.key(),
        x,
        y,
        name,
        health: 100,
        controlled_tiles: controlled_tiles.clone(),
    };

    let new_city = City::new(params);

    ctx.accounts.player_account.cities.push(new_city);

    // Mark controlled tiles as discovered
    for tile_coord in controlled_tiles {
        let tile_index = (tile_coord.y * MAP_BOUND) + tile_coord.x;
        ctx.accounts.game.map[tile_index as usize].discovered = true;
    }

    // Remove the settler unit used to found the city.
    ctx.accounts.player_account.units.remove(unit_idx);

    // Update the next_city_id in the player account.
    ctx.accounts.player_account.next_city_id = ctx
        .accounts
        .player_account
        .next_city_id
        .checked_add(1)
        .unwrap();

    msg!("Founded new city!");

    Ok(())
}

pub fn upgrade_tile(ctx: Context<UpgradeTile>, x: u8, y: u8, unit_id: u32) -> Result<()> {
    // Validate if the unit with `unit_id` is a Builder and is at `x` and `y`.
    let unit_idx = ctx
        .accounts
        .player_account
        .units
        .iter()
        .position(|u| u.unit_id == unit_id)
        .ok_or(UnitError::UnitNotFound)?;
    let unit = &ctx.accounts.player_account.units[unit_idx];
    if unit.unit_type != UnitType::Builder {
        return err!(UnitError::InvalidUnitType);
    }
    if (unit.x, unit.y) != (x, y) {
        return err!(UnitError::UnitWrongPosition);
    }

    // Check if the tile type is upgradeable and the tile is not occupied by a City or another Tile.
    let map_idx = (y as usize) * MAP_BOUND as usize + x as usize;
    match ctx.accounts.game.map[map_idx].terrain {
        1 | 2 | 5 | 6 | 7 => {} // allowable tile types
        _ => return err!(TileError::NotUpgradeable),
    }

    if ctx
        .accounts
        .player_account
        .cities
        .iter()
        .any(|city| city.x == x && city.y == y)
        || ctx
            .accounts
            .player_account
            .tiles
            .iter()
            .any(|tile| tile.x == x && tile.y == y)
    {
        return err!(TileError::TileOccupied);
    }

    // Check if the tile is controlled by any of the player's cities
    let is_controlled = ctx.accounts.player_account.cities.iter().any(|city| {
        city.controlled_tiles
            .iter()
            .any(|tile| tile.x == x && tile.y == y)
    });
    if !is_controlled {
        return err!(TileError::TileNotControlled);
    }

    // Initialize the new Tile and push it to player_account tiles vector.
    let tile_type = match ctx.accounts.game.map[map_idx].terrain {
        1 => TileType::IronMine,
        2 => TileType::LumberMill,
        5 => TileType::StoneQuarry,
        6 => TileType::Farm,
        7 => TileType::Pasture,
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

pub fn attack_unit(ctx: Context<AttackUnit>, attacker_id: u32, defender_id: u32) -> Result<()> {
    let attacker = ctx
        .accounts
        .player_account
        .units
        .iter_mut()
        .find(|u| u.unit_id == attacker_id)
        .ok_or(UnitError::UnitNotFound)?;
    let defender = ctx
        .accounts
        .npc_account
        .units
        .iter_mut()
        .find(|u| u.unit_id == defender_id)
        .ok_or(UnitError::UnitNotFound)?;

    if attacker.movement_range == 0 {
        return err!(UnitError::NoMovementPoints);
    }

    // Check proximity (attacker should be 1 tile away from defender)
    // Chebyshev Distance:
    let dist_x = (attacker.x as i16 - defender.x as i16).abs();
    let dist_y = (attacker.y as i16 - defender.y as i16).abs();
    let dist = std::cmp::max(dist_x, dist_y) as u8;

    if dist != 1 {
        return err!(UnitError::OutOfAttackRange);
    }

    attacker.attack_unit(defender, None)?;

    if !defender.is_alive {
        ctx.accounts.player_account.resources.gems = ctx
            .accounts
            .player_account
            .resources
            .gems
            .checked_add(GEMS_PER_KILL as u32)
            .unwrap_or(u32::MAX);
    }

    // Retain only alive units in the game
    ctx.accounts.player_account.units.retain(|u| u.is_alive);
    ctx.accounts.npc_account.units.retain(|u| u.is_alive);

    Ok(())
}

pub fn attack_city(ctx: Context<AttackCity>, attacker_id: u32, city_id: u32) -> Result<()> {
    let attacker = ctx
        .accounts
        .player_account
        .units
        .iter_mut()
        .find(|u| u.unit_id == attacker_id)
        .ok_or(UnitError::UnitNotFound)?;

    if attacker.movement_range == 0 {
        return err!(UnitError::NoMovementPoints);
    }

    let target_city = ctx
        .accounts
        .npc_account
        .cities
        .iter_mut()
        .find(|c| c.city_id == city_id)
        .ok_or(CityError::CityNotFound)?;

    let dist_x = (attacker.x as i16 - target_city.x as i16).abs();
    let dist_y = (attacker.y as i16 - target_city.y as i16).abs();
    let dist = std::cmp::max(dist_x, dist_y) as u8;

    if dist != 1 {
        return err!(UnitError::OutOfAttackRange);
    }

    attacker.attack_city(target_city)?;
    attacker.movement_range = 0;
    attacker.experience = get_new_exp(attacker.level, attacker.experience, 3);

    let city_was_destroyed = target_city.health == 0;

    if city_was_destroyed {
        ctx.accounts.player_account.resources.gems = ctx
            .accounts
            .player_account
            .resources
            .gems
            .checked_add(GEMS_PER_CITY_DESTROYED as u32)
            .unwrap_or(u32::MAX);
    }

    ctx.accounts.player_account.units.retain(|u| u.is_alive);
    ctx.accounts.npc_account.cities.retain(|c| c.health > 0);

    Ok(())
}

#[derive(Accounts)]
pub struct FoundCity<'info> {
    #[account(mut, has_one = player)]
    pub game: Account<'info, Game>,
    #[account(mut, has_one = player)]
    pub player_account: Account<'info, Player>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MoveUnit<'info> {
    #[account(mut, has_one = player)]
    pub game: Box<Account<'info, Game>>,
    #[account(mut, has_one = player)]
    pub player_account: Account<'info, Player>,
    #[account(mut)]
    pub player: Signer<'info>,
}

#[derive(Accounts)]
pub struct HealUnit<'info> {
    #[account(mut, has_one = player)]
    pub player_account: Account<'info, Player>,
    #[account(mut)]
    pub player: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpgradeUnit<'info> {
    #[account(mut, has_one = player)]
    pub player_account: Account<'info, Player>,
    #[account(mut)]
    pub player: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpgradeTile<'info> {
    #[account(mut, has_one = player)]
    pub game: Box<Account<'info, Game>>,
    #[account(mut, has_one = player)]
    pub player_account: Account<'info, Player>,
    #[account(mut)]
    pub player: Signer<'info>,
}

#[derive(Accounts)]
pub struct AttackUnit<'info> {
    #[account(mut, has_one = player)]
    pub game: Box<Account<'info, Game>>,
    #[account(mut, has_one = player)]
    pub player_account: Account<'info, Player>,
    #[account(mut, has_one = player)]
    pub npc_account: Account<'info, Npc>,
    #[account(mut)]
    pub player: Signer<'info>,
}

#[derive(Accounts)]
pub struct AttackCity<'info> {
    #[account(mut, has_one = player)]
    pub game: Box<Account<'info, Game>>,
    #[account(mut, has_one = player)]
    pub player_account: Account<'info, Player>,
    #[account(mut, has_one = player)]
    pub npc_account: Account<'info, Npc>,
    #[account(mut)]
    pub player: Signer<'info>,
}
