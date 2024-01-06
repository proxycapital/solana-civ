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

    #[msg("Unit is not damaged")]
    UnitNotDamaged,

    #[msg("Not enough of food to heal the unit")]
    NotEnoughResources,

    #[msg("Max level reached")]
    MaxLevelReached,

    #[msg("Not enought experience to level up unit")]
    NotEnoughExp,
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

    #[msg("Technology is not unlocked")]
    TechnologyNotResearched,

    #[msg("Not enough wood")]
    InsufficientWood,

    #[msg("Not enough stone")]
    InsufficientStone,

    #[msg("Wall not damaged")]
    NotDamagedWall,

    #[msg("No wall in the city")]
    NoWall,

    #[msg("Not enough gold for maintenance")]
    InsufficientGoldForMaintenance,

    #[msg("Not enough citizens to recruit a Settler")]
    InsufficientPopulationForSettler,
}

#[error_code]
pub enum ResearchError {
    #[msg("Invalid research")]
    InvalidResearch,
    #[msg("Research already in progress")]
    AlreadyResearching,
    #[msg("Research already completed")]
    ResearchAlreadyCompleted,
    #[msg("You need to unlock the previous technology first")]
    CannotResearch,
    #[msg("Research not complete")]
    ResearchNotComplete,
    #[msg("No active research")]
    NoActiveResearch,
}

#[error_code]
pub enum GameError {
    #[msg("Not enough gems")]
    NotEnoughGems,
}
