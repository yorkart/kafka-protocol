use super::*;
pub fn metadata_v0_request<'i, I>() -> impl Parser<I, Output = MetadataV0Request<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (array(|| string().expected("topics").expected("topics")),)
        .map(|(topics,)| MetadataV0Request { topics })
}

#[derive(Clone, Debug, PartialEq)]
pub struct MetadataV0Request<'i> {
    pub topics: Vec<&'i str>,
}

impl<'i> crate::Encode for MetadataV0Request<'i> {
    fn encode_len(&self) -> usize {
        self.topics.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.topics.encode(writer);
    }
}

pub const VERSION: i16 = 0;
