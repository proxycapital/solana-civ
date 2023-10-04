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

    #[msg("The provided unit cannot perform this action")]
    InvalidUnitType,

    #[msg("The provided unit is not at the required coordinates")]
    UnitWrongPosition,

    #[msg("The provided unit cannot attack")]
    InvalidAttack,

    #[msg("The provided unit is out of attack range")]
    OutOfAttackRange,

    #[msg("No movement points left this turn")]
    NoMovementPoints,
}

#[error_code]
pub enum BuildingError {
    #[msg("Tile is occupied by another construction")]
    TileOccupied,
}

#[error_code]
pub enum TileError {
    #[msg("Tile is not upgradeable")]
    NotUpgradeable,

    #[msg("Tile is occupied by another construction")]
    TileOccupied,
}

#[error_code]
pub enum CityError {
    #[msg("Production queue is full")]
    QueueFull,

    #[msg("Building already exists")]
    BuildingAlreadyExists,

    #[msg("City not found")]
    CityNotFound,

    #[msg("Counstruction is already in progress")]
    AlreadyQueued,

    #[msg("Not enough resources")]
    InsufficientResources,

    #[msg("Invalid production item")]
    InvalidItem,

    #[msg("Item not found in the production queue of the city")]
    QueueItemNotFound,

    #[msg("Not enough gold")]
    InsufficientGold,
}
