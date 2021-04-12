use super::*;
pub fn metadata_v3_request<'i, I>() -> impl Parser<I, Output = MetadataV3Request<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (array(|| string().expected("topics").expected("topics")),)
        .map(|(topics,)| MetadataV3Request { topics })
}

#[derive(Clone, Debug, PartialEq)]
pub struct MetadataV3Request<'i> {
    pub topics: Vec<&'i str>,
}

impl<'i> crate::Encode for MetadataV3Request<'i> {
    fn encode_len(&self) -> usize {
        self.topics.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.topics.encode(writer);
    }
}

pub const VERSION: i16 = 3;
