use crate::errors::*;
use crate::state::*;
use anchor_lang::prelude::*;

const MAX_UNITS: u8 = 20;
const MAX_CITIES: u8 = 20;
const MAX_BUILDINGS: u8 = 20;
const MAX_UPGRADED_TILES: u8 = 100;
const MAX_PRODUCTION_QUEUE: u8 = 5;
const MAP_BOUND: u8 = 20;

pub fn initialize_game(ctx: Context<InitializeGame>, map: [u8; 400]) -> Result<()> {
    ctx.accounts.game.player = ctx.accounts.player.key().clone();
    ctx.accounts.game.turn = 1;
    ctx.accounts.game.map = map;
    ctx.accounts.game.defeat = false;
    ctx.accounts.game.victory = false;

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
        food: 0,
        wood: 0,
        stone: 0,
        iron: 0,
    };
    // player starts with 3 units: Settler, Builder, Warrior
    ctx.accounts.player_account.units = vec![
        Unit::new(
            0,
            ctx.accounts.player.key().clone(),
            ctx.accounts.game.key().clone(),
            UnitType::Settler,
            2,
            2,
        ),
        Unit::new(
            1,
            ctx.accounts.player.key().clone(),
            ctx.accounts.game.key().clone(),
            UnitType::Builder,
            3,
            2,
        ),
        Unit::new(
            2,
            ctx.accounts.player.key().clone(),
            ctx.accounts.game.key().clone(),
            UnitType::Warrior,
            2,
            3,
        ),
    ];
    ctx.accounts.player_account.next_unit_id = 3;

    ctx.accounts.player_account.researched_technologies = vec![];

    msg!("Player created!");

    Ok(())
}

pub fn initialize_npc(ctx: Context<InitializeNpc>) -> Result<()> {
    ctx.accounts.npc_account.game = ctx.accounts.game.key().clone();
    ctx.accounts.npc_account.player = ctx.accounts.player.key().clone();
    ctx.accounts.npc_account.next_city_id = 0;
    ctx.accounts.npc_account.next_unit_id = 0;
    ctx.accounts.game.npc = ctx.accounts.npc_account.key().clone();

    ctx.accounts.npc_account.cities = vec![
        City::new(
            0,
            ctx.accounts.npc_account.player.clone(),
            ctx.accounts.game.key().clone(),
            2,
            17,
            "Barbarian Village".to_string(),
        ),
        City::new(
            1,
            ctx.accounts.npc_account.player.clone(),
            ctx.accounts.game.key().clone(),
            17,
            17,
            "Barbarian Village".to_string(),
        ),
    ];

    // Initialize units for the NPC.
    ctx.accounts.npc_account.units = vec![Unit::new(
        0,
        ctx.accounts.npc_account.key().clone(),
        ctx.accounts.game.key().clone(),
        UnitType::Warrior,
        16,
        17,
    )];
    ctx.accounts.npc_account.next_unit_id = 1;
    ctx.accounts.npc_account.next_city_id = 2;

    msg!("NPC created!");

    Ok(())
}

// #[access_control(MoveUnit::validate_unit_move(&ctx, unit_id, x, y))]
pub fn move_unit(ctx: Context<MoveUnit>, unit_id: u32, x: u8, y: u8) -> Result<()> {
    let unit = ctx
        .accounts
        .player_account
        .units
        .iter()
        .find(|u| u.unit_id == unit_id)
        .ok_or(UnitError::UnitNotFound)?;

    // Check if the tile is within the map bounds
    if x >= 20 || y >= 20 {
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

    Ok(())
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

    // Initialize the new City.
    let new_city = City::new(
        ctx.accounts.player_account.next_city_id,
        ctx.accounts.player_account.player,
        ctx.accounts.game.key(),
        x,
        y,
        name,
    );

    ctx.accounts.player_account.cities.push(new_city);

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
    let map_idx = (y as usize) * 20 + x as usize;
    match ctx.accounts.game.map[map_idx] {
        1 | 2 | 5 | 6 => {} // allowable tile types
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

    // Initialize the new Tile and push it to player_account tiles vector.
    let tile_type = match ctx.accounts.game.map[map_idx] {
        1 => TileType::IronMine,
        2 => TileType::LumberMill,
        5 => TileType::StoneQuarry,
        6 => TileType::Farm,
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

pub fn start_research(ctx: Context<StartResearch>, technology_type: TechnologyType) -> Result<()> {
    let player_account = &mut ctx.accounts.player_account;

    // Ensure the research hasn't already been started or completed.
    if player_account
        .researched_technologies
        .contains(&technology_type)
    {
        return err!(ResearchError::ResearchAlreadyCompleted);
    }

    player_account.start_research(technology_type)?;

    msg!("Research started!");

    Ok(())
}

pub fn add_to_production_queue(
    ctx: Context<AddToProductionQueue>,
    city_id: u32,
    item: ProductionItem,
) -> Result<()> {
    let player_account = &mut ctx.accounts.player_account;

    let city = player_account
        .cities
        .iter()
        .find(|city| city.city_id == city_id)
        .ok_or(CityError::CityNotFound)?;

    if (city.production_queue.len() as u8) >= MAX_PRODUCTION_QUEUE {
        return err!(CityError::QueueFull);
    }

    let total_cost = match &item {
        ProductionItem::Building(building_type) => {
            if !building_type.can_construct(&player_account.researched_technologies) {
                return err!(CityError::TechnologyNotResearched);
            }
            if city.buildings.contains(building_type) {
                return err!(CityError::BuildingAlreadyExists);
            }
            if city.production_queue.contains(&item) {
                return err!(CityError::AlreadyQueued);
            }
            0 // No cost for building types
        }
        ProductionItem::Unit(unit_type) => {
            if !unit_type.can_recruit(&player_account.researched_technologies) {
                return err!(CityError::TechnologyNotResearched);
            }
            match unit_type {
                UnitType::Settler => {
                    let settlers_count = player_account.cities.len() as u32
                        + player_account
                            .units
                            .iter()
                            .filter(|unit| matches!(unit.unit_type, UnitType::Settler))
                            .count() as u32;
                    settlers_count * Unit::get_resource_cost(*unit_type)
                }
                UnitType::Swordsman => Unit::get_resource_cost(*unit_type),
                _ => 0, // No resource cost for other unit types
            }
        }
    };

    // Perform the necessary deductions
    if total_cost > 0 {
        let resource_type = match &item {
            ProductionItem::Unit(UnitType::Settler) => &mut player_account.resources.food,
            ProductionItem::Unit(UnitType::Swordsman) => &mut player_account.resources.iron,
            // can this really happen?
            _ => return err!(CityError::InvalidItem),
        };

        if *resource_type < total_cost {
            return err!(CityError::InsufficientResources);
        }
        *resource_type -= total_cost;
    }

    let city = player_account
        .cities
        .iter_mut()
        .find(|city| city.city_id == city_id)
        .ok_or(CityError::CityNotFound)?;

    city.production_queue.push(item);

    Ok(())
}

pub fn remove_from_production_queue(
    ctx: Context<RemoveFromProductionQueue>,
    city_id: u32,
    index: u8,
) -> Result<()> {
    let player_account = &mut ctx.accounts.player_account;

    let city = player_account
        .cities
        .iter_mut()
        .find(|city| city.city_id == city_id)
        .ok_or(CityError::CityNotFound)?;

    if index >= MAX_PRODUCTION_QUEUE {
        return err!(CityError::QueueItemNotFound);
    }

    if city.production_queue.get(index as usize).is_none() {
        return err!(CityError::QueueItemNotFound);
    }

    // @todo: refund the resources if applicable

    // Remove the item from the production queue.
    city.production_queue.remove(index as usize);

    Ok(())
}

pub fn purchase_with_gold(
    ctx: Context<PurchaseWithGold>,
    city_id: u32,
    item: ProductionItem,
) -> Result<()> {
    let player_account = &mut ctx.accounts.player_account;
    let next_unit_id = player_account.next_unit_id;
    let player = player_account.player.clone();
    let game = player_account.game.clone();
    // Determine the cost of the unit/building.
    let cost = match &item {
        ProductionItem::Building(building_type) => BuildingType::get_gold_cost(*building_type),
        ProductionItem::Unit(unit_type) => Unit::get_gold_cost(*unit_type),
    };

    // Check the player's gold balance.
    if player_account.resources.gold < cost {
        return err!(CityError::InsufficientGold);
    }

    let researched_technologies = player_account.researched_technologies.clone();

    // Deduct the cost from the player's gold balance.
    player_account.resources.gold -= cost;

    // Find the city by city_id.
    let city = player_account
        .cities
        .iter_mut()
        .find(|city| city.city_id == city_id)
        .ok_or(CityError::CityNotFound)?;

    // Add the unit/building to the player's assets.
    match &item {
        ProductionItem::Building(building_type) => {
            // Check if the technology is unlocked
            if !building_type.can_construct(&researched_technologies) {
                return err!(CityError::TechnologyNotResearched);
            }
            // Check if the building already exists in the city.
            if city.buildings.contains(building_type) {
                return err!(CityError::BuildingAlreadyExists);
            }

            // Check if the building is in the city's production_queue.
            // Remove, if so.
            if let Some(index) = city.production_queue.iter().position(|&i| i == item) {
                city.production_queue.remove(index);
            }

            city.construct_building(*building_type)?;
        }
        ProductionItem::Unit(unit_type) => {
            if !unit_type.can_recruit(&researched_technologies) {
                return err!(CityError::TechnologyNotResearched);
            }
            let unit = Unit::new(next_unit_id, player, game, *unit_type, city.x, city.y);
            player_account.units.push(unit);
            player_account.next_unit_id += 1;
        }
    }

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

    attacker.attack_unit(defender)?;

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

    ctx.accounts.player_account.units.retain(|u| u.is_alive);
    ctx.accounts.npc_account.cities.retain(|c| c.health > 0);

    Ok(())
}

fn reset_units_movement_range(units: &mut [Unit]) {
    for unit in units.iter_mut().filter(|u| u.is_alive) {
        unit.movement_range = Unit::get_base_movement_range(unit.unit_type);
    }
}

fn calculate_resources(player_account: &Player) -> (u32, u32, u32, u32, u32, u32) {
    // Calculate resources yielded by cities and tiles.
    // This function will return a tuple (gold, food, wood, stone).
    let mut resources = (0, 0, 0, 0, 0, 0);

    for city in &player_account.cities {
        resources.0 += city.gold_yield;
        resources.1 += city.food_yield;
        resources.5 += city.science_yield;
    }

    for tile in &player_account.tiles {
        match tile.tile_type {
            TileType::LumberMill => resources.2 += 2,
            TileType::StoneQuarry => resources.3 += 2,
            TileType::Farm => resources.1 += 2,
            TileType::IronMine => resources.4 += 2,
        }
    }

    resources
}

fn process_production_queues(player_account: &mut Player, game_key: Pubkey) -> Result<()> {
    let mut new_units = Vec::new();
    let mut next_unit_id = player_account.next_unit_id;
    let player = player_account.player.clone();

    for city in &mut player_account.cities {
        if let Some(item) = city.production_queue.first().cloned() {
            let cost = match item {
                ProductionItem::Unit(unit_type) => Unit::get_base_stats(unit_type).5,
                ProductionItem::Building(building_type) => {
                    BuildingType::get_base_stats(building_type).0
                }
            };

            if city.accumulated_production >= cost {
                // Production completed
                match item {
                    ProductionItem::Unit(unit_type) => {
                        // Create a new unit and add it to the player's units
                        let new_unit = Unit::new(
                            next_unit_id,
                            player.clone(),
                            game_key.clone(),
                            unit_type,
                            city.x,
                            city.y,
                        );
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
            } else {
                // Increment the accumulated production by the city's production yield
                city.accumulated_production += city.production_yield;
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
                if let Some(player_unit) = player
                    .units
                    .iter_mut()
                    .find(|u| u.x == target_x && u.y == target_y && u.is_alive)
                {
                    npc_units[i].attack_unit(player_unit)?;
                } else if let Some(player_city) = player
                    .cities
                    .iter_mut()
                    .find(|c| c.x == target_x && c.y == target_y && c.health > 0)
                {
                    npc_units[i].attack_city(player_city)?;
                }
            } else {
                let dir_x = if npc_units[i].x < target_x {
                    1
                } else if npc_units[i].x > target_x {
                    -1
                } else {
                    0
                };
                let dir_y = if npc_units[i].y < target_y {
                    1
                } else if npc_units[i].y > target_y {
                    -1
                } else {
                    0
                };

                let new_x = (npc_units[i].x as i16 + dir_x) as u8;
                let new_y = (npc_units[i].y as i16 + dir_y) as u8;

                if new_x < MAP_BOUND
                    && new_y < MAP_BOUND
                    && !is_occupied(new_x, new_y, &player.units, &npc_units, &player.cities)
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
    player_units: &Vec<Unit>,
    npc_units: &Vec<Unit>,
    player_cities: &Vec<City>,
) -> bool {
    player_units
        .iter()
        .any(|u| u.x == x && u.y == y && u.is_alive)
        || npc_units.iter().any(|u| u.x == x && u.y == y && u.is_alive)
        || player_cities.iter().any(|c| c.x == x && c.y == y)
}

pub fn end_turn(ctx: Context<EndTurn>) -> Result<()> {
    // check if the game is over via defeat or victory
    if ctx.accounts.game.defeat || ctx.accounts.game.victory {
        return Ok(());
    }
    // Reset units' movement range
    reset_units_movement_range(&mut ctx.accounts.player_account.units);

    // Calculate and update player's resources
    let (gold, food, wood, stone, iron, science) =
        calculate_resources(&ctx.accounts.player_account);
    ctx.accounts
        .player_account
        .update_resources(gold, food, wood, stone, iron)?;

    let player_account = &mut ctx.accounts.player_account;

    process_npc_movements_and_attacks(&mut ctx.accounts.npc_account.units, player_account)?;

    // Process the production queues of each city for the player
    let game_key = ctx.accounts.game.key().clone();
    process_production_queues(&mut ctx.accounts.player_account, game_key)?;

    // Check research progress
    ctx.accounts.player_account.add_research_points(science)?;

    // Retain only alive units in the game
    ctx.accounts.player_account.units.retain(|u| u.is_alive);
    ctx.accounts.player_account.cities.retain(|c| c.health > 0);
    ctx.accounts.npc_account.units.retain(|u| u.is_alive);
    ctx.accounts.npc_account.cities.retain(|c| c.health > 0);

    // spawn new NPC units every 7 turns
    if ctx.accounts.game.turn % 7 == 0 {
        let clock = Clock::get()?;
        let random_factor = clock.unix_timestamp % 10;

        // Temporary vector to store new units.
        let mut new_units = Vec::new();
        let mut next_npc_id = ctx.accounts.npc_account.next_unit_id;

        for city in &ctx.accounts.npc_account.cities {
            // 0-4 Warrior, 5-9 Archer
            let unit_type = if random_factor < 5 {
                UnitType::Warrior
            } else {
                UnitType::Archer
            };

            let new_unit = Unit::new(
                next_npc_id,
                ctx.accounts.npc_account.player.clone(),
                ctx.accounts.game.key().clone(),
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
            4 + (15 * MAX_CITIES as usize) +
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
pub struct InitializeNpc<'info> {
    pub game: Account<'info, Game>,

    #[account(
        init,
        seeds=[
            b"NPC",
            game.key().as_ref(),
        ],
        bump,
        payer = player,
        space = std::mem::size_of::<Npc>() +
            4 + (20 * MAX_CITIES as usize) +
            std::mem::size_of::<Unit>() * MAX_UNITS as usize +
            std::mem::size_of::<City>() * MAX_CITIES as usize + 8)
    ]
    pub npc_account: Box<Account<'info, Npc>>,

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
pub struct AddToProductionQueue<'info> {
    #[account(mut)]
    pub player_account: Account<'info, Player>,
    #[account(mut)]
    pub player: Signer<'info>,
}

#[derive(Accounts)]
pub struct RemoveFromProductionQueue<'info> {
    #[account(mut)]
    pub player_account: Account<'info, Player>,
    #[account(mut)]
    pub player: Signer<'info>,
}

#[derive(Accounts)]
pub struct PurchaseWithGold<'info> {
    #[account(mut)]
    pub player_account: Account<'info, Player>,
    #[account(mut)]
    pub player: Signer<'info>,
}

#[derive(Accounts)]
pub struct AttackUnit<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub player_account: Account<'info, Player>,
    #[account(mut)]
    pub npc_account: Account<'info, Npc>,
    #[account(mut)]
    pub player: Signer<'info>,
}

#[derive(Accounts)]
pub struct AttackCity<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub player_account: Account<'info, Player>,
    #[account(mut)]
    pub npc_account: Account<'info, Npc>,
    #[account(mut)]
    pub player: Signer<'info>,
}

#[derive(Accounts)]
pub struct StartResearch<'info> {
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
    pub npc_account: Account<'info, Npc>,
    #[account(mut)]
    pub player: Signer<'info>,
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut, close = player, has_one = player)]
    game: Account<'info, Game>,
    #[account(mut, close = player, has_one = player)]
    player_account: Account<'info, Player>,
    #[account(mut, close = player, has_one = player)]
    npc_account: Account<'info, Npc>,
    #[account(mut)]
    player: Signer<'info>,
}
