use crate::ErrorCode;
use anchor_lang::prelude::*;

/// calculate amount*numerator/denominator
/// as value  = shares * share_price where share_price=total_value/total_shares
/// or shares = amount_value / share_price where share_price=total_value/total_shares
///     => shares = amount_value * 1/share_price where 1/share_price=total_shares/total_value
pub fn proportional(amount: u64, numerator: u64, denominator: u64) -> Result<u64> {
    if denominator == 0 {
        return Ok(amount);
    }
    u64::try_from((amount as u128) * (numerator as u128) / (denominator as u128))
        .map_err(|_| error!(ErrorCode::CalculationFailure))
}

/// Quote leg received when `base_fill` is taken from `base_reserve` at the
/// current pool ratio (`quote_reserve / base_reserve`). Same proportional
/// primitive used for stake-pool withdrawals and partial-fill sizing.
pub fn fill_quote_from_base(
    base_fill: u64,
    base_reserve: u64,
    quote_reserve: u64,
) -> Result<u64> {
    require!(base_fill > 0, ErrorCode::InvalidCalculation);
    require!(base_reserve > 0, ErrorCode::InvalidCalculation);
    proportional(quote_reserve, base_fill, base_reserve)
}

/// Base leg received when `quote_fill` is paid against `quote_reserve` at the
/// current pool ratio (`base_reserve / quote_reserve`).
pub fn fill_base_from_quote(
    quote_fill: u64,
    base_reserve: u64,
    quote_reserve: u64,
) -> Result<u64> {
    require!(quote_fill > 0, ErrorCode::InvalidCalculation);
    require!(quote_reserve > 0, ErrorCode::InvalidCalculation);
    proportional(base_reserve, quote_fill, quote_reserve)
}

#[cfg(test)]
mod tests {
    use super::{fill_base_from_quote, fill_quote_from_base, proportional};

    #[test]
    fn proportional_returns_identity_when_denominator_is_zero() {
        assert_eq!(proportional(1_000, 50, 0).unwrap(), 1_000);
    }

    #[test]
    fn fill_quote_from_base_matches_pool_ratio() {
        assert_eq!(fill_quote_from_base(100, 1_000, 5_000).unwrap(), 500);
    }

    #[test]
    fn fill_base_from_quote_matches_pool_ratio() {
        assert_eq!(fill_base_from_quote(500, 1_000, 5_000).unwrap(), 100);
    }

    #[test]
    fn fill_helpers_reject_zero_fill_or_reserve() {
        assert!(fill_quote_from_base(0, 1_000, 5_000).is_err());
        assert!(fill_quote_from_base(100, 0, 5_000).is_err());
        assert!(fill_base_from_quote(0, 1_000, 5_000).is_err());
        assert!(fill_base_from_quote(500, 1_000, 0).is_err());
    }
}
