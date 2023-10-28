use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone)]
pub struct Resources {
    pub gold: i32,
    pub food: u32,
    pub wood: u32,
    pub stone: u32,
    pub iron: u32,
    pub gems: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pub x: u8,
    pub y: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum TileType {
    LumberMill,
    StoneQuarry,
    Farm,
    IronMine,
}

impl Tile {
    pub fn new(tile_type: TileType, x: u8, y: u8) -> Self {
        Self { tile_type, x, y }
    }
}
