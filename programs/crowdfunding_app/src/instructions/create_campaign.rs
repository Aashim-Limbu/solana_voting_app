use anchor_lang::prelude::*;

use crate::constants::ANCHOR_DISCRIMINATOR_SIZE;
use crate::errors::ErrorCode::*;
use crate::states::{Campaign, ProgramState};

pub fn create_campaign(
    ctx: Context<CreateCampaignCtx>,
    title: String,
    description: String,
    image_url: String,
    goal: u64,
) -> Result<()> {
    let campaign = &mut ctx.accounts.campaign;
    let state = &mut ctx.accounts.program_state;
    if title.len() > 64 {
        return Err(TitleTooLong.into());
    }
    if description.len() > 512 {
        return Err(DescriptionTooLong.into());
    }
    if image_url.len() > 64 {
        return Err(ImageURLTooLong.into());
    }
    if goal < 1_000_000_000 {
        return Err(InvalidGoalAmount.into());
    }
    campaign.cid = state.campaign_count;
    campaign.title = title;
    campaign.amount_raised = 0;
    campaign.donors = 0;
    campaign.timestamp = Clock::get()?.unix_timestamp as u64;
    campaign.active = true;
    campaign.description = description;
    campaign.creator = ctx.accounts.creator.key();
    campaign.image_url = image_url;
    campaign.goal = goal;
    Ok(())
}

#[derive(Accounts)]
pub struct CreateCampaignCtx<'info> {
    pub program_state: Account<'info, ProgramState>,
    #[account(
        init,
        payer=creator,
        space= ANCHOR_DISCRIMINATOR_SIZE + Campaign::INIT_SPACE,
        // each PDA must be unique .To avoid collision
        seeds=[
            b"campaign",
            (program_state.campaign_count + 1).to_le_bytes().as_ref(),
            ],
        bump
    )]
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
