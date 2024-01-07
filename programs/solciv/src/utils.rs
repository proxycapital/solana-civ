use crate::consts::*;

pub fn get_new_exp(current_level: u8, current_exp: u8, exp_amount: u8) -> u8 {
    if current_level as usize >= EXP_THRESHOLDS.len() {
        return 0;
    }

    let max_exp = EXP_THRESHOLDS[current_level as usize];
    let new_exp = current_exp.saturating_add(exp_amount);

    if new_exp >= max_exp {
        // Cap the experience at the max for the current level
        max_exp
    } else {
        // Otherwise, just add the new experience
        new_exp
    }
}
