use crate::consts::*;

pub fn get_new_exp(current_level: u8, current_exp: u8, exp_amount: u8) -> u8 {
    // Adjust current_level to match array indexing and check if max level was reached
    let adjusted_level = current_level.saturating_sub(1);
    if adjusted_level as usize >= EXP_THRESHOLDS.len() {
        return current_exp;
    }

    let max_exp = EXP_THRESHOLDS[adjusted_level as usize];
    let new_exp = current_exp.saturating_add(exp_amount);

    if new_exp >= max_exp {
        // Cap the experience at the max for the current level
        max_exp
    } else {
        // Otherwise, just add the new experience
        new_exp
    }
}
