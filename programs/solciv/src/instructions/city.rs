use crate::errors::*;
use crate::state::*;
use crate::consts::*;
use anchor_lang::prelude::*;

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