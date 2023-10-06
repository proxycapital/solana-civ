use crate::errors::*;
use crate::state::*;
use anchor_lang::prelude::*;

pub fn start_research(ctx: Context<StartResearch>, technology_type: TechnologyType) -> Result<()> {
    let player_account = &mut ctx.accounts.player_account;

    // Ensure the research hasn't already been started or completed.
    if player_account
        .researched_technologies
        .contains(&technology_type)
    {
        return err!(ResearchError::ResearchAlreadyCompleted);
    }

    player_account.start_research(technology_type)?;

    msg!("Research started!");

    Ok(())
}

#[derive(Accounts)]
pub struct StartResearch<'info> {
    #[account(mut)]
    pub player_account: Account<'info, Player>,
    #[account(mut)]
    pub player: Signer<'info>,
}
