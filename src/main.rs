use futures::StreamExt;
use libp2p::ping::{Ping, PingConfig};
use libp2p::swarm::SwarmEvent;
use libp2p::swarm::{dial_opts::DialOpts, Swarm};
use libp2p::{identity, Multiaddr, PeerId};
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    /*
    /////////////Peer Identity/////////////////////
    local_key--> Public key of peer generated from ED25519
    loca_peer_id --> Private key created from public key
    */
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {:?}", local_peer_id);

    /*
    /////////Transport using TCP and noise for encryption//////////
    defines how to send bytes on the network
    using development_transport function

    */

    let transport = libp2p::development_transport(local_key).await?;

    /*
    ///////Behaviour/////////
    Define what bytes to send
    ping protocol sends 32 bytes of random data.
    */
    let behaviour = Ping::new(PingConfig::new().with_keep_alive(true));

    /*
    ///////////Swarm//////////
    Connect Transport and Network behaviour
    pass commands from NetworkBehaviour and Transport and Events from Transport to NetworkBehaviour
    */

    let mut swarm = Swarm::new(transport, behaviour, local_peer_id);

    /*

    ///////Listening to all interface at defined port/////
    Using swarm.listen

    */
    swarm.listen_on("/ip4/0.0.0.0/tcp/16384".parse()?)?;

    // Check second line argument in the command

    if let Some(addr) = std::env::args().nth(1) {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
        println!("Dialed {}", addr)
    }
    // Drive swarm in a loop- listen to incoming connections
    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {:?}", address),
            SwarmEvent::Behaviour(event) => println!("{:?}", event),
            _ => {}
        }
    }

    Ok(())
}
