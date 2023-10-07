use crate::errors::GameError;
use crate::state::Player;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

pub fn mint_gems(ctx: Context<MintGems>) -> Result<()> {
    if ctx.accounts.player_account.resources.gems == 0 {
        return err!(GameError::NotEnoughGems);
    }
    let seeds = &["mint".as_bytes(), &[*ctx.bumps.get("mint").unwrap()]];
    let signer = [&seeds[..]];
    let amount = ctx.accounts.player_account.resources.gems as u64 * 1_000_000_000;
    ctx.accounts.player_account.resources.gems = 0;
    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                authority: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.destination.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
            },
            &signer,
        ),
        amount,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct MintGems<'info> {
    #[account(
        mut,
        seeds = [b"mint"],
        bump,
        mint::authority = mint,
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = player,
        associated_token::mint = mint,
        associated_token::authority = owner,
    )]
    pub destination: Account<'info, TokenAccount>,
    /// CHECK:
    pub owner: AccountInfo<'info>,
    #[account(mut)]
    pub player_account: Account<'info, Player>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
