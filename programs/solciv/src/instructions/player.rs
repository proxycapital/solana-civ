use crate::consts::*;
use crate::state::*;
use anchor_lang::prelude::*;

pub fn initialize_player(ctx: Context<InitializePlayer>, position: TileCoordinate) -> Result<()> {
    ctx.accounts.player_account.game = ctx.accounts.game.key();
    ctx.accounts.player_account.player = ctx.accounts.player.key();
    ctx.accounts.player_account.points = 0;
    ctx.accounts.player_account.next_city_id = 0;
    ctx.accounts.player_account.next_unit_id = 0;
    // @todo: consider implementing helper methods for initializing the resources, units or other default things
    ctx.accounts.player_account.resources = Resources {
        gold: 0,
        wood: 0,
        stone: 0,
        iron: 0,
        gems: 0,
        horses: 0,
    };
    // player starts with 3 units: Settler, Builder, Warrior
    ctx.accounts.player_account.units = vec![
        Unit::new(
            0,
            ctx.accounts.player.key(),
            ctx.accounts.game.key(),
            UnitType::Settler,
            position.x,
            position.y,
        ),
        Unit::new(
            1,
            ctx.accounts.player.key(),
            ctx.accounts.game.key(),
            UnitType::Builder,
            position.x + 1,
            position.y,
        ),
        Unit::new(
            2,
            ctx.accounts.player.key(),
            ctx.accounts.game.key(),
            UnitType::Warrior,
            position.x,
            position.y + 1,
        ),
    ];
    ctx.accounts.player_account.next_unit_id = 3;

    ctx.accounts.player_account.researched_technologies = vec![];

    /* Set surrounding tiles to 'discovered' */
    let start_x = position.x.saturating_sub(2).max(0);
    let end_x = position.x.saturating_add(2).min(MAP_BOUND - 1);
    let start_y = position.y.saturating_sub(2).max(0);
    let end_y = position.y.saturating_add(2).min(MAP_BOUND - 1);

    for i in start_x..=end_x {
        for j in start_y..=end_y {
            let index = (j as usize) * MAP_BOUND as usize + i as usize;
            ctx.accounts.game.map[index].discovered = true;
        }
    }

    msg!("Player created!");

    Ok(())
}

#[derive(Accounts)]
pub struct InitializePlayer<'info> {
    #[account(mut)]
    pub game: Box<Account<'info, Game>>,

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
