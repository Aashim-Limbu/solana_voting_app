use anchor_lang::prelude::*;

use crate::errors::ErrorCode::*;
use crate::states::{campaign, Campaign};

pub fn delete_campaign(ctx: Context<DeleteCampaignCtx>, cid: u64) -> Result<()> {
    let campaign = &mut ctx.accounts.campaign;
    let creator = &mut ctx.accounts.creator;
    if campaign.cid != cid {
        return Err(CampaignNotFound.into());
    }
    if creator.key() != campaign.creator {
        return Err(UnauthorizedAccess.into());
    }
    if !campaign.active {
        return Err(CampaignInactive.into());
    }
    campaign.active = false;
    Ok(())
}



#[derive(Accounts)]
#[instruction(cid:u64)]
pub struct DeleteCampaignCtx<'info> {
    #[account(
        mut,
        seeds=[
            b"campaign",
            cid.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub campaign: Account<'info, Campaign>,

    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
