use super::*;
pub fn find_coordinator_response<'i, I>(
) -> impl Parser<I, Output = FindCoordinatorResponse<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i16()
            .and_then(|i| {
                ErrorCode::try_from(i).map_err(StreamErrorFor::<I>::unexpected_static_message)
            })
            .expected("error_code"),
        be_i32().expected("node_id"),
        string().expected("host"),
        be_i32().expected("port"),
    )
        .map(
            |(error_code, node_id, host, port)| FindCoordinatorResponse {
                error_code,
                node_id,
                host,
                port,
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct FindCoordinatorResponse<'i> {
    pub error_code: ErrorCode,
    pub node_id: i32,
    pub host: &'i str,
    pub port: i32,
}

impl<'i> crate::Encode for FindCoordinatorResponse<'i> {
    fn encode_len(&self) -> usize {
        self.error_code.encode_len()
            + self.node_id.encode_len()
            + self.host.encode_len()
            + self.port.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.error_code.encode(writer);
        self.node_id.encode(writer);
        self.host.encode(writer);
        self.port.encode(writer);
    }
}

pub const VERSION: i16 = 0;
