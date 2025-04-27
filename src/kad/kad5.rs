use openssl::pkey::Private;
use openssl::rsa::Rsa;
use rlibdht::kad::kademlia_base::KademliaBase;
use rlibdht::kademlia::Kademlia;
use rlibdht::messages::inter::message_base::MessageBase;
use rlibdht::rpc::events::inter::event::Event;
use rlibdht::rpc::events::inter::message_event::MessageEvent;
use crate::kad::messages::bind_tracker_request::BindTrackerRequest;
use crate::kad::messages::bind_tracker_response::BindTrackerResponse;
use crate::kad::messages::challenge_request::ChallengeRequest;
use crate::kad::messages::challenge_response::ChallengeResponse;

pub struct Kad5 {
    rsa: Rsa<Private>,
    kad: Kademlia
    //tracker: HashMap<TUID, Vec<Node>>
}

impl Kad5 {

    pub fn new() -> Self {
        let rsa = Rsa::generate(4096).unwrap();

        let mut kad = Kademlia::default();
        kad.get_routing_table().lock().unwrap().set_secure_only(false);
        kad.get_server().lock().unwrap().set_allow_bogon(true);

        kad.get_server().lock().unwrap().register_message(|| Box::new(BindTrackerRequest::default()));
        kad.get_server().lock().unwrap().register_message(|| Box::new(BindTrackerResponse::default()));
        kad.get_server().lock().unwrap().register_message(|| Box::new(ChallengeRequest::default()));
        kad.get_server().lock().unwrap().register_message(|| Box::new(ChallengeResponse::default()));

        kad.get_server().lock().unwrap().register_request_listener("bind_tracker", move |event| {
            if event.is_prevent_default() {
                return;
            }

            let mut response = BindTrackerResponse::new(*event.get_message().get_transaction_id());
            response.set_destination(event.get_message().get_origin().unwrap());
            response.set_public(event.get_message().get_origin().unwrap());
            event.set_response(Box::new(response));

            //NOW CHALLENGE VIA PORT GIVEN
            //IF CHALLENGE IS SUCCESSFUL THEN HASH KEY AND ADD TO HASHMAP
        });

        Self {
            rsa,
            kad
        }
    }

    pub fn get_rsa(&self) -> &Rsa<Private> {
        &self.rsa
    }

    pub fn get_kademlia(&self) -> &Kademlia {
        &self.kad
    }
}
