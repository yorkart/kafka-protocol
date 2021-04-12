use super::*;
pub fn metadata_request<'i, I>() -> impl Parser<I, Output = MetadataRequest<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (array(|| string().expected("topics").expected("topics")),)
        .map(|(topics,)| MetadataRequest { topics })
}

#[derive(Clone, Debug, PartialEq)]
pub struct MetadataRequest<'i> {
    pub topics: Vec<&'i str>,
}

impl<'i> crate::Encode for MetadataRequest<'i> {
    fn encode_len(&self) -> usize {
        self.topics.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.topics.encode(writer);
    }
}

pub const VERSION: i16 = 2;
