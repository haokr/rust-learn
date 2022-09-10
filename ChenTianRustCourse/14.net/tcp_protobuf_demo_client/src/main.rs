use anyhow::Result;
use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

#[tokio::main]
async fn main() -> Result<()>{
    // 连接服务器
    let stream = TcpStream::connect("127.0.0.1:9527").await?;
    // 创建消息帧
    let mut stream = Framed::new(stream, LengthDelimitedCodec::new());
    // 发送
    stream.send(Bytes::from("hello world!")).await?;

    if let Some(Ok(data)) = stream.next().await {
        println!("Got: {:?}", String::from_utf8_lossy(&data));
    }
    Ok(())
}
