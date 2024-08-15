use anchor_lang::prelude::*;
use anchor_spl::token::{
    self, spl_token::instruction::AuthorityType, CloseAccount, Mint, SetAuthority, Token,
    TokenAccount, Transfer,
};

declare_id!("3GtHR9kYEejJP9X6zpSiGtSLEWY8ZJdawsEWAJ55h4sB");

#[program]
pub mod anchor_escrow {
    use super::*;
}

#[derive(Accounts)]
#[instruction(escrow_seed: u64, initializer_amount: [u64;5])]
pub struct Initialize<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub initializer: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub taker: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [b"state".as_ref(), b"admin".as_ref()],
        bump = admin_state.bump
    )]
    pub admin_state: Box<Account<'info, AdminState>>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        seeds = [b"vault".as_ref(), &escrow_seed.to_le_bytes()],
        bump,
        payer = initializer,
        token::mint = mint,
        token::authority = initializer,
    )]
    pub vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = mint,
        token::authority = initializer,
        constraint = initializer_deposit_token_account.amount >=(initializer_amount[0]+initializer_amount[1]+initializer_amount[2]+initializer_amount[3]+initializer_amount[4])
    )]
    pub initializer_deposit_token_account: Account<'info, TokenAccount>,
    #[account(
        init,
        seeds = [b"state".as_ref(), &escrow_seed.to_le_bytes()],
        bump,
        payer = initializer,
        space = EscrowState::space()
    )]
    pub escrow_state: Box<Account<'info, EscrowState>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct AdminState {
    pub bump: u8,
    pub admin_fee: u64,
    pub resolver_fee: u64,
    pub admin1: Pubkey,
    pub admin2: Pubkey,
    pub resolver: Pubkey,
    pub total_amount: u64,
    pub locked_amount: u64,
    pub active_escrow: u64,
    pub completed_escrow: u64,
    pub disputed_escrow: u64,
    pub refunded_escrow: u64,
}

impl AdminState {
    pub fn space() -> usize {
        8 + 161
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

impl<'info> Initialize<'info> {
    fn into_transfer_to_pda_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.initializer_deposit_token_account.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.initializer.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

    fn into_set_authority_context(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            account_or_mint: self.vault.to_account_info(),
            current_authority: self.initializer.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}
