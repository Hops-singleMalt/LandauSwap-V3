use anchor_lang::prelude::*;

pub mod error;
pub mod math;
pub mod state;

use error::LandauError;
use math::compute_rational_trade;
use state::{BatchTotals, CurveType, Pool, TradeDirection, POOL_SEED};

declare_id!("2Rq56TYSfZzQiKfScXu7bDPpLuoFSFdjSAh4ZFRrRDXJ");

#[program]
pub mod landau_swap {
    use super::*;

    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        bump: u8,
        curve_type: CurveType,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.authority = ctx.accounts.authority.key();
        pool.token_mint_a = ctx.accounts.token_mint_a.key();
        pool.token_mint_b = ctx.accounts.token_mint_b.key();
        pool.vault_a = ctx.accounts.vault_a.key();
        pool.vault_b = ctx.accounts.vault_b.key();
        pool.reserve_a = 0;
        pool.reserve_b = 0;
        pool.accumulated_fee_a = 0;
        pool.accumulated_fee_b = 0;
        pool.batch_totals = BatchTotals::default();
        pool.curve_type = curve_type;
        pool.bump = bump;
        pool.padding = [0; 6];
        Ok(())
    }

    pub fn add_liquidity(
        ctx: Context<ModifyLiquidity>,
        amount_a: u64,
        amount_b: u64,
    ) -> Result<()> {
        require!(amount_a > 0 || amount_b > 0, LandauError::InvalidDirection);
        let pool = &mut ctx.accounts.pool;

        if amount_a > 0 {
            pool.reserve_a = pool
                .reserve_a
                .checked_add(amount_a)
                .ok_or(LandauError::MathOverflow)?;
        }

        if amount_b > 0 {
            pool.reserve_b = pool
                .reserve_b
                .checked_add(amount_b)
                .ok_or(LandauError::MathOverflow)?;
        }

        msg!("Liquidity added: A={}, B={}", amount_a, amount_b);
        Ok(())
    }

    pub fn remove_liquidity(
        ctx: Context<ModifyLiquidity>,
        amount_a: u64,
        amount_b: u64,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        require!(
            amount_a <= pool.reserve_a && amount_b <= pool.reserve_b,
            LandauError::InsufficientReserves
        );

        if amount_a > 0 {
            pool.reserve_a = pool
                .reserve_a
                .checked_sub(amount_a)
                .ok_or(LandauError::MathOverflow)?;
        }

        if amount_b > 0 {
            pool.reserve_b = pool
                .reserve_b
                .checked_sub(amount_b)
                .ok_or(LandauError::MathOverflow)?;
        }

        msg!("Liquidity removed: A={}, B={}", amount_a, amount_b);
        Ok(())
    }

    pub fn place_order(
        ctx: Context<PlaceOrder>,
        direction: TradeDirection,
        amount: u64,
    ) -> Result<()> {
        require!(amount > 0, LandauError::InvalidDirection);

        let pool = &mut ctx.accounts.pool;
        let batch = &mut pool.batch_totals;
        let amount_i128 = i128::try_from(amount).map_err(|_| LandauError::MathOverflow)?;

        match direction {
            TradeDirection::AForB => {
                batch.net_delta_a = batch
                    .net_delta_a
                    .checked_add(amount_i128)
                    .ok_or(LandauError::MathOverflow)?;
            }
            TradeDirection::BForA => {
                batch.net_delta_b = batch
                    .net_delta_b
                    .checked_add(amount_i128)
                    .ok_or(LandauError::MathOverflow)?;
            }
        }

        batch.order_count = batch
            .order_count
            .checked_add(1)
            .ok_or(LandauError::MathOverflow)?;
        batch.last_updated_slot = Clock::get()?.slot;

        msg!("Order placed: direction={:?}, amount={}", direction, amount);
        Ok(())
    }

    pub fn settle_batch(ctx: Context<SettleBatch>) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        let batch = &mut pool.batch_totals;
        require!(batch.order_count > 0, LandauError::EmptyBatch);

        let curve_type = pool.curve_type;
        let clock = Clock::get()?;

        if batch.net_delta_a > 0 && batch.net_delta_b == 0 {
            let amount_in =
                u64::try_from(batch.net_delta_a).map_err(|_| LandauError::MathOverflow)?;
            let (actual_out, fee) = match curve_type {
                CurveType::Rational => {
                    compute_rational_trade(amount_in, pool.reserve_a, pool.reserve_b)?
                }
                CurveType::Exponential => return err!(LandauError::UnsupportedCurve),
            };

            pool.reserve_a = pool
                .reserve_a
                .checked_add(amount_in)
                .ok_or(LandauError::MathOverflow)?;
            require!(
                actual_out <= pool.reserve_b,
                LandauError::InsufficientReserves
            );
            pool.reserve_b = pool
                .reserve_b
                .checked_sub(actual_out)
                .ok_or(LandauError::MathOverflow)?;
            pool.accumulated_fee_b = pool
                .accumulated_fee_b
                .checked_add(fee)
                .ok_or(LandauError::MathOverflow)?;

            msg!(
                "Batch settled (A→B): in={}, out={}, fee={}",
                amount_in,
                actual_out,
                fee
            );
        } else if batch.net_delta_b > 0 && batch.net_delta_a == 0 {
            let amount_in =
                u64::try_from(batch.net_delta_b).map_err(|_| LandauError::MathOverflow)?;
            let (actual_out, fee) = match curve_type {
                CurveType::Rational => {
                    compute_rational_trade(amount_in, pool.reserve_b, pool.reserve_a)?
                }
                CurveType::Exponential => return err!(LandauError::UnsupportedCurve),
            };

            pool.reserve_b = pool
                .reserve_b
                .checked_add(amount_in)
                .ok_or(LandauError::MathOverflow)?;
            require!(
                actual_out <= pool.reserve_a,
                LandauError::InsufficientReserves
            );
            pool.reserve_a = pool
                .reserve_a
                .checked_sub(actual_out)
                .ok_or(LandauError::MathOverflow)?;
            pool.accumulated_fee_a = pool
                .accumulated_fee_a
                .checked_add(fee)
                .ok_or(LandauError::MathOverflow)?;

            msg!(
                "Batch settled (B→A): in={}, out={}, fee={}",
                amount_in,
                actual_out,
                fee
            );
        } else {
            return err!(LandauError::MixedBatchDirections);
        }

        batch.net_delta_a = 0;
        batch.net_delta_b = 0;
        batch.order_count = 0;
        batch.last_updated_slot = clock.slot;
        batch.batch_id = batch
            .batch_id
            .checked_add(1)
            .ok_or(LandauError::MathOverflow)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = Pool::LEN,
        seeds = [POOL_SEED, token_mint_a.key().as_ref(), token_mint_b.key().as_ref()],
        bump
    )]
    pub pool: Account<'info, Pool>,
    /// CHECK: validated off-chain for MVP
    pub token_mint_a: UncheckedAccount<'info>,
    /// CHECK: validated off-chain for MVP
    pub token_mint_b: UncheckedAccount<'info>,
    /// CHECK: vault handling deferred to MVP demo interfaces.
    pub vault_a: UncheckedAccount<'info>,
    /// CHECK: vault handling deferred to MVP demo interfaces.
    pub vault_b: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyLiquidity<'info> {
    #[account(mut, has_one = authority)]
    pub pool: Account<'info, Pool>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct PlaceOrder<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    pub trader: Signer<'info>,
}

#[derive(Accounts)]
pub struct SettleBatch<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    pub settler: Signer<'info>,
}
