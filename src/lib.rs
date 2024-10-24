use ethnum::U256;

pub type ErrorCode = u16;

pub const ARITHMETIC_OVERFLOW: ErrorCode = 9003;
pub const AMOUNT_EXCEEDS_MAX_U64: ErrorCode = 9004;

/// Get the initializable tick index.
/// If the tick index is already initializable, it is returned as is.
///
/// # Parameters
/// - `tick_index` - A i32 integer representing the tick integer
/// - `tick_spacing` - A i32 integer representing the tick spacing
/// - `round_up` - A boolean value indicating if the supplied tick index should be rounded up. None will round to the nearest.
///
/// # Returns
/// - A i32 integer representing the previous initializable tick index
pub fn get_initializable_tick_index(
    tick_index: i32,
    tick_spacing: u16,
    round_up: Option<bool>,
) -> i32 {
    let tick_spacing_i32 = tick_spacing as i32;
    let remainder = tick_index % tick_spacing_i32;
    let result = tick_index / tick_spacing_i32 * tick_spacing_i32;

    let should_round_up = if let Some(round_up) = round_up {
        round_up && remainder > 0
    } else {
        remainder >= tick_spacing_i32 / 2
    };

    if should_round_up {
        result + tick_spacing_i32
    } else {
        result
    }
}

/// Check if a tick is initializable.
/// A tick is initializable if it is divisible by the tick spacing.
///
/// # Parameters
/// - `tick_index` - A i32 integer representing the tick integer
/// - `tick_spacing` - A i32 integer representing the tick spacing
///
/// # Returns
/// - A boolean value indicating if the tick is initializable
pub fn is_tick_initializable(tick_index: i32, tick_spacing: u16) -> bool {
    let tick_spacing_i32 = tick_spacing as i32;
    tick_index % tick_spacing_i32 == 0
}

/// Calculate the amount-delta between two sqrt_prices
///
/// # Parameters
/// - `current_sqrt_price`: The current square root price
/// - `target_sqrt_price`: The target square root price
/// - `current_liquidity`: The current liquidity
/// - `round_up`: Whether to round up or not
///
/// # Returns
/// - `u64`: The amount delta
pub fn try_get_amount_delta(
    sqrt_price_1: u128,
    sqrt_price_2: u128,
    liquidity: u128,
    round_up: bool,
) -> Result<u64, ErrorCode> {
    let (sqrt_price_lower, sqrt_price_upper) =
        order_prices(sqrt_price_1.into(), sqrt_price_2.into());
    let sqrt_price_diff = sqrt_price_upper - sqrt_price_lower;
    let numerator: U256 = <U256>::from(liquidity)
        .checked_mul(sqrt_price_diff.into())
        .ok_or(ARITHMETIC_OVERFLOW)?
        .checked_shl(64)
        .ok_or(ARITHMETIC_OVERFLOW)?;

    let denominator: U256 = <U256>::from(sqrt_price_lower)
        .checked_mul(sqrt_price_upper.into())
        .ok_or(ARITHMETIC_OVERFLOW)?;

    let quotient = numerator / denominator;
    let remainder = numerator % denominator;

    let result = if round_up && remainder != 0 {
        quotient + 1
    } else {
        quotient
    };

    result.try_into().map_err(|_| AMOUNT_EXCEEDS_MAX_U64)
}

// Private functions

fn order_prices(a: u128, b: u128) -> (u128, u128) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}
