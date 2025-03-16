use anchor_lang::prelude::*;
use crate::states::config::ConfigUpdated;
use crate::error::SolLearningError;
use crate::instructions::structs::update_program_config_struct::UpdateProgramConfig;
use crate::utils::pause::check_program_running;
use crate::utils::config::check_authority;

pub fn update_program_config_handler(
    ctx: Context<UpdateProgramConfig>,
    max_educators: Option<u16>,
    max_courses_per_educator: Option<u16>,
    max_mint_amount: Option<u64>,
    mint_cooldown_period: Option<i64>,
) -> Result<()> {
    check_program_running(&ctx.accounts.program_state)?;
    check_authority(&ctx.accounts.program_config, &ctx.accounts.authority)?;

    let current_time = Clock::get()?.unix_timestamp;
    let mut updated = false;

    {
        let config = &mut ctx.accounts.program_config;

        if let Some(value) = max_educators {
            config.max_educators = value;
            updated = true;
        }

        if let Some(value) = max_courses_per_educator {
            config.max_courses_per_educator = value;
            updated = true;
        }

        if let Some(value) = max_mint_amount {
            require!(value > 0, SolLearningError::InvalidAmount);
            config.max_mint_amount = value;
            updated = true;
        }

        if let Some(value) = mint_cooldown_period {
            require!(value >= 0, SolLearningError::InvalidAmount);
            config.mint_cooldown_period = value;
            updated = true;
        }

        if updated {
            config.last_updated_at = current_time;
        }
    }

    if updated {
        emit!(ConfigUpdated {
            authority: ctx.accounts.authority.key(),
            timestamp: current_time,
        });

        msg!("Program config updated by {}", ctx.accounts.authority.key());
    } else {
        msg!("No updates provided to program config");
    }

    Ok(())
}