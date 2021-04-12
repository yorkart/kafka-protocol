use super::*;
pub fn fetch_request<'i, I>() -> impl Parser<I, Output = FetchRequest<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32().expected("replica_id"),
        be_i32().expected("max_wait_time"),
        be_i32().expected("min_bytes"),
        be_i32().expected("max_bytes"),
        be_i8().expected("isolation_level"),
        array(|| {
            (
                string().expected("topic"),
                array(|| {
                    (
                        be_i32().expected("partition"),
                        be_i64().expected("fetch_offset"),
                        be_i32().expected("partition_max_bytes"),
                    )
                        .map(
                            |(partition, fetch_offset, partition_max_bytes)| Partitions {
                                partition,
                                fetch_offset,
                                partition_max_bytes,
                            },
                        )
                        .expected("partitions")
                }),
            )
                .map(|(topic, partitions)| Topics { topic, partitions })
                .expected("topics")
        }),
    )
        .map(
            |(replica_id, max_wait_time, min_bytes, max_bytes, isolation_level, topics)| {
                FetchRequest {
                    replica_id,
                    max_wait_time,
                    min_bytes,
                    max_bytes,
                    isolation_level,
                    topics,
                }
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct FetchRequest<'i> {
    pub replica_id: i32,
    pub max_wait_time: i32,
    pub min_bytes: i32,
    pub max_bytes: i32,
    pub isolation_level: i8,
    pub topics: Vec<Topics<'i>>,
}

impl<'i> crate::Encode for FetchRequest<'i> {
    fn encode_len(&self) -> usize {
        self.replica_id.encode_len()
            + self.max_wait_time.encode_len()
            + self.min_bytes.encode_len()
            + self.max_bytes.encode_len()
            + self.isolation_level.encode_len()
            + self.topics.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.replica_id.encode(writer);
        self.max_wait_time.encode(writer);
        self.min_bytes.encode(writer);
        self.max_bytes.encode(writer);
        self.isolation_level.encode(writer);
        self.topics.encode(writer);
    }
}

pub const VERSION: i16 = 4;

#[derive(Clone, Debug, PartialEq)]
pub struct Partitions {
    pub partition: i32,
    pub fetch_offset: i64,
    pub partition_max_bytes: i32,
}

impl crate::Encode for Partitions {
    fn encode_len(&self) -> usize {
        self.partition.encode_len()
            + self.fetch_offset.encode_len()
            + self.partition_max_bytes.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.partition.encode(writer);
        self.fetch_offset.encode(writer);
        self.partition_max_bytes.encode(writer);
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
