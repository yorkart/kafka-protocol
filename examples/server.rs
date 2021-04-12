use std::mem;
use std::net::SocketAddr;

use bytes::{BufMut, BytesMut};
use combine::parser::Parser;
use combine::EasyParser;
use futures::{SinkExt, StreamExt};
use kafka_protocol::api_key::ApiKey;
use kafka_protocol::parser::request_header;
use kafka_protocol::parser::response_header;
use kafka_protocol::{Encode, Record, RecordBatch};
use tokio::net::tcp::ReadHalf;
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::{BytesCodec, FramedRead, FramedWrite, LengthDelimitedCodec};

#[tokio::main]
async fn main() {
    serve(
        "10.181.160.207:9092".parse().unwrap(),
        "10.100.49.2:9092".parse().unwrap(),
    )
    .await
    .unwrap();
}

pub async fn serve(serve_addr: SocketAddr, proxy_addr: SocketAddr) -> std::io::Result<()> {
    let listener = TcpListener::bind(serve_addr).await?;

    while let Ok((mut inbound, _)) = listener.accept().await {
        println!("accept addr {}", inbound.peer_addr().unwrap());

        tokio::spawn(async move {
            let (in_read_half, in_write_half) = inbound.split();
            let mut in_framed_write = FramedWrite::new(in_write_half, BytesCodec::new());
            let mut in_framed_read: FramedRead<ReadHalf, LengthDelimitedCodec> =
                LengthDelimitedCodec::builder()
                    .length_field_offset(0)
                    .length_field_length(4)
                    .length_adjustment(4)
                    .num_skip(0)
                    // .max_frame_length(self.tcp_frame_max_size as usize)
                    .big_endian()
                    .new_read(in_read_half);

            let mut outbound = TcpStream::connect(proxy_addr).await.unwrap();
            let (out_read_half, out_write_half) = outbound.split();
            let mut out_framed_write = FramedWrite::new(out_write_half, BytesCodec::new());
            let mut out_framed_read: FramedRead<ReadHalf, LengthDelimitedCodec> =
                LengthDelimitedCodec::builder()
                    .length_field_offset(0)
                    .length_field_length(4)
                    .length_adjustment(4)
                    .num_skip(0)
                    // .max_frame_length(self.tcp_frame_max_size as usize)
                    .big_endian()
                    .new_read(out_read_half);

            while let Some(request_data) = in_framed_read.next().await {
                let request_bytes = request_data.unwrap();
                let request_bytes = request_bytes.freeze();
                let request_buf = request_bytes.as_ref();
                println!("<in> recv: {:?}", request_buf);

                let (header_request, rest) = request_header()
                    .parse(&request_buf[mem::size_of::<i32>()..])
                    .expect("Invalid header");
                println!("<in> header request: {:?}", header_request);

                let api_version = header_request.api_version;
                if header_request.api_key == ApiKey::API_VERSIONS {
                    api_versions_request(api_version, rest);

                    out_framed_write.send(request_bytes).await.unwrap();
                    println!("<out>API_VERSIONS send proxy request");

                    if let Some(response_data) = out_framed_read.next().await {
                        let response_bytes = response_data.unwrap();
                        let response_bytes = response_bytes.freeze();
                        let response_buf = response_bytes.as_ref();
                        println!("<out>API_VERSIONS recv: {:?}", response_buf);

                        let (header_response, rest) = response_header()
                            .parse(&response_buf[mem::size_of::<i32>()..])
                            .expect("Invalid header");
                        println!("<out>API_VERSIONS header response: {:?}", header_response);

                        api_versions_response(api_version, rest);

                        in_framed_write.send(response_bytes).await.unwrap();
                        println!("<in>API_VERSIONS send proxy response");
                    }
                } else if header_request.api_key == ApiKey::METADATA {
                    metadata_request(api_version, rest);

                    out_framed_write.send(request_bytes).await.unwrap();
                    println!("<out>METADATA send proxy request");

                    if let Some(response_data) = out_framed_read.next().await {
                        let response_bytes = response_data.unwrap();
                        let response_buf = response_bytes.as_ref();
                        println!("<out>METADATA recv: {:?}", response_buf);

                        let (header_response, rest) = response_header()
                            .parse(&response_buf[mem::size_of::<i32>()..])
                            .expect("Invalid header");
                        println!("<out>METADATA header response: {:?}", header_response);

                        let resp_meta = metadata_response(api_version, rest);

                        let body_len = header_response.encode_len() + resp_meta.len();
                        let mut bytes_mut = BytesMut::with_capacity(4 + body_len);
                        bytes_mut.put_i32(body_len as i32);
                        header_response.encode(&mut bytes_mut);
                        bytes_mut.put_slice(resp_meta.as_slice());

                        in_framed_write.send(bytes_mut.freeze()).await.unwrap();
                        println!("<in>METADATA send proxy response");
                    }
                } else if header_request.api_key == ApiKey::PRODUCE {
                    produce_request(api_version, rest);

                    out_framed_write.send(request_bytes).await.unwrap();
                    println!("<out>Produce send proxy request");

                    if let Some(response_data) = out_framed_read.next().await {
                        let response_bytes = response_data.unwrap();
                        let response_bytes = response_bytes.freeze();
                        let response_buf = response_bytes.as_ref();
                        println!("<out>Produce recv: {:?}", response_buf);

                        let (header_response, rest) = response_header()
                            .parse(&response_buf[mem::size_of::<i32>()..])
                            .expect("Invalid header");
                        println!("<out>Produce header response: {:?}", header_response);

                        produce_response(api_version, rest);

                        in_framed_write.send(response_bytes).await.unwrap();
                        println!("<in>Produce send proxy response");
                    }
                } else {
                    eprintln!("=============");
                    out_framed_write.send(request_bytes).await.unwrap();

                    if let Some(response_data) = out_framed_read.next().await {
                        let response_bytes = response_data.unwrap();
                        let response_bytes = response_bytes.freeze();

                        in_framed_write.send(response_bytes).await.unwrap();
                    }
                }

                eprintln!("/");
            }
        });
    }

    Ok(())
}

fn api_versions_request(api_version: i16, rest: &[u8]) {
    match api_version {
        0 => {
            let (api_versions, _rest) = kafka_protocol::parser::api_versions_v0_request()
                .easy_parse(rest)
                .expect("Invalid api_versions_request");
            println!("<in>API_VERSIONS request body: {:?}", api_versions);
        }
        1 => {
            let (api_versions, _rest) = kafka_protocol::parser::api_versions_v1_request()
                .easy_parse(rest)
                .expect("Invalid api_versions_request");
            println!("<in>API_VERSIONS request body: {:?}", api_versions);
        }
        2 => {
            let (api_versions, _rest) = kafka_protocol::parser::api_versions_v2_request()
                .easy_parse(rest)
                .expect("Invalid api_versions_request");
            println!("<in>API_VERSIONS request body: {:?}", api_versions);
        }
        3 => {
            eprintln!("<in>API_VERSIONS unsupported `api_version` {}", api_version);
        }
        _ => {
            eprintln!("<in>API_VERSIONS unknown `api_version` {}", api_version);
        }
    }
}

fn api_versions_response(api_version: i16, rest: &[u8]) {
    match api_version {
        0 => {
            let (api_versions, _rest) = kafka_protocol::parser::api_versions_v0_response()
                .easy_parse(rest)
                .expect("Invalid api_versions_response");
            println!("<out>API_VERSIONS header response:: {:?}", api_versions);
        }
        1 => {
            let (api_versions, _rest) = kafka_protocol::parser::api_versions_v1_response()
                .easy_parse(rest)
                .expect("Invalid api_versions_response");
            println!("<out>API_VERSIONS header response:: {:?}", api_versions);
        }
        2 => {
            let (api_versions, _rest) = kafka_protocol::parser::api_versions_v2_response()
                .easy_parse(rest)
                .expect("Invalid api_versions_response");
            println!("<out>API_VERSIONS header response:: {:?}", api_versions);
        }
        3 => {
            eprintln!(
                "<out>API_VERSIONS unsupported `api_version` {}",
                api_version
            );
        }
        _ => {
            eprintln!("<out>API_VERSIONS unknown `api_version` {}", api_version);
        }
    }
}

fn metadata_request(api_version: i16, rest: &[u8]) {
    match api_version {
        0 => {
            let (api_versions, _rest) = kafka_protocol::parser::metadata_v0_request()
                .easy_parse(rest)
                .expect("Invalid api_versions_request");
            println!("<in>METADATA request body: {:?}", api_versions);
        }
        1 => {
            let (api_versions, _rest) = kafka_protocol::parser::metadata_v1_request()
                .easy_parse(rest)
                .expect("Invalid api_versions_request");
            println!("<in>METADATA request body: {:?}", api_versions);
        }
        2 => {
            let (api_versions, _rest) = kafka_protocol::parser::metadata_v2_request()
                .easy_parse(rest)
                .expect("Invalid api_versions_request");
            println!("<in>METADATA request body: {:?}", api_versions);
        }
        3 => {
            let (api_versions, _rest) = kafka_protocol::parser::metadata_v3_request()
                .easy_parse(rest)
                .expect("Invalid api_versions_request");
            println!("<in>METADATA request body: {:?}", api_versions);
        }
        4 => {
            let (api_versions, _rest) = kafka_protocol::parser::metadata_v4_request()
                .easy_parse(rest)
                .expect("Invalid api_versions_request");
            println!("<in>METADATA request body: {:?}", api_versions);
        }
        5 => {
            let (api_versions, _rest) = kafka_protocol::parser::metadata_v5_request()
                .easy_parse(rest)
                .expect("Invalid api_versions_request");
            println!("<in>METADATA request body: {:?}", api_versions);
        }
        6 => {
            let (api_versions, _rest) = kafka_protocol::parser::metadata_v6_request()
                .easy_parse(rest)
                .expect("Invalid api_versions_request");
            println!("<in>METADATA request body: {:?}", api_versions);
        }
        7 => {
            let (api_versions, _rest) = kafka_protocol::parser::metadata_v7_request()
                .easy_parse(rest)
                .expect("Invalid api_versions_request");
            println!("<in>METADATA request body: {:?}", api_versions);
        }
        8 => {
            let (api_versions, _rest) = kafka_protocol::parser::metadata_v8_request()
                .easy_parse(rest)
                .expect("Invalid api_versions_request");
            println!("<in>METADATA request body: {:?}", api_versions);
        }
        _ => {
            eprintln!("<in>METADATA unknown `api_version` {}", api_version);
        }
    }
}

fn metadata_response(api_version: i16, rest: &[u8]) -> Vec<u8> {
    match api_version {
        0 => {
            let (mut api_versions, _rest) = kafka_protocol::parser::metadata_v0_response()
                .easy_parse(rest)
                .expect("Invalid api_versions_response");
            println!("<out>METADATA response:: {:?}", api_versions);
            api_versions.brokers.clear();
            api_versions
                .brokers
                .push(kafka_protocol::parser::metadata_v0_response::Brokers {
                    node_id: 1i32,
                    host: "10.181.160.207",
                    port: 9092i32,
                });
            println!("<out>METADATA response replace:: {:?}", api_versions);

            let mut encode_bytes: Vec<u8> = Vec::with_capacity(api_versions.encode_len());
            api_versions.encode(&mut encode_bytes);
            encode_bytes
        }
        1 => {
            let (mut api_versions, _rest) = kafka_protocol::parser::metadata_v1_response()
                .easy_parse(rest)
                .expect("Invalid api_versions_response");
            println!("<out>METADATA response:: {:?}", api_versions);

            api_versions.brokers.clear();
            api_versions
                .brokers
                .push(kafka_protocol::parser::metadata_v1_response::Brokers {
                    node_id: 1i32,
                    host: "10.181.160.207",
                    port: 9092i32,
                    rack: None,
                });
            println!("<out>METADATA response replace:: {:?}", api_versions);

            let mut encode_bytes: Vec<u8> = Vec::with_capacity(api_versions.encode_len());
            api_versions.encode(&mut encode_bytes);
            encode_bytes
        }
        2 => {
            let (mut api_versions, _rest) = kafka_protocol::parser::metadata_v2_response()
                .easy_parse(rest)
                .expect("Invalid api_versions_response");
            println!("<out>METADATA response:: {:?}", api_versions);

            api_versions.brokers.clear();
            api_versions
                .brokers
                .push(kafka_protocol::parser::metadata_v2_response::Brokers {
                    node_id: 1i32,
                    host: "10.181.160.207",
                    port: 9092i32,
                    rack: None,
                });
            println!("<out>METADATA response replace:: {:?}", api_versions);

            let mut encode_bytes: Vec<u8> = Vec::with_capacity(api_versions.encode_len());
            api_versions.encode(&mut encode_bytes);
            encode_bytes
        }
        3 => {
            let (mut api_versions, _rest) = kafka_protocol::parser::metadata_v3_response()
                .easy_parse(rest)
                .expect("Invalid api_versions_response");
            println!("<out>METADATA response:: {:?}", api_versions);

            api_versions.brokers.clear();
            api_versions
                .brokers
                .push(kafka_protocol::parser::metadata_v3_response::Brokers {
                    node_id: 1i32,
                    host: "10.181.160.207",
                    port: 9092i32,
                    rack: None,
                });
            println!("<out>METADATA response replace:: {:?}", api_versions);

            let mut encode_bytes: Vec<u8> = Vec::with_capacity(api_versions.encode_len());
            api_versions.encode(&mut encode_bytes);
            encode_bytes
        }
        4 => {
            let (mut api_versions, _rest) = kafka_protocol::parser::metadata_v4_response()
                .easy_parse(rest)
                .expect("Invalid api_versions_response");
            println!("<out>METADATA response:: {:?}", api_versions);

            api_versions.brokers.clear();
            api_versions
                .brokers
                .push(kafka_protocol::parser::metadata_v4_response::Brokers {
                    node_id: 1i32,
                    host: "10.181.160.207",
                    port: 9092i32,
                    rack: None,
                });
            println!("<out>METADATA response replace:: {:?}", api_versions);

            let mut encode_bytes: Vec<u8> = Vec::with_capacity(api_versions.encode_len());
            api_versions.encode(&mut encode_bytes);
            encode_bytes
        }
        5 => {
            let (mut api_versions, _rest) = kafka_protocol::parser::metadata_v5_response()
                .easy_parse(rest)
                .expect("Invalid api_versions_response");
            println!("<out>METADATA response:: {:?}", api_versions);

            api_versions.brokers.clear();
            api_versions
                .brokers
                .push(kafka_protocol::parser::metadata_v5_response::Brokers {
                    node_id: 1i32,
                    host: "10.181.160.207",
                    port: 9092i32,
                    rack: None,
                });
            println!("<out>METADATA response replace:: {:?}", api_versions);

            let mut encode_bytes: Vec<u8> = Vec::with_capacity(api_versions.encode_len());
            api_versions.encode(&mut encode_bytes);
            encode_bytes
        }
        6 => {
            let (mut api_versions, _rest) = kafka_protocol::parser::metadata_v6_response()
                .easy_parse(rest)
                .expect("Invalid api_versions_response");
            println!("<out>METADATA response:: {:?}", api_versions);

            api_versions.brokers.clear();
            api_versions
                .brokers
                .push(kafka_protocol::parser::metadata_v6_response::Brokers {
                    node_id: 1i32,
                    host: "10.181.160.207",
                    port: 9092i32,
                    rack: None,
                });
            println!("<out>METADATA response replace:: {:?}", api_versions);

            let mut encode_bytes: Vec<u8> = Vec::with_capacity(api_versions.encode_len());
            api_versions.encode(&mut encode_bytes);
            encode_bytes
        }
        7 => {
            let (mut api_versions, _rest) = kafka_protocol::parser::metadata_v7_response()
                .easy_parse(rest)
                .expect("Invalid api_versions_response");
            println!("<out>METADATA response:: {:?}", api_versions);

            api_versions.brokers.clear();
            api_versions
                .brokers
                .push(kafka_protocol::parser::metadata_v7_response::Brokers {
                    node_id: 1i32,
                    host: "10.181.160.207",
                    port: 9092i32,
                    rack: None,
                });
            println!("<out>METADATA response replace:: {:?}", api_versions);

            let mut encode_bytes: Vec<u8> = Vec::with_capacity(api_versions.encode_len());
            api_versions.encode(&mut encode_bytes);
            encode_bytes
        }
        8 => {
            let (mut api_versions, _rest) = kafka_protocol::parser::metadata_v8_response()
                .easy_parse(rest)
                .expect("Invalid api_versions_response");
            println!("<out>METADATA response:: {:?}", api_versions);

            api_versions.brokers.clear();
            api_versions
                .brokers
                .push(kafka_protocol::parser::metadata_v8_response::Brokers {
                    node_id: 1i32,
                    host: "10.181.160.207",
                    port: 9092i32,
                    rack: None,
                });
            println!("<out>METADATA response replace:: {:?}", api_versions);

            let mut encode_bytes: Vec<u8> = Vec::with_capacity(api_versions.encode_len());
            api_versions.encode(&mut encode_bytes);
            encode_bytes
        }
        _ => {
            eprintln!("<out>METADATA unknown `api_version` {}", api_version);
            rest.to_vec()
        }
    }
}

fn produce_request(api_version: i16, rest: &[u8]) {
    match api_version {
        0 => {
            let (api_versions, _rest) = kafka_protocol::parser::produce_v0_request::<
                Option<RecordBatch<Vec<Record<'_>>>>,
                _,
            >()
            .easy_parse(rest)
            .expect("Invalid produce_request");
            println!("<in>Produce request body: {:?}", api_versions);
        }
        1 => {
            let (api_versions, _rest) = kafka_protocol::parser::produce_v1_request::<
                Option<RecordBatch<Vec<Record<'_>>>>,
                _,
            >()
            .easy_parse(rest)
            .expect("Invalid produce_request");
            println!("<in>Produce request body: {:?}", api_versions);
        }
        2 => {
            let (api_versions, _rest) = kafka_protocol::parser::produce_v2_request::<
                Option<RecordBatch<Vec<Record<'_>>>>,
                _,
            >()
            .easy_parse(rest)
            .expect("Invalid produce_request");
            println!("<in>Produce request body: {:?}", api_versions);
        }
        3 => {
            let (api_versions, _rest) = kafka_protocol::parser::produce_v3_request::<
                Option<RecordBatch<Vec<Record<'_>>>>,
                _,
            >()
            .easy_parse(rest)
            .expect("Invalid produce_request");
            println!("<in>Produce request body: {:?}", api_versions);
        }
        4 => {
            let (api_versions, _rest) = kafka_protocol::parser::produce_v4_request::<
                Option<RecordBatch<Vec<Record<'_>>>>,
                _,
            >()
            .easy_parse(rest)
            .expect("Invalid produce_request");
            println!("<in>Produce request body: {:?}", api_versions);
        }
        5 => {
            let (api_versions, _rest) = kafka_protocol::parser::produce_v5_request::<
                Option<RecordBatch<Vec<Record<'_>>>>,
                _,
            >()
            .easy_parse(rest)
            .expect("Invalid produce_request");
            println!("<in>Produce request body: {:?}", api_versions);
        }
        6 => {
            let (api_versions, _rest) = kafka_protocol::parser::produce_v6_request::<
                Option<RecordBatch<Vec<Record<'_>>>>,
                _,
            >()
            .easy_parse(rest)
            .expect("Invalid produce_request");
            println!("<in>Produce request body: {:?}", api_versions);
        }
        7 => {
            let (api_versions, _rest) = kafka_protocol::parser::produce_v7_request::<
                Option<RecordBatch<Vec<Record<'_>>>>,
                _,
            >()
            .easy_parse(rest)
            .expect("Invalid produce_request");
            println!("<in>Produce request body: {:?}", api_versions);
        }
        _ => {
            eprintln!("<in>Produce unknown `api_version` {}", api_version);
        }
    }
}

fn produce_response(api_version: i16, rest: &[u8]) {
    match api_version {
        0 => {
            let (api_versions, _rest) = kafka_protocol::parser::produce_v0_response()
                .easy_parse(rest)
                .expect("Invalid produce_response");
            println!("<out>Produce header response:: {:?}", api_versions);
        }
        1 => {
            let (api_versions, _rest) = kafka_protocol::parser::produce_v1_response()
                .easy_parse(rest)
                .expect("Invalid produce_response");
            println!("<out>Produce header response:: {:?}", api_versions);
        }
        2 => {
            let (api_versions, _rest) = kafka_protocol::parser::produce_v2_response()
                .easy_parse(rest)
                .expect("Invalid produce_response");
            println!("<out>Produce header response:: {:?}", api_versions);
        }
        3 => {
            let (api_versions, _rest) = kafka_protocol::parser::produce_v3_response()
                .easy_parse(rest)
                .expect("Invalid produce_response");
            println!("<out>Produce header response:: {:?}", api_versions);
        }
        4 => {
            let (api_versions, _rest) = kafka_protocol::parser::produce_v4_response()
                .easy_parse(rest)
                .expect("Invalid produce_response");
            println!("<out>Produce header response:: {:?}", api_versions);
        }
        5 => {
            let (api_versions, _rest) = kafka_protocol::parser::produce_v5_response()
                .easy_parse(rest)
                .expect("Invalid produce_response");
            println!("<out>Produce header response:: {:?}", api_versions);
        }
        6 => {
            let (api_versions, _rest) = kafka_protocol::parser::produce_v6_response()
                .easy_parse(rest)
                .expect("Invalid produce_response");
            println!("<out>Produce header response:: {:?}", api_versions);
        }
        7 => {
            let (api_versions, _rest) = kafka_protocol::parser::produce_v7_response()
                .easy_parse(rest)
                .expect("Invalid produce_response");
            println!("<out>Produce header response:: {:?}", api_versions);
        }
        _ => {
            eprintln!("<out>Produce unknown `api_version` {}", api_version);
        }
    }
}
