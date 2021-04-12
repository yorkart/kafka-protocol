use super::*;
pub fn list_offsets_response<'i, I>() -> impl Parser<I, Output = ListOffsetsResponse<'i>> + 'i
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
                    be_i16()
                        .and_then(|i| {
                            ErrorCode::try_from(i)
                                .map_err(StreamErrorFor::<I>::unexpected_static_message)
                        })
                        .expected("error_code"),
                    be_i64().expected("timestamp"),
                    be_i64().expected("offset"),
                )
                    .map(
                        |(partition, error_code, timestamp, offset)| PartitionResponses {
                            partition,
                            error_code,
                            timestamp,
                            offset,
                        },
                    )
                    .expected("partition_responses")
            }),
        )
            .map(|(topic, partition_responses)| Responses {
                topic,
                partition_responses,
            })
            .expected("responses")
    }),)
        .map(|(responses,)| ListOffsetsResponse { responses })
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListOffsetsResponse<'i> {
    pub responses: Vec<Responses<'i>>,
}

impl<'i> crate::Encode for ListOffsetsResponse<'i> {
    fn encode_len(&self) -> usize {
        self.responses.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.responses.encode(writer);
    }
}

pub const VERSION: i16 = 1;

#[derive(Clone, Debug, PartialEq)]
pub struct PartitionResponses {
    pub partition: i32,
    pub error_code: ErrorCode,
    pub timestamp: i64,
    pub offset: i64,
}

impl crate::Encode for PartitionResponses {
    fn encode_len(&self) -> usize {
        self.partition.encode_len()
            + self.error_code.encode_len()
            + self.timestamp.encode_len()
            + self.offset.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.partition.encode(writer);
        self.error_code.encode(writer);
        self.timestamp.encode(writer);
        self.offset.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Responses<'i> {
    pub topic: &'i str,
    pub partition_responses: Vec<PartitionResponses>,
}

impl<'i> crate::Encode for Responses<'i> {
    fn encode_len(&self) -> usize {
        self.topic.encode_len() + self.partition_responses.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.topic.encode(writer);
        self.partition_responses.encode(writer);
    }
}
