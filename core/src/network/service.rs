use std::{error::Error, time::Duration};
use libp2p::{
    noise, tcp, yamux, relay,
    identity, PeerId, Transport, Multiaddr,
    futures::StreamExt,
    futures::future::Either,
    core::muxing::StreamMuxerBox,
    swarm::SwarmEvent,
};
use tokio::sync::{mpsc, oneshot};

use crate::network::behaviour::AeonBehaviour;
use crate::router::pid::RouteWeightController;

#[derive(Debug)]
pub enum NetworkCommand {
    GetPeerCount(oneshot::Sender<u32>),
    SendMessage { peer_id: String, msg: Vec<u8> },
}

pub struct NetworkService {
    swarm: libp2p::Swarm<AeonBehaviour>,
    #[allow(dead_code)]
    controller: RouteWeightController,
    command_rx: mpsc::Receiver<NetworkCommand>,
}

impl NetworkService {
    // FIX IMPORTANT: Added '+ Send + Sync' to the return error type.
    // This tells Tokio that it is safe to move this error between threads.
    pub async fn new(seed: u64) -> Result<(Self, mpsc::Sender<NetworkCommand>), Box<dyn Error + Send + Sync + 'static>> {
        let id_keys = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(id_keys.public());
        println!("Local Peer ID: {}", local_peer_id);

        let noise_config = noise::Config::new(&id_keys).expect("Noise failed");
        let yamux_config = yamux::Config::default();

        let (relay_transport, relay_behaviour) = relay::client::new(local_peer_id);
        
        // Relay Transport Upgrade
        let relay_transport = relay_transport
            .upgrade(libp2p::core::upgrade::Version::V1)
            .authenticate(noise_config.clone())
            .multiplex(yamux_config.clone());

        // TCP Transport Upgrade
        let tcp_transport = tcp::tokio::Transport::new(tcp::Config::default().nodelay(true))
            .upgrade(libp2p::core::upgrade::Version::V1)
            .authenticate(noise_config)
            .multiplex(yamux_config);

        // Combined Transport
        let transport = relay_transport
            .or_transport(tcp_transport)
            .map(|either, _| match either {
                Either::Left((peer, muxer)) => (peer, StreamMuxerBox::new(muxer)),
                Either::Right((peer, muxer)) => (peer, StreamMuxerBox::new(muxer)),
            })
            .boxed();

        let behaviour = AeonBehaviour::new(id_keys.clone(), relay_behaviour);
        
        let swarm = libp2p::SwarmBuilder::with_existing_identity(id_keys)
            .with_tokio()
            .with_other_transport(|_key| transport)? // The error here is now compatible with Send + Sync
            .with_behaviour(|_key| behaviour)?
            .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(60)))
            .build();

        let controller = RouteWeightController::new(0.5, 0.1, 0.1, seed);
        
        let (cmd_tx, cmd_rx) = mpsc::channel(32);

        Ok((Self { swarm, controller, command_rx: cmd_rx }, cmd_tx))
    }

    pub async fn run(&mut self) {
        loop {
            tokio::select! {
                event = self.swarm.select_next_some() => {
                    match event {
                        SwarmEvent::NewListenAddr { address, .. } => {
                            println!("Aeon Node Listening on: {:?}", address);
                        }
                        _ => {}
                    }
                }
                Some(cmd) = self.command_rx.recv() => {
                    match cmd {
                        NetworkCommand::GetPeerCount(reply) => {
                            let count = self.swarm.network_info().num_peers();
                            let _ = reply.send(count as u32);
                        }
                        NetworkCommand::SendMessage { peer_id, msg } => {
                            println!("CMD: Send '{}' to {}", String::from_utf8_lossy(&msg), peer_id);
                        }
                    }
                }
            }
        }
    }
    
    pub fn listen(&mut self, addr: Multiaddr) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        self.swarm.listen_on(addr)?;
        Ok(())
    }
}
