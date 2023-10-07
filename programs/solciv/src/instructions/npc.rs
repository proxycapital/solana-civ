use crate::consts::*;
use crate::state::*;
use anchor_lang::prelude::*;

pub fn initialize_npc(ctx: Context<InitializeNpc>) -> Result<()> {
    ctx.accounts.npc_account.game = ctx.accounts.game.key();
    ctx.accounts.npc_account.player = ctx.accounts.player.key();
    ctx.accounts.npc_account.next_city_id = 0;
    ctx.accounts.npc_account.next_unit_id = 0;
    ctx.accounts.game.npc = ctx.accounts.npc_account.key();

    ctx.accounts.npc_account.cities = vec![
        City::new(
            0,
            ctx.accounts.npc_account.player,
            ctx.accounts.game.key(),
            2,
            17,
            "Barbarian Village".to_string(),
            1000,
        ),
        City::new(
            1,
            ctx.accounts.npc_account.player,
            ctx.accounts.game.key(),
            17,
            17,
            "Barbarian Village".to_string(),
            1000,
        ),
    ];

    // Initialize units for the NPC.
    ctx.accounts.npc_account.units = vec![Unit::new(
        0,
        ctx.accounts.npc_account.key(),
        ctx.accounts.game.key(),
        UnitType::Warrior,
        16,
        17,
    )];
    ctx.accounts.npc_account.next_unit_id = 1;
    ctx.accounts.npc_account.next_city_id = 2;

    msg!("NPC created!");

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeNpc<'info> {
    pub game: Box<Account<'info, Game>>,

    #[account(
        init,
        seeds=[
            b"NPC",
            game.key().as_ref(),
        ],
        bump,
        payer = player,
        space = std::mem::size_of::<Npc>() +
            4 + (20 * MAX_CITIES as usize) +
            std::mem::size_of::<Unit>() * MAX_UNITS as usize +
            std::mem::size_of::<City>() * MAX_CITIES as usize + 8)
    ]
    pub npc_account: Box<Account<'info, Npc>>,

    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}
