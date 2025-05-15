use anchor_lang::prelude::*;
pub mod constants;
pub mod errors;
pub mod instructions;
pub mod states;

use instructions::*;
use states::*;

declare_id!("7H5eMwxLuMCz1tcSFHDb9j7FnHMtKhyW8mxw8CD8c9Gi");

#[program]
pub mod crowdfunding_app {
    use super::*;

    pub fn initialize(ctx: Context<InitializeCtx>) -> Result<()> {
        instructions::initialize(ctx)
    }
    pub fn create_campaign(
        ctx: Context<CreateCampaignCtx>,
        title: String,
        description: String,
        image_url: String,
        goal: u64,
    ) -> Result<()> {
        instructions::create_campaign(ctx, title, description, image_url, goal)
    }

    pub fn update_campaign(
        ctx: Context<UpdateCampaignCtx>,
        cid: u64,
        title: String,
        image_url: String,
        goal: u64,
        description: String,
    ) -> Result<()> {
        instructions::update_campaign(ctx, cid, title, image_url, goal, description)
    }

    pub fn delete_campaign(ctx: Context<DeleteCampaignCtx>, cid: u64) -> Result<()> {
        instructions::delete_campaign(ctx, cid)
    }
}
