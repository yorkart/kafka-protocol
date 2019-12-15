use super::*;
pub fn elect_preferred_leaders_request<'i, I>(
) -> impl Parser<I, Output = ElectPreferredLeadersRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        optional(
            (string(), optional(be_i32())).map(|(topic, partition_id)| TopicPartitions {
                topic,
                partition_id,
            }),
        ),
        be_i32(),
    )
        .map(
            |(topic_partitions, timeout_ms)| ElectPreferredLeadersRequest {
                topic_partitions,
                timeout_ms,
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct ElectPreferredLeadersRequest<'i> {
    pub topic_partitions: Option<TopicPartitions<'i>>,
    pub timeout_ms: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TopicPartitions<'i> {
    pub topic: &'i str,
    pub partition_id: Option<i32>,
}
