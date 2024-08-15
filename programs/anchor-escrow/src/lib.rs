use anchor_lang::prelude::*;
use anchor_spl::token::{
    self, spl_token::instruction::AuthorityType, CloseAccount, Mint, SetAuthority, Token,
    TokenAccount, Transfer,
};

declare_id!("3GtHR9kYEejJP9X6zpSiGtSLEWY8ZJdawsEWAJ55h4sB");

#[program]
pub mod anchor_escrow {
    use super::*;

    // need to call this function after deploying smart contract to set admin wallet
    pub fn init_admin(ctx: Context<InitAdmin>) -> Result<()> {
        ctx.accounts.admin_state.admin = *ctx.accounts.admin.key;
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
}

impl AdminState {
    pub fn space() -> usize {
        8 + 1 + 32 + 8 + 8 + 8 + 8 + 8 + 8
    }
}

#[account]
pub struct EscrowState {
    pub random_seed: u64,
    pub initializer_key: Pubkey,
    pub taker: Pubkey,
    pub initializer_amount: [u64; 5],
    pub dispute_status: bool,
    pub refund_status: bool,
    pub mint: Pubkey,
    pub bump: u8,
    pub vault_bump: u8,
}

impl EscrowState {
    pub fn space() -> usize {
        8 + 148
    }
}
