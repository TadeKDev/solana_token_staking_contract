use anchor_lang::prelude::*;
use anchor_spl::token::{
    self, Mint, Token,
    TokenAccount, Transfer,
};

declare_id!("3GtHR9kYEejJP9X6zpSiGtSLEWY8ZJdawsEWAJ55h4sB");

#[program]
pub mod anchor_escrow {
    use super::*;

    // need to call this function after deploying smart contract to set admin wallet
    pub fn init_admin(ctx: Context<InitAdmin>) -> Result<()> {
        ctx.accounts.admin_state.admin = *ctx.accounts.admin.key;
        ctx.accounts.admin_state.stake_token = *ctx.accounts.stake_token.to_account_info().key;
        ctx.accounts.admin_state.bump = *ctx.bumps.get("admin_state").unwrap();
        ctx.accounts.admin_state.admin_fee = 0;
        ctx.accounts.admin_state.reward_rate = 0;
        ctx.accounts.admin_state.lock_period = 0;
        ctx.accounts.admin_state.total_amount = 0;
        ctx.accounts.admin_state.locked_amount = 0;
        ctx.accounts.admin_state.staked_user_amount = 0;
        Ok(())
    }

    // change admin wallet using this function
    pub fn change_admin(ctx: Context<ChangeAdmin>) -> Result<()> {
        ctx.accounts.admin_state.admin = *ctx.accounts.new_admin.key;

        Ok(())
    }

    //  update necessary parameters like fee/stake reward/lock period
    pub fn update_admin_info(
        ctx: Context<UpdateAdminInfo>,
        admin_fee: u64,
        reward_rate: u64,
        lock_period: u64,
    ) -> Result<()> {
        ctx.accounts.admin_state.admin_fee = admin_fee;
        ctx.accounts.admin_state.reward_rate = reward_rate;
        ctx.accounts.admin_state.lock_period = lock_period;

        Ok(())
    }

    pub fn stake(ctx: Context<Stake>, stake_amount: u64) -> Result<()> {
        let clock = Clock::get()?;
        let diff_time = clock.unix_timestamp - ctx.accounts.user_state.stake_date;
        ctx.accounts.user_state.previous_stake_reward = ctx.accounts.user_state.stake_amount
            * ctx.accounts.admin_state.reward_rate 
            * (diff_time as u64) // locking time
            / (86400 * 180); // 180 lock days

        ctx.accounts.user_state.stake_amount = ctx.accounts.user_state.stake_amount + stake_amount;
        ctx.accounts.user_state.stake_date = clock.unix_timestamp;

        // transfer tokens to vault account
        token::transfer(
            ctx.accounts.into_transfer_to_pda_context(),
            stake_amount
        )?;

        Ok(())
    }

    // withdraw possible staking reward
    pub fn get_reward(ctx: Context<GetReward>) -> Result<()> {
        let clock = Clock::get()?;
        let diff_time = clock.unix_timestamp - ctx.accounts.user_state.stake_date;
        let stake_reward = ctx.accounts.user_state.stake_amount
            * ctx.accounts.admin_state.reward_rate 
            * (diff_time as u64) // locking time
            / (86400 * 180); // 180 lock days

        ctx.accounts.user_state.previous_stake_reward = 0;

        ctx.accounts.user_state.stake_date = clock.unix_timestamp;

        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(), 
            Transfer {
                from: ctx.accounts.vault.to_account_info(), 
                to: ctx.accounts.staker_deposit_token_account.to_account_info(),
                authority: ctx.accounts.vault.to_account_info(), 
            }
        );
        let bump = *ctx.bumps.get("vault").unwrap();
        let stake_token_key = ctx.accounts.stake_token.key();
        let pda_sign = &[
            b"vault",
            stake_token_key.as_ref(),
            &[bump],
        ];

        token::transfer(
            transfer_ctx.with_signer(&[pda_sign]), 
            stake_reward
        )?;

        Ok(())
    }

    // withdraw possible staking reward
    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        let clock = Clock::get()?;
        let diff_time = clock.unix_timestamp - ctx.accounts.user_state.stake_date;
        let stake_reward = ctx.accounts.user_state.stake_amount
            * ctx.accounts.admin_state.reward_rate 
            * (diff_time as u64) // locking time
            / (86400 * 180); // 180 lock days

        ctx.accounts.user_state.previous_stake_reward = 0;

        ctx.accounts.user_state.stake_date = clock.unix_timestamp;

        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(), 
            Transfer {
                from: ctx.accounts.vault.to_account_info(), 
                to: ctx.accounts.staker_deposit_token_account.to_account_info(),
                authority: ctx.accounts.vault.to_account_info(), 
            }
        );
        let bump = *ctx.bumps.get("vault").unwrap();
        let stake_token_key = ctx.accounts.stake_token.key();
        let pda_sign = &[
            b"vault",
            stake_token_key.as_ref(),
            &[bump],
        ];

        token::transfer(
            transfer_ctx.with_signer(&[pda_sign]), 
            stake_reward + ctx.accounts.user_state.stake_amount,
        )?;

        ctx.accounts.user_state.stake_amount = 0;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitAdmin<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
         init,
         seeds = [b"state".as_ref(), b"admin".as_ref()],
         bump,
         payer = admin,
         space = AdminState::space()
     )]
    pub admin_state: Box<Account<'info, AdminState>>,
    #[account(mut)]
    pub stake_token: Account<'info, Mint>,
    // create vault to lock tokens
    #[account(
        init,
        seeds = [b"vault".as_ref(), stake_token.key().as_ref()],
        bump,
        payer = admin,
        token::mint = stake_token,
        token::authority = vault,
    )]
    pub vault: Account<'info, TokenAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ChangeAdmin<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub new_admin: AccountInfo<'info>,
    #[account(
        mut,
        constraint = admin_state.admin == *admin.key,
        seeds = [b"state".as_ref(), b"admin".as_ref()],
        bump = admin_state.bump
    )]
    pub admin_state: Box<Account<'info, AdminState>>,
}

#[derive(Accounts)]
#[instruction(admin_fee: u64, reward_rate: u64, lock_period: u64)]
pub struct UpdateAdminInfo<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(
        mut,
        constraint = admin_state.admin == *admin.key,
        seeds = [b"state".as_ref(), b"admin".as_ref()],
        bump = admin_state.bump
    )]
    pub admin_state: Box<Account<'info, AdminState>>,
}

#[derive(Accounts)]
#[instruction(stake_amount: u64)]
pub struct Stake<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub staker: Signer<'info>,
    #[account(
        mut,
        seeds = [b"state".as_ref(), b"admin".as_ref()],
        bump = admin_state.bump
    )]
    pub admin_state: Box<Account<'info, AdminState>>,
    #[account(mut)]
    pub stake_token: Account<'info, Mint>,
    #[account(
        seeds = [b"vault".as_ref(), stake_token.key().as_ref()],
        bump,
    )]
    pub vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = stake_token,
        token::authority = staker,
        constraint = staker_deposit_token_account.amount >=stake_amount
    )]
    pub staker_deposit_token_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        seeds = [b"state".as_ref(), b"user".as_ref(), staker.key().as_ref()],
        bump,
        payer = staker,
        space = UserState::space()
    )]
    pub user_state: Box<Account<'info, UserState>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: Program<'info, Token>,
}


#[derive(Accounts)]
pub struct GetReward<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub staker: Signer<'info>,
    #[account(
        mut,
        seeds = [b"state".as_ref(), b"admin".as_ref()],
        bump = admin_state.bump
    )]
    pub admin_state: Box<Account<'info, AdminState>>,
    #[account(mut)]
    pub stake_token: Account<'info, Mint>,
    #[account(
        seeds = [b"vault".as_ref(), stake_token.key().as_ref()],
        bump,
    )]
    pub vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = stake_token,
        token::authority = staker,
    )]
    pub staker_deposit_token_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        seeds = [b"state".as_ref(), b"user".as_ref(), staker.key().as_ref()],
        bump,
        payer = staker,
        space = UserState::space()
    )]
    pub user_state: Box<Account<'info, UserState>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub staker: Signer<'info>,
    #[account(
        mut,
        seeds = [b"state".as_ref(), b"admin".as_ref()],
        bump = admin_state.bump
    )]
    pub admin_state: Box<Account<'info, AdminState>>,
    #[account(mut)]
    pub stake_token: Account<'info, Mint>,
    #[account(
        seeds = [b"vault".as_ref(), stake_token.key().as_ref()],
        bump,
    )]
    pub vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = stake_token,
        token::authority = staker,
    )]
    pub staker_deposit_token_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        seeds = [b"state".as_ref(), b"user".as_ref(), staker.key().as_ref()],
        bump,
        payer = staker,
        space = UserState::space()
    )]
    pub user_state: Box<Account<'info, UserState>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct AdminState {
    pub bump: u8,
    pub admin: Pubkey,
    pub admin_fee: u64,     // admin will get fee when users withdraw stake reward
    pub reward_rate: u64,   // reward rate
    pub lock_period: u64,   // lock period for staking
    pub total_amount: u64,  // total staked amount all time
    pub locked_amount: u64, // current locked amount
    pub staked_user_amount: u64,
    pub stake_token: Pubkey,
}

impl AdminState {
    pub fn space() -> usize {
        8 + 1 + 32 + 8 + 8 + 8 + 8 + 8 + 8 + 32
    }
}

#[account]
pub struct UserState {
    pub bump: u8,
    pub stake_date: i64,
    pub stake_amount: u64,
    // will be added when user stake more. previous staked amount was 100,
    // and user stakes 200 tokens again, then we need to calculate current reward and need to add to previous_stake_reward
    pub previous_stake_reward: u64,
}

impl UserState {
    pub fn space() -> usize {
        8 + 1 + 8 + 8 + 8
    }
}


impl<'info> Stake<'info> {
    fn into_transfer_to_pda_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.staker_deposit_token_account.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.staker.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}