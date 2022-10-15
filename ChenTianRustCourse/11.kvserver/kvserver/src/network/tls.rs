use std::{sync::Arc, io::Cursor};

use tokio::io::{AsyncRead, AsyncWrite};
use tokio_rustls::{rustls::{ServerConfig, ClientConfig, NoClientAuth, RootCertStore, AllowAnyAuthenticatedClient, Certificate, internal::pemfile, PrivateKey, OwnedTrustAnchor}, webpki::{DNSNameRef, TLSServerTrustAnchors, trust_anchor_util}, TlsConnector};
use tokio_rustls::{ client::TlsStream as ClientTlsStream, server::TlsStream as ServerTlsStream, TlsAcceptor,};
use crate::KvError;


/// kv server  自己的 ALPN （Application Layer Protocal Negotiation
const ALPN_KV: &str = "kv";

/// 存放 TLS ServerConfig 并提供方法 accept 把底层的协议转换成 TLS
#[derive(Clone)]
pub struct TlsServerAcceptor {
    inner: Arc<ServerConfig>,
}

/// 存放 TLS Client 并提供方法 connect 把底层的协议转换成 TLS
#[derive(Clone)]
pub struct TlsClientConnector {
    pub config: Arc<ClientConfig>,
    pub domain: Arc<String>,
}

impl TlsClientConnector {

    pub fn new(
        domain: impl Into<String>,
        identity: Option<(&str, &str)>,
        server_ca: Option<&str>,
    ) -> Result<Self, KvError> {

        let mut config = ClientConfig::new();

        // 如果有客户端证书，加载
        if let Some((cert, key)) = identity {
            let certs = load_certs(cert)?;
            let key = load_key(key)?;
            match config.set_single_client_cert(certs, key) {
                Ok(it) => it,
                Err(err) => return Err(KvError::CertifcateParseError("server".into(), "CA".into())),
            };
        }

        // 加载本地信任的根证书链
        let mut root_store = RootCertStore::empty();
        webpki_roots::TLS_SERVER_ROOTS
            .0
            .iter()
            .map(|ta| {
                let trust_anchor = tokio_rustls::webpki::TrustAnchor {
                    subject: ta.subject,
                    spki: ta.spki,
                    name_constraints: ta.name_constraints
                };
                OwnedTrustAnchor::from_trust_anchor(&trust_anchor)
            })
            .for_each(|e| {
                root_store.roots.push(e)
            });
        config.root_store = root_store;

        // 如果有签署服务器的 CA 证书，则加载，这样服务器证书不在根证书链
        // 但是这个 CA 证书能验证它，也可以
        if let Some(cert) = server_ca {
            let mut buf = Cursor::new(cert);
            config.root_store.add_pem_file(&mut buf).unwrap();
        }

        Ok(Self {
            config: Arc::new(config),
            domain: Arc::new(domain.into()),
        })
    }

    /// 触发 TLS 协议，把底层的 stream 转换成 TLS stream
    pub async fn connect<S>(&self, stream: S) -> Result<ClientTlsStream<S>, KvError> 
    where
        S: AsyncRead + AsyncWrite + Unpin + Send,
    {
        let dns = DNSNameRef::try_from_ascii_str(self.domain.as_str())
            .map_err(|_| KvError::Internal("Invalid DNS name".into()))?;

        let stream = TlsConnector::from(self.config.clone())
            .connect(dns, stream)
            .await?;

        Ok(stream)
    }
}

impl TlsServerAcceptor {

    /// 加载 server cert / CA cert，生成 ServerConfig
    pub fn new(cert: &str, key: &str, client_ca: Option<&str>) -> Result<Self, KvError> {
        let certs = load_certs(cert)?;
        let key = load_key(key)?;

        let mut config = match client_ca {
            None => ServerConfig::new(NoClientAuth::new()),
            Some(cert) => {
                // 如果客户端证书是某个 CA 证书签发的，则吧这个 CA 证书加载到信任链中
                let mut cert = Cursor::new(cert);
                let mut client_root_cert_store = RootCertStore::empty();
                client_root_cert_store.add_pem_file(&mut cert)
                    .map_err(|_| KvError::CertifcateParseError("CA".into(), "cert".into()))?;

                let client_auth = AllowAnyAuthenticatedClient::new(client_root_cert_store);
                ServerConfig::new(client_auth)
            }
        };

        // 加载服务器证书
        config.set_single_cert(certs, key)
            .map_err(|_| KvError::CertifcateParseError("server".into(), "cert".into()))?;
        config.set_protocols(&[Vec::from(&ALPN_KV[..])]);

        Ok(Self {
            inner: Arc::new(config),
        })
    } 

    // 触发 TLS 协议，把底层的 stream 转换成 TLS stream
    pub async fn accept<S>(&self, stream: S) -> Result<ServerTlsStream<S>, KvError> 
    where  
        S: AsyncRead + AsyncWrite + Unpin + Send,
    {
        let acceptor = TlsAcceptor::from(self.inner.clone());
        Ok(acceptor.accept(stream).await?)
    }
}

fn load_certs(cert: &str) -> Result<Vec<Certificate>, KvError> {
    let mut cert = Cursor::new(cert);
    pemfile::certs(&mut cert).map_err(|_| KvError::CertifcateParseError("server".into(), "cert".into()))
}

fn load_key(key: &str) -> Result<PrivateKey, KvError> {
    let mut cursor = Cursor::new(key);

    // 先尝试用 PKCS8 加载私钥
    if let Ok(mut keys) = pemfile::pkcs8_private_keys(&mut cursor) {
        if !keys.is_empty() {
            return Ok(keys.remove(0));
        }
    }

    // 再尝试加载 RSA key
    cursor.set_position(0);
    if let Ok(mut keys) = pemfile::rsa_private_keys(&mut cursor) {
        if !keys.is_empty() {
            return Ok(keys.remove(0));
        }
    }

    // 不支持的私钥类型
    Err(KvError::CertifcateParseError("private".into(), "key".into()))
}