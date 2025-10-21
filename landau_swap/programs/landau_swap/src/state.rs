use anchor_lang::prelude::*;

pub const POOL_SEED: &[u8] = b"pool";

/// Available impact curve configurations for the pool.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum CurveType {
    /// Rational saturation curve: s(r) = r^2 / (1 + r^2)
    #[default]
    Rational = 0,
    /// Exponential damping curve: s(r) = 1 - exp(-r^2)
    Exponential = 1,
}

/// Direction for a submitted trade in the batch queue.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum TradeDirection {
    /// Trader provides token A and receives token B.
    AForB = 0,
    /// Trader provides token B and receives token A.
    BForA = 1,
}

/// Aggregated totals for the current batch window.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
pub struct BatchTotals {
    /// Monotonically increasing identifier for batches.
    pub batch_id: u64,
    /// Net quantity of token A submitted during the batch (positive = inflow).
    pub net_delta_a: i128,
    /// Net quantity of token B submitted during the batch (positive = inflow).
    pub net_delta_b: i128,
    /// Number of individual orders aggregated into the batch.
    pub order_count: u32,
    /// Last slot that touched this batch (helps enforce batch windows).
    pub last_updated_slot: u64,
}

#[account]
pub struct Pool {
    /// Authority allowed to configure the pool (e.g. update params).
    pub authority: Pubkey,
    /// Mint of token A in the trading pair.
    pub token_mint_a: Pubkey,
    /// Mint of token B in the trading pair.
    pub token_mint_b: Pubkey,
    /// Vault token account holding token A reserves.
    pub vault_a: Pubkey,
    /// Vault token account holding token B reserves.
    pub vault_b: Pubkey,
    /// Current on-chain reserves for token A.
    pub reserve_a: u64,
    /// Current on-chain reserves for token B.
    pub reserve_b: u64,
    /// Accumulated fees denominated in token A.
    pub accumulated_fee_a: u64,
    /// Accumulated fees denominated in token B.
    pub accumulated_fee_b: u64,
    /// Aggregated order totals waiting to be settled.
    pub batch_totals: BatchTotals,
    /// Selected resistance curve for slippage computation.
    pub curve_type: CurveType,
    /// Bump seed used for PDA derivation.
    pub bump: u8,
    /// Padding for future extensions / 8-byte alignment.
    pub padding: [u8; 6],
}

impl Pool {
    /// Size of the account (including discriminator) required for allocation.
    pub const LEN: usize = 8 /* discriminator */
        + 32  // authority
        + 32  // token_mint_a
        + 32  // token_mint_b
        + 32  // vault_a
        + 32  // vault_b
        + 8   // reserve_a
        + 8   // reserve_b
        + 8   // accumulated_fee_a
        + 8   // accumulated_fee_b
        + BatchTotals::LEN
        + 1   // curve_type
        + 1   // bump
        + 6; // padding
}

impl BatchTotals {
    pub const LEN: usize = 8  // batch_id
        + 16 // net_delta_a
        + 16 // net_delta_b
        + 4  // order_count
        + 8; // last_updated_slot
}
