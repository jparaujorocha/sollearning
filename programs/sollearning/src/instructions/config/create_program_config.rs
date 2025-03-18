use anchor_lang::prelude::*;
use crate::states::config::ConfigUpdated;
use crate::instructions::config::create_program_config_struct::CreateProgramConfig;

pub fn create_program_config_handler(
    ctx: Context<CreateProgramConfig>,
    max_educators: u16,
    max_courses_per_educator: u16,
    max_mint_amount: u64,
    mint_cooldown_period: i64,
) -> Result<()> {
    let config_bump = ctx.bumps.program_config;
    let current_time = Clock::get()?.unix_timestamp;

    {
        let config = &mut ctx.accounts.program_config;
        config.max_educators = max_educators;
        config.max_courses_per_educator = max_courses_per_educator;
        config.max_mint_amount = max_mint_amount;
        config.mint_cooldown_period = mint_cooldown_period;
        config.authority = ctx.accounts.authority.key();
        config.last_updated_at = current_time;
        config.bump = config_bump;
    }

    emit!(ConfigUpdated {
        authority: ctx.accounts.authority.key(),
        timestamp: current_time,
    });

    msg!(
        "Program config created with max educators: {}, max courses per educator: {}, max mint amount: {}, mint cooldown period: {}",
        max_educators,
        max_courses_per_educator,
        max_mint_amount,
        mint_cooldown_period
    );

    Ok(())
}