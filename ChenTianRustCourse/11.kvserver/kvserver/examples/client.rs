
use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::prelude::*;
use kvserver::{CommandRequest, CommandResponse, tls::TlsClientConnector};
use tokio::net::TcpStream;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:9527";

    // ca 证书
    let ca_cert = include_str!("../fixtures/ca.cert");

    let client_cert = include_str!("../fixtures/client.cert");
    let client_key = include_str!("../fixtures/client.key");

    let connector = TlsClientConnector::new("kvserver.acme.inc", Some((client_cert, client_key)), Some(ca_cert))?;
    
    // 连接服务器
    let stream = TcpStream::connect(addr).await?;
    // 建立 tls 连接
    let stream = connector.connect(stream).await?;
    // 使用 AsyncProstStream 来处理 TCP
    let mut client = AsyncProstStream::<_, CommandResponse, CommandRequest, _>::from(stream).for_async();

    // 生成一个 HSET 命令
    //let cmd = CommandRequest::new_hset("table1", "hao", "wang".into());
    //let cmd = CommandRequest::new_hget("table1", "hello");
    let cmd = CommandRequest::new_hgetall("table1");


    // 发送 HSET 命令
    client.send(cmd).await?;

    if let Some(Ok(data)) = client.next().await {
        info!("Got response {:?}", data);
    }

    Ok(())
}