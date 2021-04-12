use super::*;
pub fn describe_groups_request<'i, I>() -> impl Parser<I, Output = DescribeGroupsRequest<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (array(|| string().expected("groups").expected("groups")),)
        .map(|(groups,)| DescribeGroupsRequest { groups })
}

#[derive(Clone, Debug, PartialEq)]
pub struct DescribeGroupsRequest<'i> {
    pub groups: Vec<&'i str>,
}

impl<'i> crate::Encode for DescribeGroupsRequest<'i> {
    fn encode_len(&self) -> usize {
        self.groups.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.groups.encode(writer);
    }
}

pub const VERSION: i16 = 0;
