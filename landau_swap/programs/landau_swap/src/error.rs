use anchor_lang::prelude::*;

#[error_code]
pub enum LandauError {
    #[msg("Math overflow during computation")]
    MathOverflow,
    #[msg("Pool reserves are empty")]
    EmptyReserves,
    #[msg("Batch has no orders to settle")]
    EmptyBatch,
    #[msg("Batch contains mixed order directions")]
    MixedBatchDirections,
    #[msg("Unsupported curve type for this operation")]
    UnsupportedCurve,
    #[msg("Trade amount exceeds pool reserves")]
    InsufficientReserves,
    #[msg("Invalid trade direction")]
    InvalidDirection,
}
