use super::*;
pub fn produce_v0_response<'i, I>() -> impl Parser<I, Output = ProduceV0Response<'i>> + 'i
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
                    be_i64().expected("base_offset"),
                )
                    .map(|(partition, error_code, base_offset)| PartitionResponses {
                        partition,
                        error_code,
                        base_offset,
                    })
                    .expected("partition_responses")
            }),
        )
            .map(|(topic, partition_responses)| Responses {
                topic,
                partition_responses,
            })
            .expected("responses")
    }),)
        .map(|(responses,)| ProduceV0Response { responses })
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProduceV0Response<'i> {
    pub responses: Vec<Responses<'i>>,
}

impl<'i> crate::Encode for ProduceV0Response<'i> {
    fn encode_len(&self) -> usize {
        self.responses.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.responses.encode(writer);
    }
}

pub const VERSION: i16 = 0;

#[derive(Clone, Debug, PartialEq)]
pub struct PartitionResponses {
    pub partition: i32,
    pub error_code: ErrorCode,
    pub base_offset: i64,
}

impl crate::Encode for PartitionResponses {
    fn encode_len(&self) -> usize {
        self.partition.encode_len() + self.error_code.encode_len() + self.base_offset.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.partition.encode(writer);
        self.error_code.encode(writer);
        self.base_offset.encode(writer);
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
