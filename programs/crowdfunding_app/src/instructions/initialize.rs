use anchor_lang::prelude::*;

use crate::constants::ANCHOR_DISCRIMINATOR_SIZE;
use crate::errors::ErrorCode::AlreadyInitialized;
use crate::states::ProgramState;

pub fn initialize(ctx: Context<InitializeCtx>) -> Result<()> {
    let state = &mut ctx.accounts.program_state;
    let deployer = &ctx.accounts.deployer;
    if state.initialize {
        return Err(AlreadyInitialized.into());
    }
    state.campaign_count = 0;
    state.platform_fee = 5;
    state.platform_address = deployer.key();
    state.initialize = true;
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeCtx<'info> {
    #[account(
        init,
        payer= deployer,
        space = ANCHOR_DISCRIMINATOR_SIZE + ProgramState::INIT_SPACE,
        seeds = [b"program_state"],
        bump
    )]
    pub program_state: Account<'info, ProgramState>,

    #[account(mut)] // this allow us to change the balance of deployer
    pub deployer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
