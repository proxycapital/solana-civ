use crate::consts::*;

pub fn calculate_exp_amount(current_level: u8, current_exp: u8, exp_amount: u8) -> u8 {
  const MAX_EXP_LEVELS: [u8; 3] = [EXP_FOR_LEVEL_1, EXP_FOR_LEVEL_2, EXP_FOR_LEVEL_3];
  
  let max_exp = MAX_EXP_LEVELS[current_level as usize];
  let new_exp = current_exp + exp_amount;

  if new_exp >= max_exp - exp_amount {
      max_exp
  } else {
      new_exp
  }
}