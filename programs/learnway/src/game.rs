use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, TokenAccount, Transfer};
use anchor_lang::solana_program;
use solana_program::msg;
use std::time::{Instant, Duration};

declare_id!("4PPXwojpTCyAvCf2A9WerJr51H6bF9dT7qdPQeLwxukx");

const QUESTION_DURATION_SECS: u64 = 30;
const BONUS_THRESHOLD_1_SECS: u64 = 3;
const BONUS_THRESHOLD_2_SECS: u64 = 5;
const CORRECT_ANSWER_POINTS: i32 = 5;
const INCORRECT_ANSWER_POINTS: i32 = -5;
const BONUS_THRESHOLD_1_POINTS: i32 = 3;
const BONUS_THRESHOLD_2_POINTS: i32 = 1;

#[program]
pub mod quiz_game {
    use super::*;

    #[derive(Accounts)]
    pub struct MatchPlayers<'info> {
        #[account(signer)]
        player_a: AccountInfo<'info>,
        #[account(signer)]
        player_b: AccountInfo<'info>,
        #[account(mut)]
        admin_token_account: Account<'info, TokenAccount>,
        #[account(mut)]
        token_program: AccountInfo<'info>,
    }

    pub fn match_players(ctx: Context<MatchPlayers>) -> ProgramResult {
        // Fetch staked tokens from both players
        let amount_a = ctx.accounts.player_a.lamports();
        let amount_b = ctx.accounts.player_b.lamports();

        // Ensure both players have staked the same amount
        if amount_a != amount_b {
            return Err(ErrorCode::UnequalStakes.into());
        }

        // Perform other operations to match players and generate questions
        // ...

        Ok(())
    }

    #[derive(Accounts)]
    pub struct AnswerQuestion<'info> {
        #[account(signer)]
        user: AccountInfo<'info>,
        #[account(mut)]
        user_token_account: Account<'info, TokenAccount>,
        #[account(mut)]
        admin_token_account: Account<'info, TokenAccount>,
        #[account(mut)]
        token_program: AccountInfo<'info>,
    }

    pub fn answer_question(ctx: Context<AnswerQuestion>, correct: bool, time_taken_secs: u64) -> ProgramResult {
        // Calculate points earned based on correctness and time taken
        let mut points = if correct { CORRECT_ANSWER_POINTS } else { INCORRECT_ANSWER_POINTS };
        if time_taken_secs <= BONUS_THRESHOLD_1_SECS {
            points += BONUS_THRESHOLD_1_POINTS;
        } else if time_taken_secs <= BONUS_THRESHOLD_2_SECS {
            points += BONUS_THRESHOLD_2_POINTS;
        }

        // Add points to user's total score
        // ...

        // Perform other operations to handle the answer
        // ...

        Ok(())
    }

    // Other game-related functions...
}

#[error]
pub enum ErrorCode {
    #[msg("Unequal stakes")]
    UnequalStakes,
}