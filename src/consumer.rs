use std::{collections::BTreeMap, convert::TryFrom, io, time::Duration};

use tokio::io::{AsyncRead, AsyncWrite};

use crate::{
    client::Client,
    parser::{fetch_response, FetchRequest, FetchResponse, ListOffsetsRequest},
    Compression, ErrorCode, RawRecords, Record, RecordBatch, Result, FETCH_LATEST_OFFSET,
};

pub struct Consumer<I> {
    client: Client<I>,
    fetch_offsets: BTreeMap<String, i64>,
}

pub(crate) struct Decoder {
    #[cfg(feature = "snap")]
    snap: snap::raw::Decoder,
    #[cfg(feature = "snap")]
    buf: Vec<u8>,
}

impl Decoder {
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "snap")]
            snap: snap::raw::Decoder::new(),
            #[cfg(feature = "snap")]
            buf: vec![],
        }
    }

    pub fn decompress<'a>(
        &'a mut self,
        compression: Compression,
        input: &'a [u8],
    ) -> Result<&'a [u8]> {
        Ok(match compression {
            Compression::None => input,
            #[cfg(feature = "snap")]
            Compression::Snappy => {
                // TODO unwraps and resize
                self.buf
                    .resize(snap::raw::decompress_len(input).unwrap(), 0);
                let len = self
                    .snap
                    .decompress(input, &mut self.buf)
                    .unwrap_or_else(|err| panic!("{}", err));
                &self.buf[..len]
            }
            _ => return Err(format!("Unsupported compression {:?}", compression).into()),
        })
    }
}

impl Consumer<tokio::net::TcpStream> {
    pub async fn connect(addr: impl tokio::net::ToSocketAddrs) -> io::Result<Self> {
        let client = Client::connect(addr).await?;

        Ok(Self {
            client,
            fetch_offsets: Default::default(),
        })
    }
}

pub struct FetchedRecords<'a> {
    response: FetchResponse<'a, Option<RecordBatch<RawRecords<'a>>>>,
    responses: Option<fetch_response::Responses<'a, Option<RecordBatch<RawRecords<'a>>>>>,
    partition_responses:
        Option<fetch_response::PartitionResponses<Option<RecordBatch<RawRecords<'a>>>>>,
    decoder: Decoder,
}

impl FetchedRecords<'_> {
    pub fn next_batch(&mut self) -> Option<impl Iterator<Item = (&str, Record<'_>)>> {
        let (topic, record_batch) = 'outer: loop {
            if let Some(responses) = &mut self.responses {
                loop {
                    if let Some(partition_responses) = &mut self.partition_responses {
                        if let Some(record_set) = partition_responses.record_set.take() {
                            break 'outer (responses.topic, record_set);
                        }
                    }
                    self.partition_responses = responses.partition_responses.pop();
                    if self.partition_responses.is_none() {
                        break;
                    }
                }
            }

            self.responses = self.response.responses.pop();
            if self.responses.is_none() {
                return None;
            }
        };
        use combine::Parser;
        let input = self
            .decoder
            .decompress(
                record_batch.attributes.compression(),
                record_batch.records.bytes,
            )
            .unwrap();
        // TODO unwraps
        let count = usize::try_from(record_batch.records.count).unwrap();
        let (value, _rest) =
            combine::parser::repeat::count_min_max(count, count, crate::parser::record())
                .parse(input)
                .unwrap();
        let value: Vec<_> = value;
        Some(
            value
                .into_iter()
                .map(move |record| (topic, Record::from(record))),
        )
    }
}

impl<I> Consumer<I>
where
    I: AsyncRead + AsyncWrite + std::marker::Unpin,
{
    pub async fn fetch<'a>(
        &'a mut self,
        topics: impl IntoIterator<Item = &'a str>,
    ) -> Result<FetchedRecords<'a>> {
        let response = self.fetch_raw(topics).await?;
        Ok(FetchedRecords {
            response,
            responses: None,
            partition_responses: None,
            decoder: Decoder::new(),
        })
    }

    async fn fetch_raw<'a, 'b>(
        &'a mut self,
        topics: impl IntoIterator<Item = &'b str>,
    ) -> Result<FetchResponse<'a, Option<RecordBatch<RawRecords<'a>>>>> {
        let topics: Vec<_> = topics.into_iter().collect();
        let mut fetch_topics = Vec::with_capacity(topics.len());
        let mut list_topics = Vec::new();
        for topic in &topics {
            if let Some(&fetch_offset) = self.fetch_offsets.get(*topic) {
                fetch_topics.push(crate::parser::fetch_request::Topics {
                    topic,
                    partitions: vec![crate::parser::fetch_request::Partitions {
                        current_leader_epoch: 0,
                        fetch_offset,
                        log_start_offset: 0,
                        partition: 0,
                        partition_max_bytes: 1024 * 128,
                    }],
                });
            } else {
                list_topics.push(crate::parser::list_offsets_request::Topics {
                    topic,
                    partitions: vec![crate::parser::list_offsets_request::Partitions {
                        partition: 0,
                        timestamp: FETCH_LATEST_OFFSET,
                        current_leader_epoch: 0,
                    }],
                });
            }
        }

        if !list_topics.is_empty() {
            let list_offsets = self
                .client
                .list_offsets(ListOffsetsRequest {
                    replica_id: 0,
                    isolation_level: 0,
                    topics: list_topics,
                })
                .await
                .unwrap();
            assert_eq!(
                list_offsets.responses[0].partition_responses[0].error_code,
                ErrorCode::None,
                "{:#?}",
                list_offsets
            );

            for response in list_offsets.responses {
                assert_eq!(response.partition_responses.len(), 1);
                let fetch_offset = response.partition_responses[0].offset;
                self.fetch_offsets
                    .insert(response.topic.into(), fetch_offset);
                fetch_topics.push(crate::parser::fetch_request::Topics {
                    topic: topics
                        .iter()
                        .find(|topic| **topic == response.topic)
                        .unwrap(),
                    partitions: vec![crate::parser::fetch_request::Partitions {
                        current_leader_epoch: 0,
                        fetch_offset,
                        log_start_offset: 0,
                        partition: 0,
                        partition_max_bytes: 1024 * 128,
                    }],
                });
            }
        }

        self.client
            .fetch(FetchRequest {
                replica_id: -1,
                session_epoch: 0,
                forgotten_topics_data: Vec::new(),
                isolation_level: 0,
                session_id: 0,
                min_bytes: 1,
                max_bytes: 1024 * 1024,
                rack_id: "",
                max_wait_time: i32::try_from(Duration::from_millis(1000).as_millis()).unwrap(),
                topics: fetch_topics,
            })
            .await
    }

    pub fn commit(&mut self) -> io::Result<()> {
        Ok(())
    }
}
