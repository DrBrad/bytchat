use rlibdht::kad::kademlia_base::KademliaBase;
use rlibdht::kademlia::Kademlia;

pub fn run_p2p() {
    let kad = Kademlia::default();

    #[cfg(profile = "debug")]
    {
        kad.get_routing_table().lock().unwrap().set_secure_only(false);
        kad.get_server().lock().unwrap().set_allow_bogon(true);
    }

    kad.bind(7890);
    //kad.join(8080, SocketAddr::new(IpAddr::from([127, 0, 0, 1]), 7890)).unwrap();
}
