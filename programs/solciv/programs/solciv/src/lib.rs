use anchor_lang::prelude::*;

declare_id!("GoiXQMoEhhLM8MSbfUFhHz4punJqXNHEQh6ysegmuHJz");

const MAX_UNITS: u8 = 10;

#[program]
pub mod solciv {
    use super::*;

    pub fn initialize_game(ctx: Context<InitializeGame>, map: [u8; 400]) -> Result<()> {
        ctx.accounts.game.player = ctx.accounts.player.key().clone();
        ctx.accounts.game.map = map;
    
        msg!("Game created!");
    
        Ok(())
    }

    pub fn initialize_player(ctx: Context<InitializePlayer>) -> Result<()> {
        ctx.accounts.player_account.game = ctx.accounts.game.key().clone();
        ctx.accounts.player_account.player = ctx.accounts.player.key().clone();
        ctx.accounts.player_account.points = 0;
        ctx.accounts.player_account.resources = Resources {
            gold: 0,
            food: 10,
            wood: 5,
            stone: 0,
            iron: 0,
        };
        ctx.accounts.player_account.units = vec![
            Unit {
                unit_id: 0,
                player: ctx.accounts.player.key().clone(),
                game: ctx.accounts.game.key().clone(),
                unit_type: UnitType::Settler,
                x: 2,
                y: 2,
                attack: 0,
                health: 100,
                movement_range: 2,
                remaining_actions: 1,
                is_alive: true,
            },
            Unit {
                unit_id: 1,
                player: ctx.accounts.player.key().clone(),
                game: ctx.accounts.game.key().clone(),
                unit_type: UnitType::Builder,
                x: 3,
                y: 2,
                attack: 14,
                health: 100,
                movement_range: 2,
                remaining_actions: 1,
                is_alive: true,
            },
            Unit {
                unit_id: 2,
                player: ctx.accounts.player.key().clone(),
                game: ctx.accounts.game.key().clone(),
                unit_type: UnitType::Warrior,
                x: 2,
                y: 3,
                attack: 14,
                health: 100,
                movement_range: 2,
                remaining_actions: 1,
                is_alive: true,
            }
        ];
    
        msg!("Player created!");
    
        Ok(())
    }
}

#[account]
pub struct Game {
    pub player: Pubkey,
    pub map: [u8; 400],
}

#[derive(Accounts)]
pub struct InitializeGame<'info> {
    #[account(
        init, 
        seeds=[b"GAME", player.key().as_ref()],
        bump,
        payer = player, 
        space = std::mem::size_of::<Game>()+ 8
    )]
    pub game: Account<'info, Game>,

    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Player {
    pub game: Pubkey,
    pub player: Pubkey,
    pub points: u64,
    pub units: Vec<Unit>,
    pub resources: Resources,
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
        space = std::mem::size_of::<Player>() + std::mem::size_of::<Unit>() * MAX_UNITS as usize + std::mem::size_of::<Resources>() + 8)
    ]
    pub player_account: Box<Account<'info, Player>>,

    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone)]
pub struct Unit {
    pub unit_id: u8,
    pub player: Pubkey,
    pub game: Pubkey,
    pub unit_type: UnitType,
    pub x: u8,
    pub y: u8,
    pub attack: u8,
    pub health: u8,
    pub movement_range: u8,
    pub remaining_actions: u8,
    pub is_alive: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone)]
pub enum UnitType {
    Settler,
    Builder,
    Warrior,
    Archer,
    Swordsman,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone)]
pub struct Resources {
    pub gold: u32,
    pub food: u32,
    pub wood: u32,
    pub stone: u32,
    pub iron: u32,
}