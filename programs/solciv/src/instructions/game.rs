use crate::consts::*;
use crate::state::*;
use anchor_lang::prelude::*;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn initialize_game(ctx: Context<InitializeGame>, map: [u8; 400]) -> Result<()> {
    ctx.accounts.game.player = ctx.accounts.player.key();
    ctx.accounts.game.turn = 1;
    ctx.accounts.game.defeat = false;
    ctx.accounts.game.victory = false;

    // Set the tiles from 0 to 7 as discovered and initialize all tiles with a terrain type
    for i in 0..20 {
        for j in 0..20 {
            let index = i * 20 + j;

            ctx.accounts.game.map[index].terrain = map[index];

            // Mark tiles from (0,0) to (7,7) as discovered
            ctx.accounts.game.map[index].discovered = i < 8 && j < 8;
        }
    }

    msg!("Game created!");

    Ok(())
}

fn heal_units_and_reset_movement_range(units: &mut [Unit]) {
    for unit in units.iter_mut().filter(|u| u.is_alive) {
        // Heal if the unit did not move/attack and has less than max HP
        if unit.health < 100 && unit.movement_range == Unit::get_base_movement_range(unit.unit_type)
        {
            unit.health = std::cmp::min(unit.health + 5, 100);
        }

        // Reset movement range
        unit.movement_range = Unit::get_base_movement_range(unit.unit_type);
    }
}

fn find_adjacent_tiles(
    tiles: &[TileCoordinate],
    controlled_tiles: &[TileCoordinate],
) -> Vec<TileCoordinate> {
    let controlled_set: HashSet<_> = HashSet::from_iter(controlled_tiles.iter());
    let tile_set: HashSet<_> = HashSet::from_iter(tiles.iter());

    tiles
        .iter()
        .flat_map(adjacent_coords)
        .filter(|coord| {
            is_within_bounds(coord) && !tile_set.contains(coord) && !controlled_set.contains(coord)
        })
        .collect()
}

fn adjacent_coords(tile: &TileCoordinate) -> Vec<TileCoordinate> {
    vec![
        TileCoordinate {
            x: tile.x,
            y: tile.y.saturating_sub(1),
        },
        TileCoordinate {
            x: tile.x,
            y: tile.y + 1,
        },
        TileCoordinate {
            x: tile.x.saturating_sub(1),
            y: tile.y,
        },
        TileCoordinate {
            x: tile.x + 1,
            y: tile.y,
        },
    ]
}

fn is_within_bounds(coord: &TileCoordinate) -> bool {
    coord.x < MAP_BOUND && coord.y < MAP_BOUND
}

fn calculate_resources(player_account: &Player) -> (i32, u32, u32, u32, u32, u32) {
    // Calculate resources yielded by cities and tiles.
    // This function will return a tuple (gold, wood, stone, iron, horses, science).
    let mut resources: (i32, u32, u32, u32, u32, u32) = (0, 0, 0, 0, 0, 0);

    for city in &player_account.cities {
        resources.0 += city.gold_yield as i32;
        resources.5 += city.science_yield;
    }

    for tile in &player_account.tiles {
        match tile.tile_type {
            TileType::LumberMill => resources.1 += 2,
            TileType::StoneQuarry => resources.2 += 2,
            TileType::IronMine => resources.3 += 2,
            TileType::Pasture => resources.4 += 2,
            _ => {} // Ignore other cases, including Farm
        }
    }

    // Deduct unit maintenance costs from gold yield
    for unit in &player_account.units {
        resources.0 = resources
            .0
            .checked_sub(unit.maintenance_cost)
            .unwrap_or(i32::MIN);
    }

    resources
}

fn process_production_queues(player_account: &mut Player, game_key: Pubkey) -> Result<()> {
    let mut new_units = Vec::new();
    let mut next_unit_id = player_account.next_unit_id;
    let player = player_account.player;

    for city in &mut player_account.cities {
        if let Some(item) = city.production_queue.first().cloned() {
            let cost = match item {
                ProductionItem::Unit(unit_type) => Unit::get_base_stats(unit_type).5,
                ProductionItem::Building(building_type) => {
                    BuildingType::get_base_stats(building_type).0
                }
            };

            // Increment the accumulated production by the city's production yield
            city.accumulated_production += city.production_yield;

            if city.accumulated_production >= cost {
                // Production completed
                match item {
                    ProductionItem::Unit(unit_type) => {
                        // If the unit is a Settler and the city population is greater than 1, decrease the population
                        if unit_type == UnitType::Settler && city.population > 1 {
                            city.population -= 1;
                        }
                        // Create a new unit and add it to the player's units
                        let new_unit =
                            Unit::new(next_unit_id, player, game_key, unit_type, city.x, city.y);
                        new_units.push(new_unit);
                        next_unit_id += 1;
                    }
                    ProductionItem::Building(building_type) => {
                        // Construct the building in the city
                        city.construct_building(building_type)?;
                    }
                }
                // Remove the item from the production queue and reset accumulated_production
                city.production_queue.remove(0);
                city.accumulated_production = 0;
            }
        }
    }

    player_account.units.append(&mut new_units);
    player_account.next_unit_id = next_unit_id;

    Ok(())
}

fn process_npc_movements_and_attacks(npc_units: &mut Vec<Unit>, player: &mut Player) -> Result<()> {
    let npc_units_count = npc_units.len();
    for i in 0..npc_units_count {
        if !npc_units[i].is_alive {
            continue;
        }

        // Level up check
        if npc_units[i].level < EXP_THRESHOLDS.len() as u8
            && npc_units[i].experience >= EXP_THRESHOLDS[npc_units[i].level as usize]
        {
            npc_units[i].level += 1;
            npc_units[i].attack += 2;
            npc_units[i].health = std::cmp::min(npc_units[i].health + 30, 100);
            npc_units[i].movement_range = 0;

            msg!(
                "NPC unit #{} leveled up to level {}",
                npc_units[i].unit_id,
                npc_units[i].level
            );

            continue; // Skip to the next unit
        }

        let mut min_dist = u16::MAX;
        let mut closest_target: Option<(u8, u8)> = None;

        // Find the closest player's unit or city to the NPC unit
        for player_unit in player.units.iter().filter(|u| u.is_alive) {
            let dist = ((npc_units[i].x as i16 - player_unit.x as i16).pow(2)
                + (npc_units[i].y as i16 - player_unit.y as i16).pow(2))
                as u16;
            if dist < min_dist {
                min_dist = dist;
                closest_target = Some((player_unit.x, player_unit.y));
            }
        }

        for city in player.cities.iter() {
            let dist = ((npc_units[i].x as i16 - city.x as i16).pow(2)
                + (npc_units[i].y as i16 - city.y as i16).pow(2)) as u16;
            if dist < min_dist {
                min_dist = dist;
                closest_target = Some((city.x, city.y));
            }
        }

        // If a closest target was found, make decisions for NPC units based on the proximity to this target
        if let Some((target_x, target_y)) = closest_target {
            let dist_x = (npc_units[i].x as i16 - target_x as i16).abs();
            let dist_y = (npc_units[i].y as i16 - target_y as i16).abs();
            let dist = std::cmp::max(dist_x, dist_y) as u8;

            if dist == 1 {
                let is_player_unit = player
                    .units
                    .iter_mut()
                    .any(|u| u.x == target_x && u.y == target_y);
                let is_city_with_wall = player.cities.iter_mut().any(|c| {
                    c.x == target_x && c.y == target_y && c.health > 0 && c.wall_health > 0
                });
                if is_city_with_wall && is_player_unit {
                    // attack unit that stay in the city with wall
                    let player_unit = player
                        .units
                        .iter_mut()
                        .find(|u| u.x == target_x && u.y == target_y && u.is_alive);
                    npc_units[i].attack_unit(player_unit.unwrap(), Some(true))?;
                    if !npc_units[i].is_alive {
                        player.resources.gems = player
                            .resources
                            .gems
                            .checked_add(GEMS_PER_KILL as u32)
                            .unwrap_or(u32::MAX);
                    }
                } else if let Some(player_unit) = player
                    .units
                    .iter_mut()
                    .find(|u| u.x == target_x && u.y == target_y && u.is_alive)
                {
                    npc_units[i].attack_unit(player_unit, None)?;
                    if !npc_units[i].is_alive {
                        player.resources.gems = player
                            .resources
                            .gems
                            .checked_add(GEMS_PER_KILL as u32)
                            .unwrap_or(u32::MAX);
                    }
                } else if let Some(player_city) = player
                    .cities
                    .iter_mut()
                    .find(|c| c.x == target_x && c.y == target_y && c.health > 0)
                {
                    npc_units[i].attack_city(player_city)?;
                    if !npc_units[i].is_alive {
                        player.resources.gems = player
                            .resources
                            .gems
                            .checked_add(GEMS_PER_KILL as u32)
                            .unwrap_or(u32::MAX);
                    }
                }
            } else {
                let dir_x = match npc_units[i].x.cmp(&target_x) {
                    std::cmp::Ordering::Less => 1,
                    std::cmp::Ordering::Greater => -1,
                    std::cmp::Ordering::Equal => 0,
                };
                let dir_y = match npc_units[i].y.cmp(&target_y) {
                    std::cmp::Ordering::Less => 1,
                    std::cmp::Ordering::Greater => -1,
                    std::cmp::Ordering::Equal => 0,
                };

                let new_x = (npc_units[i].x as i16 + dir_x) as u8;
                let new_y = (npc_units[i].y as i16 + dir_y) as u8;

                if new_x < MAP_BOUND
                    && new_y < MAP_BOUND
                    && !is_occupied(new_x, new_y, &player.units, npc_units, &player.cities)
                {
                    npc_units[i].x = new_x;
                    npc_units[i].y = new_y;
                } else {
                    msg!(
                        "NPC unit #{} cannot move to position ({}, {})",
                        npc_units[i].unit_id,
                        new_x,
                        new_y
                    );
                }
            }
        }
    }
    Ok(())
}

fn is_occupied(
    x: u8,
    y: u8,
    player_units: &[Unit],
    npc_units: &[Unit],
    player_cities: &[City],
) -> bool {
    player_units
        .iter()
        .any(|u| u.x == x && u.y == y && u.is_alive)
        || npc_units.iter().any(|u| u.x == x && u.y == y && u.is_alive)
        || player_cities.iter().any(|c| c.x == x && c.y == y)
}

fn required_food_for_growth(population: u32) -> u32 {
    (0.1082 * (population as f64).powf(2.0) + 10.171 * population as f64 + 1.929) as u32
}

pub fn end_turn(ctx: Context<EndTurn>) -> Result<()> {
    // check if the game is over via defeat or victory
    if ctx.accounts.game.defeat || ctx.accounts.game.victory {
        return Ok(());
    }

    // Calculate and update player's resources
    let (gold, wood, stone, iron, horses, science) =
        calculate_resources(&ctx.accounts.player_account);
    ctx.accounts
        .player_account
        .update_resources(gold, wood, stone, iron, horses)?;

    let player_account = &mut ctx.accounts.player_account;

    process_npc_movements_and_attacks(&mut ctx.accounts.npc_account.units, player_account)?;

    for i in 0..player_account.cities.len() {
        let all_controlled_tiles: Vec<TileCoordinate> = player_account
            .cities
            .iter()
            .flat_map(|city| &city.controlled_tiles)
            .cloned()
            .collect();

        let city = &mut player_account.cities[i];

        city.accumulated_food += city.food_yield as i32;

        // Deduct food for population maintenance
        let food_consumption = city.population * 2; // 2 food per citizen
        city.accumulated_food -= food_consumption as i32;

        if city.accumulated_food >= 0 {
            let required_food = required_food_for_growth(city.population);
            if city.accumulated_food as u32 >= required_food && city.population < city.housing {
                city.population += 1;
                city.accumulated_food = 0;
            }
        } else {
            // Handle population decrease due to food shortage
            if city.population > 1 {
                city.population -= 1;
                city.accumulated_food = 0;
            }
        }

        // Auto-healing of cities
        if city.health < 100 {
            city.health = std::cmp::min(city.health + 5, 100);
        }

        // growth city
        city.growth_points += city.population; // 1 citizen growth points generated
        let points_need = 10.0 + (6.0 * city.level as f32).powf(1.3);

        if city.growth_points as f32 >= points_need {
            city.growth_points = 0;
            city.level += 1;

            let adjacent_tiles = find_adjacent_tiles(&city.controlled_tiles, &all_controlled_tiles);

            if !adjacent_tiles.is_empty() {
                let clock = Clock::get()?;
                let random_factor = clock.unix_timestamp as usize % adjacent_tiles.len();
                city.controlled_tiles.push(adjacent_tiles[random_factor]);
            };
        }
    }

    // The healing should happen only after NPC attacks
    // Reset units' movement range & heal if needed
    heal_units_and_reset_movement_range(&mut ctx.accounts.player_account.units);

    // Process the production queues of each city for the player
    let game_key = ctx.accounts.game.key();
    process_production_queues(&mut ctx.accounts.player_account, game_key)?;

    // Check research progress
    ctx.accounts.player_account.add_research_points(science)?;

    // Retain only alive units in the game
    ctx.accounts.player_account.units.retain(|u| u.is_alive);
    ctx.accounts.player_account.cities.retain(|c| c.health > 0);
    ctx.accounts.npc_account.units.retain(|u| u.is_alive);
    ctx.accounts.npc_account.cities.retain(|c| c.health > 0);

    // spawn new NPC units every 20 turns
    if ctx.accounts.game.turn % 20 == 0 {
        let clock = Clock::get()?;
        let random_factor = clock.unix_timestamp % 10;

        // Temporary vector to store new units.
        let mut new_units = Vec::new();
        let mut next_npc_id = ctx.accounts.npc_account.next_unit_id;

        for city in &ctx.accounts.npc_account.cities {
            let unit_type = if ctx.accounts.game.turn >= 100 {
                // Turn >= 100: 0-4 Swordsman, 5-9 Horseman
                if random_factor < 5 {
                    UnitType::Swordsman
                } else {
                    UnitType::Horseman
                }
            } else {
                // Turn < 100: 0-4 Warrior, 5-9 Archer
                if random_factor < 5 {
                    UnitType::Warrior
                } else {
                    UnitType::Archer
                }
            };

            let new_unit = Unit::new(
                next_npc_id,
                ctx.accounts.npc_account.player,
                ctx.accounts.game.key(),
                unit_type,
                city.x,
                city.y,
            );
            new_units.push(new_unit);
            next_npc_id += 1;
        }
        ctx.accounts.npc_account.units.append(&mut new_units);
        ctx.accounts.npc_account.next_unit_id = next_npc_id;
    }
    // if player has no units and no cities set game defeat to true
    if ctx.accounts.player_account.units.is_empty() && ctx.accounts.player_account.cities.is_empty()
    {
        ctx.accounts.game.defeat = true;
    } else if ctx.accounts.npc_account.units.is_empty()
        && ctx.accounts.npc_account.cities.is_empty()
    {
        ctx.accounts.game.victory = true;
    }

    ctx.accounts.game.turn += 1;
    Ok(())
}

pub fn close_game(_ctx: Context<Close>) -> Result<()> {
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
    pub game: Box<Account<'info, Game>>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EndTurn<'info> {
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
pub struct Close<'info> {
    #[account(mut, close = player, has_one = player)]
    game: Box<Account<'info, Game>>,
    #[account(mut, close = player, has_one = player)]
    player_account: Account<'info, Player>,
    #[account(mut, close = player, has_one = player)]
    npc_account: Account<'info, Npc>,
    #[account(mut)]
    player: Signer<'info>,
}
