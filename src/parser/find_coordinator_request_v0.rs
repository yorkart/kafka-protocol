use super::*;
pub fn find_coordinator_request<'i, I>() -> impl Parser<I, Output = FindCoordinatorRequest<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (string().expected("key"),).map(|(key,)| FindCoordinatorRequest { key })
}

#[derive(Clone, Debug, PartialEq)]
pub struct FindCoordinatorRequest<'i> {
    pub key: &'i str,
}

impl<'i> crate::Encode for FindCoordinatorRequest<'i> {
    fn encode_len(&self) -> usize {
        self.key.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.key.encode(writer);
    }
}

pub const VERSION: i16 = 0;
