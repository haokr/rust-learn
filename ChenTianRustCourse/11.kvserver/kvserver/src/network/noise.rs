use anyhow::Result;
use snow::{Builder, params::NoiseParams, TransportState};
use tokio::{net::{TcpStream}, io::{AsyncReadExt, AsyncWriteExt, AsyncRead}};
use lazy_static::lazy_static;

static SECRET: &[u8] = b"askldfhjkashdfjkhaskdjfh823432jkhad";
lazy_static! {
    static ref PARAMS: NoiseParams = "Noise_XXpsk3_25519_ChaChaPoly_BLAKE2s".parse().unwrap();
}

pub struct NoiseServerAcceptor {}

pub struct NoiseClientConnector {}

impl NoiseServerAcceptor {

    pub async fn accept<>(stream: &mut TcpStream) -> Result<TransportState> {
        let mut buf = vec![0u8; 65535];

        let builder = Builder::new(PARAMS.clone());
        let static_key = builder.generate_keypair().unwrap().private;
        let mut noise = builder.local_private_key(&static_key).psk(3, SECRET).build_responder().unwrap();

        // <- e
        noise.read_message(&recv(stream).await.unwrap(), &mut buf).unwrap();

        // -> e, ee, s, es
        let len = noise.write_message(&[0u8; 0], &mut buf).unwrap();
        send(stream, &buf[..len]).await;

        // <- s, se
        noise.read_message(&recv(stream).await.unwrap(), &mut buf).unwrap();

        let noise = noise.into_transport_mode().unwrap();

        Ok(noise)
    }
}

impl NoiseClientConnector {

    pub async fn connect(stream: &mut TcpStream) -> Result<TransportState> {
        let mut buf = vec![0u8; 65535];

        let builder = Builder::new(PARAMS.clone());
        let static_key = builder.generate_keypair().unwrap().private;
        let mut noise = builder.local_private_key(&static_key).psk(3, SECRET).build_initiator().unwrap();

        // -> e
        let len = noise.write_message(&[], &mut buf).unwrap();
        send(stream, &buf[..len]).await;

        // <- e, ee, s, es
        noise.read_message(&recv(stream).await.unwrap(), &mut buf).unwrap();

        // -> s, se
        let len = noise.write_message(&[], &mut buf).unwrap();
        send(stream, &buf[..len]).await;

        let noise = noise.into_transport_mode().unwrap();

        Ok(noise)
    }
}

async fn recv(stream: &mut TcpStream) -> Result<Vec<u8>> {
    let mut msg_len_buf = [0u8; 2];
    stream.read_exact(&mut msg_len_buf).await?;
    let msg_len = ((msg_len_buf[0] as usize) << 8) + (msg_len_buf[1] as usize);
    let mut msg = vec![0u8; msg_len];
    stream.read_exact(&mut msg[..]).await?;
    Ok(msg)
}

async fn send(stream: &mut TcpStream, buf: &[u8]) {
    let msg_len_buf = [(buf.len() >> 8) as u8, (buf.len() & 0xff) as u8];
    stream.write_all(&msg_len_buf).await.unwrap();
    stream.write_all(buf).await.unwrap();
}