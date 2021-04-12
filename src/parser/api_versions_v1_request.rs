use super::*;
pub fn api_versions_v1_request<'i, I>() -> impl Parser<I, Output = ApiVersionsV1Request> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    value(ApiVersionsV1Request {})
}

#[derive(Clone, Debug, PartialEq)]
pub struct ApiVersionsV1Request {}

impl crate::Encode for ApiVersionsV1Request {
    fn encode_len(&self) -> usize {
        0
    }
    fn encode(&self, _: &mut impl Buffer) {}
}

pub const VERSION: i16 = 1;
