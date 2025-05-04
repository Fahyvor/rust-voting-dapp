use anchor_lang::prelude::*;

declare_id!("9sgs8UoQsZwYCDJaQJUPTejLTp6bWiW98hc1Zfc4S5uW");

#[program]
pub mod sol_voting_dapp {
    use super::*;

    pub fn create_poll(ctx: Context<CreatePoll>, question: String, _options: Vec<String>) -> Result<()> {
        msg!("Creating poll with question: {}", question);
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, poll_id: Pubkey, option_index: u64) -> Result<()> {
        let poll = &mut ctx.accounts.poll;

        if !poll.initialized {
            return Err(ErrorCode::PollNotInitialized.into());
        }


        if option_index >= poll.options.len() as u64 {
            return Err(ErrorCode::InvalidOptionIndex.into());
        }

        if poll.votes.len() <= option_index as usize {
            return Err(ErrorCode::InvalidOptionIndex.into())
        }

        //Check if the user has already voted
        if poll.voted_users.contains(&ctx.accounts.user.key()) {
            return Err(ErrorCode::VoteAlreadyCasted.into());
        }

        // Add the user to the list of voted users
        poll.voted_users.push(ctx.accounts.user.key());

        // Increment the vote count for the selected option
        poll.votes[option_index as usize] += 1;

        // Store the vote in the user's account (if needed)

        msg!("Voting on poll: {} with option index: {}", poll_id, option_index);
        Ok(())
    }

    pub fn close_poll(ctx: Context<ClosePoll>, poll_id: Pubkey) -> Result<()> {
        msg!("Closing poll: {}", poll_id);
        Ok(())
    }

    pub fn create_candidate(ctx: Context<CreateCandidate>, poll_id: Pubkey, candidate_name: String) -> Result<()> {
        msg!("Creating candidate for poll: {} with name: {}", poll_id, candidate_name);
        Ok(())
    }

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct CreatePoll<'info> {
    #[account(
        init,
        payer = user,
        space = Poll::LEN,
        seeds = [b"poll", poll_id.key().as_ref()],
        bump,
    )]
    pub poll: Account<'info, Poll>,
    pub poll_id: AccountInfo<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Poll {
    pub const LEN: usize = 8 + 4 + 256 + 4 + (32 * 10) + 4 + (8 * 10) + 4 + (32 * 100);
    pub question: String,
    pub options: Vec<String>,
    pub votes: Vec<u64>,
    pub initialized: bool,
    pub voted_users: Vec<Pubkey>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub poll: Account<'info, Poll>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClosePoll<'info> {
    #[account(
        init,
        payer = user,
        space = Poll::LEN,
        seeds = [b"poll", poll_id.key().as_ref()
],
        bump,
    )]
    pub poll: Account<'info, Poll>,
    pub poll_id: AccountInfo<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateCandidate<'info> {
    #[account(mut)]
    pub poll: Account<'info, Poll>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Poll not found")]
    PollNotFound,
    #[msg("Invalid option index")]
    InvalidOptionIndex,
    #[msg("Poll already closed")]
    PollAlreadyClosed,
    #[msg("Unauthorized action")]
    UnauthorizedAction,
    #[msg("Invalid candidate name")]
    InvalidCandidateName,
    #[msg("Candidate already exists")]
    CandidateAlreadyExists,
    #[msg("Poll not initialized")]
    PollNotInitialized,
    #[msg("Vote already casted")]
    VoteAlreadyCasted,
    #[msg("Poll not active")]
    PollNotActive,
    #[msg("Poll already exists")]
    PollAlreadyExists,
    #[msg("Invalid poll ID")]
    InvalidPollId,
    #[msg("Invalid user")]
    InvalidUser,
    #[msg("Invalid program ID")]
    InvalidProgramId,
}
