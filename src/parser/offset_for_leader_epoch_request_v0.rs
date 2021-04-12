use super::*;
pub fn offset_for_leader_epoch_request<'i, I>(
) -> impl Parser<I, Output = OffsetForLeaderEpochRequest<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (array(|| {
        (
            string().expected("topic"),
            array(|| {
                (
                    be_i32().expected("partition"),
                    be_i32().expected("leader_epoch"),
                )
                    .map(|(partition, leader_epoch)| Partitions {
                        partition,
                        leader_epoch,
                    })
                    .expected("partitions")
            }),
        )
            .map(|(topic, partitions)| Topics { topic, partitions })
            .expected("topics")
    }),)
        .map(|(topics,)| OffsetForLeaderEpochRequest { topics })
}

#[derive(Clone, Debug, PartialEq)]
pub struct OffsetForLeaderEpochRequest<'i> {
    pub topics: Vec<Topics<'i>>,
}

impl<'i> crate::Encode for OffsetForLeaderEpochRequest<'i> {
    fn encode_len(&self) -> usize {
        self.topics.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.topics.encode(writer);
    }
}

pub const VERSION: i16 = 0;

#[derive(Clone, Debug, PartialEq)]
pub struct Partitions {
    pub partition: i32,
    pub leader_epoch: i32,
}

impl crate::Encode for Partitions {
    fn encode_len(&self) -> usize {
        self.partition.encode_len() + self.leader_epoch.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.partition.encode(writer);
        self.leader_epoch.encode(writer);
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
