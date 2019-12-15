use super::*;
pub fn offset_commit_request<'i, I>() -> impl Parser<I, Output = OffsetCommitRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        string(),
        be_i32(),
        string(),
        nullable_string(),
        optional(
            (
                string(),
                optional((be_i32(), be_i64(), be_i32(), nullable_string()).map(
                    |(
                        partition_index,
                        committed_offset,
                        committed_leader_epoch,
                        committed_metadata,
                    )| {
                        Partitions {
                            partition_index,
                            committed_offset,
                            committed_leader_epoch,
                            committed_metadata,
                        }
                    },
                )),
            )
                .map(|(name, partitions)| Topics { name, partitions }),
        ),
    )
        .map(
            |(group_id, generation_id, member_id, group_instance_id, topics)| OffsetCommitRequest {
                group_id,
                generation_id,
                member_id,
                group_instance_id,
                topics,
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct OffsetCommitRequest<'i> {
    pub group_id: &'i str,
    pub generation_id: i32,
    pub member_id: &'i str,
    pub group_instance_id: Option<&'i str>,
    pub topics: Option<Topics<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Partitions<'i> {
    pub partition_index: i32,
    pub committed_offset: i64,
    pub committed_leader_epoch: i32,
    pub committed_metadata: Option<&'i str>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub name: &'i str,
    pub partitions: Option<Partitions<'i>>,
}
