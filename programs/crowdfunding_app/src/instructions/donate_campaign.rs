use crate::errors::ErrorCode::*;
use anchor_lang::{prelude::*, solana_program::program};

use crate::{
    constants::ANCHOR_DISCRIMINATOR_SIZE,
    states::{Campaign, Transaction},
};

pub fn donate_campaign(ctx: Context<DonateCampaignCtx>, cid: u64, amount: u64) -> Result<()> {
    let campaign = &mut ctx.accounts.campaign;
    let donor = &mut ctx.accounts.donor;
    let transaction = &mut ctx.accounts.transaction;

    if !campaign.active {
        return Err(CampaignInactive.into());
    }
    if campaign.cid != cid {
        return Err(CampaignNotFound.into());
    }
    if amount < 1_000_000_000 {
        return Err(InvalidDonationAmount.into());
    }
    if campaign.amount_raised > campaign.goal {
        return Err(CampaignGoalAchieved.into());
    }
    let tx_instruction = anchor_lang::solana_program::system_instruction::transfer(
        &donor.key(),
        &campaign.key(),
        amount,
    );
    let result = anchor_lang::solana_program::program::invoke(
        &tx_instruction,
        &[donor.to_account_info(), campaign.to_account_info()],
    );
    Ok(())
}

#[derive(Accounts)]
#[instruction(cid:u64)]
pub struct DonateCampaignCtx<'info> {
    pub campaign: Account<'info, Campaign>,
    #[account(
        init,
        payer = donor,
        space = ANCHOR_DISCRIMINATOR_SIZE + Transaction::INIT_SPACE,
        seeds=[
            b"donor",
            donor.key().as_ref(),
            cid.to_le_bytes().as_ref(),
            (campaign.donors +1).to_le_bytes().as_ref(),
        ],
        bump,
    )]
    pub transaction: Account<'info, Transaction>,
    #[account(mut)]
    donor: Signer<'info>, // technically we're changing the balance of the donor.Thus we want this signer to be mutable.
    system_program: Program<'info, System>,
}
