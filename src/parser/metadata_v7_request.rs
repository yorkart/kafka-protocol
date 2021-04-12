use super::*;
pub fn metadata_v7_request<'i, I>() -> impl Parser<I, Output = MetadataV7Request<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        array(|| string().expected("topics").expected("topics")),
        any().map(|b| b != 0).expected("allow_auto_topic_creation"),
    )
        .map(|(topics, allow_auto_topic_creation)| MetadataV7Request {
            topics,
            allow_auto_topic_creation,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct MetadataV7Request<'i> {
    pub topics: Vec<&'i str>,
    pub allow_auto_topic_creation: bool,
}

impl<'i> crate::Encode for MetadataV7Request<'i> {
    fn encode_len(&self) -> usize {
        self.topics.encode_len() + self.allow_auto_topic_creation.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.topics.encode(writer);
        self.allow_auto_topic_creation.encode(writer);
    }
}

pub const VERSION: i16 = 7;
