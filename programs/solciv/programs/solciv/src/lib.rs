use anchor_lang::prelude::*;

declare_id!("GoiXQMoEhhLM8MSbfUFhHz4punJqXNHEQh6ysegmuHJz");

#[program]
pub mod solciv {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
