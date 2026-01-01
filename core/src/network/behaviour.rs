use libp2p::{
    dcutr, gossipsub, identify,
    kad::{self, store::MemoryStore},
    ping, 
    relay::client as relay_client,
    swarm::NetworkBehaviour, 
    PeerId, identity::Keypair,
};

#[derive(NetworkBehaviour)]
pub struct AeonBehaviour {
    pub kademlia: kad::Behaviour<MemoryStore>,
    pub gossipsub: gossipsub::Behaviour,
    pub identify: identify::Behaviour,
    pub ping: ping::Behaviour,
    pub relay_client: relay_client::Behaviour,
    pub dcutr: dcutr::Behaviour,
}

impl AeonBehaviour {
    pub fn new(local_key: Keypair, relay_client: relay_client::Behaviour) -> Self {
        let local_peer_id = PeerId::from(local_key.public());
        let store = MemoryStore::new(local_peer_id);
        
        let gossipsub = gossipsub::Behaviour::new(
            gossipsub::MessageAuthenticity::Signed(local_key.clone()),
            gossipsub::Config::default(),
        ).expect("Gossipsub init failed");

        Self {
            kademlia: kad::Behaviour::new(local_peer_id, store),
            gossipsub,
            identify: identify::Behaviour::new(identify::Config::new(
                "/aeon/1.0.0".into(),
                local_key.public(),
            )),
            ping: ping::Behaviour::default(),
            relay_client,
            dcutr: dcutr::Behaviour::new(local_peer_id),
        }
    }
}
