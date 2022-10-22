use anyhow::Result;
use bytes::Bytes;
use futures::prelude::*;
use kvserver::{CommandRequest, MemTable, Service, tls::TlsServerAcceptor};
use prost::Message;
use tokio::net::TcpListener;
use tokio_util::codec::{LengthDelimitedCodec, Framed};
use tracing::info;

/**
 * 默认日志等级是 WARN
 * 设置环境变量 RUST_LOG="info" 来展示 info 日志
 */
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let server_cert = include_str!("../fixtures/server.cert");
    let server_key = include_str!("../fixtures/server.key");
    let client_ca_cert = include_str!("../fixtures/ca.cert");

    let acceptor = TlsServerAcceptor::new(server_cert, server_key, Some(client_ca_cert))?;

    let service: Service = Service::new(MemTable::new());
    let addr = "127.0.0.1:9527";
    let listener = TcpListener::bind(addr).await?;
    info!("Start listening on {}", addr);
    loop {
        let tls = acceptor.clone();
        let (stream, addr) = listener.accept().await?;
        info!("Client {:?} connected", addr);
        let svc = service.clone();

        let stream = tls.accept(stream).await?;

        let mut stream = Framed::new(stream, LengthDelimitedCodec::new());        

        tokio::spawn(async move {
            while let Some(Ok(data)) = stream.next().await {
                let req = CommandRequest::decode(data).unwrap();
                let res = svc.execute(req);
                let buf = res.encode_to_vec();
                stream.send(Bytes::from(buf)).await.unwrap();
            }
            info!("Client {:?} disconnected", addr);
        });
    }
}