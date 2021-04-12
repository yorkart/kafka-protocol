use super::*;
pub fn list_offsets_request<'i, I>() -> impl Parser<I, Output = ListOffsetsRequest<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32().expected("replica_id"),
        array(|| {
            (
                string().expected("topic"),
                array(|| {
                    (
                        be_i32().expected("partition"),
                        be_i64().expected("timestamp"),
                        be_i32().expected("max_num_offsets"),
                    )
                        .map(|(partition, timestamp, max_num_offsets)| Partitions {
                            partition,
                            timestamp,
                            max_num_offsets,
                        })
                        .expected("partitions")
                }),
            )
                .map(|(topic, partitions)| Topics { topic, partitions })
                .expected("topics")
        }),
    )
        .map(|(replica_id, topics)| ListOffsetsRequest { replica_id, topics })
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListOffsetsRequest<'i> {
    pub replica_id: i32,
    pub topics: Vec<Topics<'i>>,
}

impl<'i> crate::Encode for ListOffsetsRequest<'i> {
    fn encode_len(&self) -> usize {
        self.replica_id.encode_len() + self.topics.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.replica_id.encode(writer);
        self.topics.encode(writer);
    }
}

pub const VERSION: i16 = 0;

#[derive(Clone, Debug, PartialEq)]
pub struct Partitions {
    pub partition: i32,
    pub timestamp: i64,
    pub max_num_offsets: i32,
}

impl crate::Encode for Partitions {
    fn encode_len(&self) -> usize {
        self.partition.encode_len()
            + self.timestamp.encode_len()
            + self.max_num_offsets.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.partition.encode(writer);
        self.timestamp.encode(writer);
        self.max_num_offsets.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub topic: &'i str,
    pub partitions: Vec<Partitions>,
}

impl<'i> crate::Encode for Topics<'i> {
    fn encode_len(&self) -> usize {
        self.topic.encode_len() + self.partitions.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.topic.encode(writer);
        self.partitions.encode(writer);
    }
}
