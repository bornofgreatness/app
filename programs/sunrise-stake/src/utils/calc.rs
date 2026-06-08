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

#[cfg(test)]
mod tests {
    use super::proportional;

    #[test]
    fn proportional_returns_identity_when_denominator_is_zero() {
        assert_eq!(proportional(1_000, 50, 0).unwrap(), 1_000);
    }

    #[test]
    fn proportional_scales_amount_by_ratio() {
        assert_eq!(proportional(1_000, 250, 1_000).unwrap(), 250);
        assert_eq!(proportional(9, 1, 3).unwrap(), 3);
    }

    #[test]
    fn proportional_truncates_toward_zero() {
        assert_eq!(proportional(10, 1, 3).unwrap(), 3);
    }

    #[test]
    fn proportional_errors_on_u64_overflow() {
        assert!(proportional(u64::MAX, u64::MAX, 1).is_err());
    }
}
