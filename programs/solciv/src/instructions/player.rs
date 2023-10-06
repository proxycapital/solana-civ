use crate::state::*;
use crate::consts::*;
use anchor_lang::prelude::*;

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