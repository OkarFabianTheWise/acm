use anchor_lang::prelude::*;

declare_id!("CvCD3RzxLJzMwW1VoSPgUG7Koy7EuRvftmS4Ao8vXah7");

#[program]
pub mod acm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
