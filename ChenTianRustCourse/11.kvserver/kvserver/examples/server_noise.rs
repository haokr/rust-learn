use anyhow::Result;
use bytes::Bytes;
use futures::prelude::*;
use kvserver::{CommandRequest, MemTable, Service, noise::NoiseServerAcceptor};
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
    let mut buf = vec![0u8; 65535];

    let service: Service = Service::new(MemTable::new());
    let addr = "127.0.0.1:9527";

    let listener = TcpListener::bind(addr).await?;

    loop {
        let svc = service.clone();

        let (mut stream, _) = listener.accept().await?;

        let noise = NoiseServerAcceptor::accept(&mut stream).await?;

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