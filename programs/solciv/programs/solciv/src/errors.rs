use anchor_lang::error_code;

#[error_code]
pub enum UnitError {
  #[msg("Unit with given ID not found")]
  UnitNotFound,
  
  #[msg("Unit cannot move this turn")]
  CannotMove,
  
  #[msg("Out of movement range")]
  OutOfMovementRange,
  
  #[msg("Out of map bounds")]
  OutOfMapBounds,
  
  #[msg("Tile is occupied by another unit")]
  TileOccupied,
}