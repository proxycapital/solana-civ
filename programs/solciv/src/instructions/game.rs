use crate::state::*;
use crate::consts::*;
use anchor_lang::prelude::*;

pub fn initialize_game(ctx: Context<InitializeGame>, map: [u8; 400]) -> Result<()> {
  ctx.accounts.game.player = ctx.accounts.player.key().clone();
  ctx.accounts.game.turn = 1;
  ctx.accounts.game.map = map;
  ctx.accounts.game.defeat = false;
  ctx.accounts.game.victory = false;

  msg!("Game created!");

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

  // spawn new NPC units every 15 turns
  if ctx.accounts.game.turn % 15 == 0 {
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