use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

pub enum GameInstruction {
    InitGameAccount { map: [u8; 400] },
    InitPlayerAccount,
}

#[derive(BorshDeserialize)]
struct GamePayload {
    map: [u8; 400],
}

impl GameInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        Ok(match variant {
            0 => {
                let payload = GamePayload::try_from_slice(rest).unwrap();
                Self::InitGameAccount { map: payload.map }
            }
            1 => Self::InitPlayerAccount,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
