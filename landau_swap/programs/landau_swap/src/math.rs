use anchor_lang::prelude::*;

use crate::error::LandauError;

/// Computes the actual output and fee using the rational resistance curve:
/// s(r) = r^2 / (1 + r^2), where r = (amount_in / reserve_in) / 2.
pub fn compute_rational_trade(
    amount_in: u64,
    reserve_in: u64,
    reserve_out: u64,
) -> Result<(u64, u64)> {
    if amount_in == 0 {
        return Ok((0, 0));
    }
    require!(
        reserve_in > 0 && reserve_out > 0,
        LandauError::EmptyReserves
    );

    let amount_in = amount_in as u128;
    let reserve_in = reserve_in as u128;
    let reserve_out = reserve_out as u128;

    let reserve_in_sq = reserve_in
        .checked_mul(reserve_in)
        .ok_or(LandauError::MathOverflow)?;
    let four_reserve_in_sq = reserve_in_sq
        .checked_mul(4)
        .ok_or(LandauError::MathOverflow)?;

    let amount_in_sq = amount_in
        .checked_mul(amount_in)
        .ok_or(LandauError::MathOverflow)?;
    let denom = amount_in_sq
        .checked_add(four_reserve_in_sq)
        .ok_or(LandauError::MathOverflow)?;

    let four_reserve_in = reserve_in.checked_mul(4).ok_or(LandauError::MathOverflow)?;
    let numerator_actual = amount_in
        .checked_mul(reserve_out)
        .and_then(|v| v.checked_mul(four_reserve_in))
        .ok_or(LandauError::MathOverflow)?;

    let actual_out = numerator_actual
        .checked_div(denom)
        .ok_or(LandauError::MathOverflow)?;

    let theoretical = amount_in
        .checked_mul(reserve_out)
        .and_then(|v| v.checked_div(reserve_in))
        .ok_or(LandauError::MathOverflow)?;

    require!(
        actual_out <= theoretical && actual_out <= reserve_out,
        LandauError::InsufficientReserves
    );

    let fee = theoretical
        .checked_sub(actual_out)
        .ok_or(LandauError::MathOverflow)?;

    let actual_out_u64 = u64::try_from(actual_out).map_err(|_| LandauError::MathOverflow)?;
    let fee_u64 = u64::try_from(fee).map_err(|_| LandauError::MathOverflow)?;
    Ok((actual_out_u64, fee_u64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_trade_has_tiny_fee() {
        let (actual, fee) = compute_rational_trade(1_000, 1_000_000, 1_000_000).unwrap();
        assert!(fee < 5, "fee should be tiny for small trades");
        assert_eq!(actual + fee, 1000);
    }

    #[test]
    fn large_trade_faces_high_fee() {
        let (actual, fee) = compute_rational_trade(500_000, 1_000_000, 1_000_000).unwrap();
        assert!(fee > actual, "fee should dominate for large trades");
    }
}
