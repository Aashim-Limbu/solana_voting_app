use anchor_lang::prelude::*;
pub mod errors;
pub mod instructions;
pub mod states;
pub mod constants;

use error::*;
use instructions::*;
use states::*;
use constants::*;

declare_id!("7H5eMwxLuMCz1tcSFHDb9j7FnHMtKhyW8mxw8CD8c9Gi");

#[program]
pub mod crowdfunding_app {
    use super::*;

    pub fn initialize(ctx:Context<InitializeCtx>) -> Result<()>{
        instructions::initialize(ctx)
    }

}
