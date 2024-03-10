use crate::consts::*;
use crate::state::city_buildings::TileCoordinate;
use crate::state::game_state::Terrain;

pub fn get_new_exp(current_level: u8, current_exp: u8, exp_amount: u8) -> u8 {
    if current_level as usize >= EXP_THRESHOLDS.len() {
        return 0;
    }

    let max_exp = EXP_THRESHOLDS[current_level as usize];
    let new_exp = current_exp.saturating_add(exp_amount);

    if new_exp >= max_exp {
        // Cap the experience at the max for the current level
        max_exp
    } else {
        // Otherwise, just add the new experience
        new_exp
    }
}

pub fn check_is_on_coast(x: u8, y: u8, game_map: &[Terrain; 400]) -> bool {
    let adjacent_tiles = adjacent_tiles(&TileCoordinate { x, y });

    let mut is_on_coast = false;

    for adjacent_tile in adjacent_tiles {
        let map_idx = (adjacent_tile.y as usize) * MAP_BOUND as usize + adjacent_tile.x as usize;
        if game_map[map_idx].terrain == SEA_TERRAIN {
            is_on_coast = true;
            break;
        }
    }

    is_on_coast
}

// return 8 tiles around passed tile as a center
pub fn adjacent_tiles(tile: &TileCoordinate) -> Vec<TileCoordinate> {
    vec![
        // left and write
        TileCoordinate {
            x: tile.x,
            y: tile.y.saturating_sub(1),
        },
        TileCoordinate {
            x: tile.x,
            y: tile.y + 1,
        },
        // top and bottom
        TileCoordinate {
            x: tile.x.saturating_sub(1),
            y: tile.y,
        },
        TileCoordinate {
            x: tile.x + 1,
            y: tile.y,
        },
        // right top and bottom
        TileCoordinate {
            x: tile.x + 1,
            y: tile.y + 1,
        },
        TileCoordinate {
            x: tile.x + 1,
            y: tile.y.saturating_sub(1),
        }, 
        // left top and bottom
        TileCoordinate {
            x: tile.x.saturating_sub(1),
            y: tile.y + 1,
        },
        TileCoordinate {
            x: tile.x.saturating_sub(1),
            y: tile.y.saturating_sub(1),
        }, 
    ]
}


pub fn find_closest_tile_for_blocked_units(
    start_location: TileCoordinate,
    end_location: TileCoordinate,
    game_map: &[Terrain; 400],
    is_naval: bool
) -> Option<TileCoordinate> {
    let adjacent_tiles: Vec<TileCoordinate> = adjacent_tiles(&TileCoordinate { x: start_location.x, y: start_location.y })
        .into_iter()
        .filter(|&adjacent_tile| {
            let map_idx = (adjacent_tile.y as usize) * MAP_BOUND as usize + adjacent_tile.x as usize;

            is_naval == (game_map[map_idx].terrain == SEA_TERRAIN)
        })
        .collect();

    // calculte minimal distance from each valid tile to closest
    let mut min_distances = vec![];

    for (index, adjacent_ground_tile) in adjacent_tiles.iter().enumerate() {
        let min_dist = ((adjacent_ground_tile.x as i16 - end_location.x as i16).pow(2)
            + (adjacent_ground_tile.y as i16 - end_location.y as i16).pow(2))
            as u16;
        min_distances.push((min_dist, index));
    }
    // find first tile with minimal distance
    let min_tile = min_distances.iter().min_by_key(|&(value, _)| value);
    min_tile.map(|(_, index_of_tile)| adjacent_tiles[*index_of_tile].clone())
}