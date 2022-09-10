use std::{borrow::Cow};
use anyhow::Result;
use futures::StreamExt;
use libp2p::{
    floodsub::{Floodsub, FloodsubEvent, self, Topic}, 
    mdns::{Mdns, MdnsEvent}, 
    NetworkBehaviour, 
    PeerId, 
    swarm::{NetworkBehaviourEventProcess, SwarmBuilder, SwarmEvent}, 
    identity, noise, core::transport::upgrade, yamux, Swarm, tcp::TokioTcpConfig, Transport};
use tokio::io::{stdin, BufReader, AsyncBufReadExt};


/// 处理 p2p 网络的 behavior 数据结构
/// 里面的每个域都需要实现 NetworkBehaviour，或者使用 #[behaviour(ignore)]
#[derive(NetworkBehaviour)]
#[behaviour(event_process = true)]
struct ChatBehavior {
    /// flood subscription，比较浪费带宽，gossipsub 是更好的选择
    floodsub: Floodsub,
    /// 本地节点发现机制
    mdns: Mdns,
    // 在 behavior 结构中，可以放其他数据，但是需要 ignore
    // #[behaviour(ignore)]
    // _useless: String,
}

impl ChatBehavior {
    /// 创建新的 CharBehavior
    pub async fn new(id: PeerId) -> Result<Self> {
        Ok(Self {
            mdns: Mdns::new(Default::default()).await?,
            floodsub: Floodsub::new(id),
        })
    }
}

impl NetworkBehaviourEventProcess<FloodsubEvent> for ChatBehavior {
    // 处理 floodsub 产生的消息
    fn inject_event(&mut self, event: FloodsubEvent) {
        if let FloodsubEvent::Message(msg) = event {
            let text = String::from_utf8_lossy(&msg.data);
            println!("{:?}: {:?}", msg.source, text);
        }
    }
}

impl NetworkBehaviourEventProcess<MdnsEvent> for ChatBehavior {
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(list) => {
                // 吧 mdns 新发现的 peer 加入到 floodsub 的 view 中
                for (id, addr) in list {
                    println!("Got peer: {} with addr {}", &id, &addr);
                    self.floodsub.add_node_to_partial_view(id);
                }
            }
            MdnsEvent::Expired(list) => {
                // 把 mdns 发现的离开的 peer 从 floodsub 的 view 中删除
                for (id, addr) in list {
                    println!("Removed peer: {} with addr {}", &id, &addr);
                    self.floodsub.remove_node_from_partial_view(&id);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // 如果带参数，当成一个 topic
    let name = match std::env::args().nth(1) {
        Some(arg) => Cow::Owned(arg),
        None => Cow::Borrowed("lobby"),
    };

    // 创建 floodsub topic
    let topic = floodsub::Topic::new(name);

    // 创建 swarm
    let mut swarm = create_swarm(topic.clone()).await?;

    swarm.listen_on("/ip4/127.0.0.1/tcp/0".parse()?)?;

    // 读 stdin 一行
    let mut stdin = BufReader::new(stdin()).lines();

    // main loop
    loop {
        tokio::select! {
            line = stdin.next_line() => {
                let line = line?.expect("stdin closed");
                swarm.behaviour_mut().floodsub.publish(topic.clone(), line.as_bytes());
            }
            event = swarm.select_next_some() => {
                if let SwarmEvent::NewListenAddr { address, .. } = event {
                    println!("Listening on {:?}", address);
                }
            }
        }
    }
}

async fn create_swarm(topic: Topic) -> Result<Swarm<ChatBehavior>> {
    // 创建密钥对
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());
    println!("Local peer id: {:?}", peer_id);

    // 使用 noise protocal 处理加密认证
    let noise_keys = noise::Keypair::<noise::X25519Spec>::new().into_authentic(&id_keys)?;
    
    // 创建传输层
    let transport = TokioTcpConfig::new()
            .nodelay(true)
            .upgrade(upgrade::Version::V1)
            .authenticate(noise::NoiseConfig::xx(noise_keys).into_authenticated())
            .multiplex(yamux::YamuxConfig::default())
            .boxed();

    // 创建 chat behavior
    let mut behavior = ChatBehavior::new(peer_id.clone()).await?;

    // 订阅 topic
    behavior.floodsub.subscribe(topic.clone());

    // 创建 swarm
    let swarm = SwarmBuilder::new(transport, behavior, peer_id)
            .executor(Box::new(|fut| {
                tokio::spawn(fut);
            }))
            .build();

    Ok(swarm)
}