use anchor_lang::prelude::*;

declare_id!("6mgv17dd75FTsfR2an6F6VYdRnKzfiVkPw964P2TQYmA");

#[program]
pub mod sol_amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
