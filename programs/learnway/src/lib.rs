use anchor_lang::prelude::*;

declare_id!("4PPXwojpTCyAvCf2A9WerJr51H6bF9dT7qdPQeLwxukx");

#[program]
pub mod quiz_game {
    use super::*;

    #[derive(Accounts)]
    pub struct CreateRoom<'info> {
        #[account(signer)]
        user: AccountInfo<'info>,
        #[account(mut)]
        user_token_account: Account<'info, TokenAccount>,
        #[account(mut)]
        admin_token_account: Account<'info, TokenAccount>,
        #[account(mut)]
        token_program: AccountInfo<'info>,
        #[account("mint")]
        token_mint: Account<'info, Mint>,
    }

    pub fn create_room(ctx: Context<CreateRoom>, amount: u64) -> ProgramResult {
        // Transfer tokens from the user to the admin account
        token::transfer(
            ctx.accounts.token_program.clone(),
            ctx.accounts.user_token_account.clone(),
            ctx.accounts.admin_token_account.clone(),
            amount,
        )?;

        // Perform other operations to create the room
        // ...

        Ok(())
    }

    #[derive(Accounts)]
    pub struct JoinRoom<'info> {
        #[account(signer)]
        user: AccountInfo<'info>,
        #[account(mut)]
        user_token_account: Account<'info, TokenAccount>,
        #[account(mut)]
        admin_token_account: Account<'info, TokenAccount>,
        #[account(mut)]
        token_program: AccountInfo<'info>,
    }

    pub fn join_room(ctx: Context<JoinRoom>, amount: u64) -> ProgramResult {
        // Transfer tokens from the user to the admin account
        token::transfer(
            ctx.accounts.token_program.clone(),
            ctx.accounts.user_token_account.clone(),
            ctx.accounts.admin_token_account.clone(),
            amount,
        )?;

        // Perform other operations to join the room
        // ...

        Ok(())
    }

    #[derive(Accounts)]
    pub struct StartGame<'info> {
        #[account(signer)]
        user: AccountInfo<'info>,
        #[account(mut)]
        user_token_account: Account<'info, TokenAccount>,
        #[account(mut)]
        admin_token_account: Account<'info, TokenAccount>,
        #[account(mut)]
        token_program: AccountInfo<'info>,
    }

    pub fn start_game(ctx: Context<StartGame>) -> ProgramResult {
        // Perform operations to start the game
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

    pub fn answer_question(ctx: Context<AnswerQuestion>) -> ProgramResult {
        // Perform operations to answer a question
        // ...

        Ok(())
    }

    // Other game-related functions...
}