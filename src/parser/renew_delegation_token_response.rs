use super::*;
pub fn renew_delegation_token_response<'i, I>(
) -> impl Parser<I, Output = RenewDelegationTokenResponse>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (be_i16(), be_i64(), be_i32()).map(|(error_code, expiry_timestamp, throttle_time_ms)| {
        RenewDelegationTokenResponse {
            error_code,
            expiry_timestamp,
            throttle_time_ms,
        }
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct RenewDelegationTokenResponse {
    pub error_code: i16,
    pub expiry_timestamp: i64,
    pub throttle_time_ms: i32,
}
