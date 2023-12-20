use crate::consts::*;
use crate::errors::*;
use crate::state::*;
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

    // Settler has special conditions as it consumes 1 Citizen, so city population should be at least 2
    if let ProductionItem::Unit(UnitType::Settler) = &item {
        if city.population < 2 {
            return err!(CityError::InsufficientPopulationForSettler);
        }
    }

    let maintenance_cost = if let ProductionItem::Unit(unit_type) = &item {
        Unit::get_maintenance_cost(*unit_type)
    } else {
        0
    };
    if maintenance_cost > 0 && player_account.resources.gold < 0 {
        return err!(CityError::InsufficientGoldForMaintenance);
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
                    // @todo: this now requires Population > 1
                    // @todo: decrease population when the settler is recruited
                    0
                },
                UnitType::Swordsman => Unit::get_resource_cost(*unit_type),
                UnitType::Horseman => Unit::get_resource_cost(*unit_type),
                _ => 0, // No resource cost for other unit types
            }
        }
    };

    // Perform the necessary deductions
    if total_cost > 0 {
        let resource_type = match &item {
            ProductionItem::Unit(UnitType::Swordsman) => &mut player_account.resources.iron,
            ProductionItem::Unit(UnitType::Horseman) => &mut player_account.resources.horses,
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

pub fn repair_wall(ctx: Context<RepairWall>, city_id: u32) -> Result<()> {
    let player_account: &mut Account<'_, Player> = &mut ctx.accounts.player_account;

    // Encapsulate the logic in a separate block to release the borrow after use.
    let (max_wall_hp, cost) = {
        let city = player_account
            .cities
            .iter()
            .find(|city| city.city_id == city_id)
            .ok_or(CityError::CityNotFound)?;

        let max_wall_hp = if city.buildings.contains(&BuildingType::WallIndustrial) {
            200
        } else if city.buildings.contains(&BuildingType::WallRenaissance) {
            150
        } else if city.buildings.contains(&BuildingType::WallMedieval) {
            100
        } else if city.buildings.contains(&BuildingType::Wall) {
            50
        } else {
            return err!(CityError::NoWall);
        };

        // 1 hp to repair = 2 wood + 2 stone
        let cost = (max_wall_hp - city.wall_health) * 2;
        (max_wall_hp, cost)
    };

    // Check and deduct the player's wood balance.
    if player_account.resources.wood < cost {
        return err!(CityError::InsufficientWood);
    }

    // Check and deduct the player's wood balance.
    if player_account.resources.stone < cost {
        return err!(CityError::InsufficientStone);
    }

    player_account.resources.wood -= cost;
    player_account.resources.stone -= cost;

    let city = player_account
        .cities
        .iter_mut()
        .find(|city| city.city_id == city_id)
        .ok_or(CityError::CityNotFound)?;

    // Set city health to max
    city.wall_health = max_wall_hp;

    Ok(())
}

pub fn purchase_with_gold(
    ctx: Context<PurchaseWithGold>,
    city_id: u32,
    item: ProductionItem,
) -> Result<()> {
    let player_account = &mut ctx.accounts.player_account;
    let next_unit_id = player_account.next_unit_id;
    let player = player_account.player;
    let game = player_account.game;
    // Determine the cost of the unit/building.
    let cost = match &item {
        ProductionItem::Building(building_type) => {
            BuildingType::get_gold_cost(*building_type) as i32
        }
        ProductionItem::Unit(unit_type) => Unit::get_gold_cost(*unit_type) as i32,
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

    // Settler has special conditions as it consumes 1 Citizen, so city population should be at least 2
    if let ProductionItem::Unit(UnitType::Settler) = &item {
        if city.population < 2 {
            return err!(CityError::InsufficientPopulationForSettler);
        } else {
            city.population -= 1;
        }
    }

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
pub struct RepairWall<'info> {
    #[account(mut, has_one = player)]
    pub player_account: Account<'info, Player>,
    #[account(mut)]
    pub player: Signer<'info>,
}

#[derive(Accounts)]
pub struct AddToProductionQueue<'info> {
    #[account(mut, has_one = player)]
    pub player_account: Account<'info, Player>,
    #[account(mut)]
    pub player: Signer<'info>,
}

#[derive(Accounts)]
pub struct RemoveFromProductionQueue<'info> {
    #[account(mut, has_one = player)]
    pub player_account: Account<'info, Player>,
    #[account(mut)]
    pub player: Signer<'info>,
}

#[derive(Accounts)]
pub struct PurchaseWithGold<'info> {
    #[account(mut, has_one = player)]
    pub player_account: Account<'info, Player>,
    #[account(mut)]
    pub player: Signer<'info>,
}
