use borsh::BorshSerialize;
use crate::instruction::GameInstruction;
use crate::state::{Balances, GameAccountState};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh0_10::try_from_slice_unchecked,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = GameInstruction::unpack(instruction_data)?;
    match instruction {
        GameInstruction::InitGameAccount { map } => initialize_game(program_id, accounts, &map)?,
        GameInstruction::InitPlayerAccount => initialize_player(program_id, accounts)?,
        // _ => Err(ProgramError::InvalidInstructionData),
    };

    Ok(())
}

pub fn initialize_game(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    map: &[u8; 400],
) -> ProgramResult {
    msg!("Initializing new game");

    let account_info_iter = &mut accounts.iter();

    let player_account = next_account_info(account_info_iter)?;
    let game_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    let (game_account_pda, bump_seed) = Pubkey::find_program_address(
        &[
            GameAccountState::DISCRIMINATOR.as_ref(),
            player_account.key.as_ref(),
        ],
        program_id,
    );

    if game_account_pda != *game_account.key {
        msg!("Invalid seeds for PDA");
        return Err(ProgramError::InvalidSeeds);
    }

    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(GameAccountState::get_account_size());

    invoke_signed(
        &system_instruction::create_account(
            player_account.key,
            game_account.key,
            rent_lamports,
            GameAccountState::get_account_size() as u64,
            program_id,
        ),
        &[
            player_account.clone(),
            game_account.clone(),
            system_program.clone(),
        ],
        &[&[
            GameAccountState::DISCRIMINATOR.as_ref(),
            player_account.key.as_ref(),
            &[bump_seed],
        ]],
    )?;

    msg!("PDA created: {}", game_account_pda);

    msg!("Unpacking game state account");
    let mut game_data =
        try_from_slice_unchecked::<GameAccountState>(&game_account.data.borrow()).unwrap();

    msg!("Checking if game account is already initialized");
    if game_data.is_initialized {
        msg!("Account already initialized");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    game_data.map = *map;
    game_data.creator = *player_account.key;
    game_data.is_initialized = true;

    msg!("Serializing game data");
    game_data.serialize(&mut &mut game_account.data.borrow_mut()[..])?;

    Ok(())
}

pub fn initialize_player(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    msg!("Initializing new player with balances");

    let account_info_iter = &mut accounts.iter();

    let player_account = next_account_info(account_info_iter)?;
    let player_balances_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    let (player_balances_pda, bump_seed) = Pubkey::find_program_address(
        &[
            Balances::DISCRIMINATOR.as_ref(),
            player_account.key.as_ref(),
        ],
        program_id,
    );

    if player_balances_pda != *player_balances_account.key {
        msg!("Invalid seeds for PDA");
        return Err(ProgramError::InvalidSeeds);
    }

    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(Balances::SIZE);

    invoke_signed(
        &system_instruction::create_account(
            player_account.key,
            player_balances_account.key,
            rent_lamports,
            Balances::SIZE as u64,
            program_id,
        ),
        &[
            player_account.clone(),
            player_balances_account.clone(),
            system_program.clone(),
        ],
        &[&[
            Balances::DISCRIMINATOR.as_ref(),
            player_account.key.as_ref(),
            &[bump_seed],
        ]],
    )?;

    msg!("PDA created: {}", player_balances_account.key);

    msg!("Unpacking balances account");
    let mut balances_data =
        try_from_slice_unchecked::<Balances>(&player_balances_account.data.borrow()).unwrap();

    msg!("Checking if balances account is already initialized");
    if balances_data.is_initialized {
        msg!("Account already initialized");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    balances_data.gold = 100 as u64;
    balances_data.food = 50 as u64;
    balances_data.lumber = 20 as u64;
    balances_data.is_initialized = true;

    msg!("Serializing balances data");
    balances_data.serialize(&mut &mut player_balances_account.data.borrow_mut()[..])?;

    Ok(())
}
