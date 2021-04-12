use super::*;
pub fn produce_v2_response<'i, I>() -> impl Parser<I, Output = ProduceV2Response<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        array(|| {
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
                        be_i64().expected("base_offset"),
                        be_i64().expected("log_append_time"),
                    )
                        .map(|(partition, error_code, base_offset, log_append_time)| {
                            PartitionResponses {
                                partition,
                                error_code,
                                base_offset,
                                log_append_time,
                            }
                        })
                        .expected("partition_responses")
                }),
            )
                .map(|(topic, partition_responses)| Responses {
                    topic,
                    partition_responses,
                })
                .expected("responses")
        }),
        be_i32().expected("throttle_time_ms"),
    )
        .map(|(responses, throttle_time_ms)| ProduceV2Response {
            responses,
            throttle_time_ms,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProduceV2Response<'i> {
    pub responses: Vec<Responses<'i>>,
    pub throttle_time_ms: i32,
}

impl<'i> crate::Encode for ProduceV2Response<'i> {
    fn encode_len(&self) -> usize {
        self.responses.encode_len() + self.throttle_time_ms.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.responses.encode(writer);
        self.throttle_time_ms.encode(writer);
    }
}

pub const VERSION: i16 = 2;

#[derive(Clone, Debug, PartialEq)]
pub struct PartitionResponses {
    pub partition: i32,
    pub error_code: ErrorCode,
    pub base_offset: i64,
    pub log_append_time: i64,
}

impl crate::Encode for PartitionResponses {
    fn encode_len(&self) -> usize {
        self.partition.encode_len()
            + self.error_code.encode_len()
            + self.base_offset.encode_len()
            + self.log_append_time.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.partition.encode(writer);
        self.error_code.encode(writer);
        self.base_offset.encode(writer);
        self.log_append_time.encode(writer);
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
