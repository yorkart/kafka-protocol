use super::*;
pub fn api_versions_v0_response<'i, I>() -> impl Parser<I, Output = ApiVersionsV0Response> + 'i
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
        array(|| {
            (
                be_i16().map(|i| ApiKey::from(i)).expected("api_key"),
                be_i16().expected("min_version"),
                be_i16().expected("max_version"),
            )
                .map(|(api_key, min_version, max_version)| ApiVersions {
                    api_key,
                    min_version,
                    max_version,
                })
                .expected("api_versions")
        }),
    )
        .map(|(error_code, api_versions)| ApiVersionsV0Response {
            error_code,
            api_versions,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct ApiVersionsV0Response {
    pub error_code: ErrorCode,
    pub api_versions: Vec<ApiVersions>,
}

impl crate::Encode for ApiVersionsV0Response {
    fn encode_len(&self) -> usize {
        self.error_code.encode_len() + self.api_versions.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.error_code.encode(writer);
        self.api_versions.encode(writer);
    }
}

pub const VERSION: i16 = 0;

#[derive(Clone, Debug, PartialEq)]
pub struct ApiVersions {
    pub api_key: ApiKey,
    pub min_version: i16,
    pub max_version: i16,
}

impl crate::Encode for ApiVersions {
    fn encode_len(&self) -> usize {
        self.api_key.encode_len() + self.min_version.encode_len() + self.max_version.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.api_key.encode(writer);
        self.min_version.encode(writer);
        self.max_version.encode(writer);
    }
}
