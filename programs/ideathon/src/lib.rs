use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};

declare_id!("EFr4g6v9FbL1MHexWPAjmoEtaoDsrYKuztF47EhpZSbu");

#[program]
pub mod ideathon {
    use super::*;

    pub fn initialize_event(ctx: Context<InitializeEvent>, event_id: u64, price: u64, reward: u64) -> Result<()> {
        let event = &mut ctx.accounts.event_account;
        event.organizer = ctx.accounts.authority.key();
        event.price = price;
        event.reward_amount = reward;
        event.is_active = true;
        event.event_id = event_id;
        event.total_checked_in = 0;
        event.winner = None;
        Ok(())
    }

    pub fn buy_ticket(ctx: Context<BuyTicket>, event_id: u64) -> Result<()> {
        let event = &ctx.accounts.event_account;
        let user_stats = &mut ctx.accounts.user_stats;

        require!(event.is_active, HypeError::EventNotActive);

        let mut final_price = event.price;
        if user_stats.attendance_count >= 3 {
            final_price = (event.price * 90) / 100;
        }

        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.buyer.to_account_info(),
                to: ctx.accounts.organizer.to_account_info(),
            },
        );
        transfer(cpi_ctx, final_price)?;

        let ticket = &mut ctx.accounts.ticket_account;
        ticket.owner = ctx.accounts.buyer.key();
        ticket.event_id = event_id;
        ticket.is_used = false;

        Ok(())
    }

    pub fn check_in(ctx: Context<CheckIn>, _event_id: u64) -> Result<()> {
        let ticket = &mut ctx.accounts.ticket_account;
        let event = &mut ctx.accounts.event_account;
        let user_stats = &mut ctx.accounts.user_stats;

        require!(!ticket.is_used, HypeError::TicketAlreadyUsed);
        require!(
            ticket.owner == ctx.accounts.ticket_owner.key(),
            HypeError::NotTicketOwner
        );

        ticket.is_used = true;
        event.total_checked_in += 1;
        user_stats.attendance_count += 1;

        msg!("Check-in realizado! Experiência confirmada.");
        Ok(())
    }

    pub fn declare_winner(ctx: Context<DeclareWinner>, _event_id: u64) -> Result<()> {
        let event = &mut ctx.accounts.event_account;

        require!(event.is_active, HypeError::EventNotActive);
        require!(event.winner.is_none(), HypeError::WinnerAlreadyDeclared);

        event.winner = Some(ctx.accounts.winner.key());
        event.is_active = false;

        msg!("Vencedor declarado! Prêmio disponível para saque.");
        Ok(())
    }

    pub fn claim_reward(ctx: Context<ClaimReward>, _event_id: u64) -> Result<()> {
        let event = &ctx.accounts.event_account;

        require!(!event.is_active, HypeError::EventStillActive);
        require!(
            event.winner == Some(ctx.accounts.winner.key()),
            HypeError::NotTheWinner
        );

        let reward = event.reward_amount;
        require!(reward > 0, HypeError::NoRewardAvailable);

        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.organizer.to_account_info(),
                to: ctx.accounts.winner.to_account_info(),
            },
        );
        transfer(cpi_ctx, reward)?;

        msg!("Prêmio de {} lamports enviado instantaneamente ao vencedor!", reward);
        Ok(())
    }
}

#[account]
pub struct EventState {
    pub organizer: Pubkey,
    pub price: u64,
    pub reward_amount: u64,
    pub is_active: bool,
    pub event_id: u64,
    pub total_checked_in: u64,
    pub winner: Option<Pubkey>,
}

#[account]
pub struct TicketAccount {
    pub owner: Pubkey,
    pub event_id: u64,
    pub is_used: bool,
}

#[account]
pub struct UserStats {
    pub attendance_count: u64,
}

#[derive(Accounts)]
#[instruction(event_id: u64)]
pub struct InitializeEvent<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8 + 8 + 1 + 8 + 8 + 1 + 32,
        seeds = [b"event", event_id.to_le_bytes().as_ref()],
        bump
    )]
    pub event_account: Account<'info, EventState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(event_id: u64)]
pub struct BuyTicket<'info> {
    #[account(mut, seeds = [b"event", event_id.to_le_bytes().as_ref()], bump, has_one = organizer)]
    pub event_account: Account<'info, EventState>,

    #[account(
        init,
        payer = buyer,
        space = 8 + 32 + 8 + 1,
        seeds = [b"ticket", buyer.key().as_ref(), event_id.to_le_bytes().as_ref()],
        bump
    )]
    pub ticket_account: Account<'info, TicketAccount>,

    #[account(
        init_if_needed,
        payer = buyer,
        space = 8 + 8,
        seeds = [b"user_stats", buyer.key().as_ref()],
        bump
    )]
    pub user_stats: Account<'info, UserStats>,

    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(mut)]
    /// CHECK: Esta conta é validada pela constraint has_one na event_account
    pub organizer: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(event_id: u64)]
pub struct CheckIn<'info> {
    #[account(mut, seeds = [b"event", event_id.to_le_bytes().as_ref()], bump, has_one = organizer)]
    pub event_account: Account<'info, EventState>,

    #[account(mut, seeds = [b"ticket", ticket_owner.key().as_ref(), event_id.to_le_bytes().as_ref()], bump)]
    pub ticket_account: Account<'info, TicketAccount>,

    #[account(mut, seeds = [b"user_stats", ticket_owner.key().as_ref()], bump)]
    pub user_stats: Account<'info, UserStats>,

    pub organizer: Signer<'info>,
    /// CHECK: Dono do ticket sendo validado pelas seeds e pela verificação em check_in
    pub ticket_owner: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(event_id: u64)]
pub struct DeclareWinner<'info> {
    #[account(mut, seeds = [b"event", event_id.to_le_bytes().as_ref()], bump, has_one = organizer)]
    pub event_account: Account<'info, EventState>,

    pub organizer: Signer<'info>,

    /// CHECK: Endereço do vencedor declarado pelo organizador
    pub winner: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(event_id: u64)]
pub struct ClaimReward<'info> {
    #[account(mut, seeds = [b"event", event_id.to_le_bytes().as_ref()], bump, has_one = organizer)]
    pub event_account: Account<'info, EventState>,

    #[account(mut)]
    /// CHECK: Organizer paga o prêmio, validado pelo has_one
    pub organizer: AccountInfo<'info>,

    #[account(mut)]
    pub winner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum HypeError {
    #[msg("O evento não está ativo.")]
    EventNotActive,
    #[msg("Este ingresso já foi utilizado.")]
    TicketAlreadyUsed,
    #[msg("Você não é o dono deste ingresso.")]
    NotTicketOwner,
    #[msg("O evento ainda está ativo.")]
    EventStillActive,
    #[msg("Você não é o vencedor deste evento.")]
    NotTheWinner,
    #[msg("Não há prêmio disponível.")]
    NoRewardAvailable,
    #[msg("O vencedor já foi declarado.")]
    WinnerAlreadyDeclared,
}