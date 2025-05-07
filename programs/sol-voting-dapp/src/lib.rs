use anchor_lang::prelude::*;

declare_id!("9sgs8UoQsZwYCDJaQJUPTejLTp6bWiW98hc1Zfc4S5uW");

#[program]
pub mod sol_voting_dapp {
    use super::*;

    // Create a new poll
    pub fn create_poll(
        ctx: Context<CreatePoll>,
        poll_id: Pubkey,
        question: String,
        candidates: Vec<String>,
    ) -> Result<()> {
        msg!("Creating poll with ID: {}", poll_id);
        let poll = &mut ctx.accounts.poll;
        
        // Set poll data
        poll.question = question.clone();
        poll.candidates = candidates.clone();
        poll.votes = vec![0; candidates.len()];
        poll.creator = ctx.accounts.user.key();
        poll.voted_users = Vec::new();
        
        // Mark as initialized - do this last to ensure all data is set
        poll.initialized = true;
        
        msg!("Poll created successfully with question: {}", question);
        msg!("Initial candidates: {:?}", candidates);
        Ok(())
    }

    // Create a new candidate for a poll
    pub fn create_candidate(
        ctx: Context<CreateCandidate>,
        poll_id: Pubkey,
        candidate_name: String,
    ) -> Result<()> {
        msg!("Adding candidate {} to poll: {}", candidate_name, poll_id);
        let poll = &mut ctx.accounts.poll;
    
        // Check if poll is initialized
        if !poll.initialized {
            msg!("Error: Poll not initialized");
            return Err(ErrorCode::PollNotInitialized.into());
        }
        
        msg!("Poll is initialized with {} candidates", poll.candidates.len());
    
        // Check if candidate already exists
        if poll.candidates.contains(&candidate_name) {
            msg!("Error: Candidate already exists");
            return Err(ErrorCode::CandidateAlreadyExists.into());
        }
    
        // Add candidate and initialize vote count
        poll.candidates.push(candidate_name.clone());
        poll.votes.push(0);
    
        msg!("Candidate {} added successfully to poll", candidate_name);
        msg!("Poll now has {} candidates", poll.candidates.len());
        Ok(())
    }

    // Vote on a poll
    pub fn vote(ctx: Context<Vote>, poll_id: Pubkey, option_index: u64) -> Result<()> {
        msg!("Voting on poll: {} with option index: {}", poll_id, option_index);
        let poll = &mut ctx.accounts.poll;

        // Check if poll is initialized
        if !poll.initialized {
            msg!("Error: Poll not initialized");
            return Err(ErrorCode::PollNotInitialized.into());
        }

        // Check if option index is valid
        if option_index >= poll.candidates.len() as u64 {
            msg!("Error: Invalid option index {}, max is {}", option_index, poll.candidates.len() - 1);
            return Err(ErrorCode::InvalidOptionIndex.into());
        }

        // Check if user has already voted
        if poll.voted_users.contains(&ctx.accounts.user.key()) {
            msg!("Error: User has already voted");
            return Err(ErrorCode::VoteAlreadyCasted.into());
        }

        // Record vote
        poll.voted_users.push(ctx.accounts.user.key());
        poll.votes[option_index as usize] += 1;

        msg!("Vote recorded successfully for option {}", option_index);
        Ok(())
    }

    // Close a poll
    pub fn close_poll(ctx: Context<ClosePoll>, poll_id: Pubkey) -> Result<()> {
        msg!("Closing poll: {}", poll_id);
        let poll = &mut ctx.accounts.poll;

        // Check if poll is initialized
        if !poll.initialized {
            msg!("Error: Poll not initialized");
            return Err(ErrorCode::PollNotInitialized.into());
        }

        // Verify that the user closing is the creator
        if poll.creator != ctx.accounts.user.key() {
            msg!("Error: Only creator can close the poll");
            return Err(ErrorCode::UnauthorizedAction.into());
        }

        // Mark poll as closed
        poll.initialized = false;
        msg!("Poll {} closed successfully", poll_id);
        Ok(())
    }

    // Initialize the program
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
#[instruction(poll_id: Pubkey, question: String, candidates: Vec<String>)]
pub struct CreatePoll<'info> {
    #[account(
        init,
        seeds = [b"poll", poll_id.as_ref()],
        bump,
        payer = user,
        space = 8  // discriminator
            + 32   // creator pubkey
            + 4 + 200 // question string (4 bytes for length + max 200 chars)
            + 4 + (4 + 50) * 10  // candidates vec: 4 bytes for vec len + up to 10 candidates of 50 chars each
            + 4 + 4 * 10  // votes vec: 4 bytes for vec len + up to 10 u32 votes
            + 1    // initialized bool
            + 4 + 32 * 100  // voted_users vec: 4 bytes for vec len + up to 100 pubkeys
    )]
    pub poll: Account<'info, Poll>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Poll {
    pub creator: Pubkey,
    pub question: String,
    pub candidates: Vec<String>,
    pub votes: Vec<u32>,
    pub initialized: bool,
    pub voted_users: Vec<Pubkey>,
}

#[derive(Accounts)]
#[instruction(poll_id: Pubkey, option_index: u64)]
pub struct Vote<'info> {
    #[account(mut, seeds = [b"poll", poll_id.as_ref()], bump)]
    pub poll: Account<'info, Poll>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(poll_id: Pubkey)]
pub struct ClosePoll<'info> {
    #[account(
        mut, 
        seeds = [b"poll", poll_id.as_ref()], 
        bump,
        close = user
    )]
    pub poll: Account<'info, Poll>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(poll_id: Pubkey, candidate_name: String)]
pub struct CreateCandidate<'info> {
    #[account(mut, seeds = [b"poll", poll_id.as_ref()], bump)]
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
}