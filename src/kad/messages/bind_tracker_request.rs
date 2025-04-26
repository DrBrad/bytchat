use std::any::Any;
use std::net::SocketAddr;
use openssl::pkey::Public;
use openssl::rsa::Rsa;
use rlibdht::kad::server::TID_LENGTH;
use rlibdht::messages::inter::message_base::{MessageBase, TID_KEY};
use rlibdht::messages::inter::message_exception::MessageException;
use rlibdht::messages::inter::message_type::{MessageType, TYPE_KEY};
use rlibdht::messages::inter::method_message_base::MethodMessageBase;
use rlibdht::rlibbencode::variables::bencode_object::{BencodeObject, PutObject};
use rlibdht::utils::uid::{ID_LENGTH, UID};

#[derive(Clone)]
pub struct BindTrackerRequest {
    uid: Option<UID>,
    tid: [u8; TID_LENGTH],
    public: Option<SocketAddr>,
    destination: Option<SocketAddr>,
    origin: Option<SocketAddr>,
    key: Option<Rsa<Public>>,
    port: Option<u16>
}

impl BindTrackerRequest {

    pub fn new(tid: [u8; TID_LENGTH]) -> Self {
        Self {
            tid,
            ..Default::default()
        }
    }

    pub fn set_key(&mut self, key: Rsa<Public>) {
        self.key = Some(key);
    }

    pub fn get_key(&self) -> Option<&Rsa<Public>> {
        self.key.as_ref()
    }

    pub fn set_port(&mut self, port: u16) {
        self.port = Some(port);
    }

    pub fn get_port(&self) -> Option<u16> {
        self.port
    }
}

impl Default for BindTrackerRequest {

    fn default() -> Self {
        Self {
            uid: None,
            tid: [0u8; TID_LENGTH],
            public: None,
            destination: None,
            origin: None,
            key: None,
            port: None
        }
    }
}

impl MessageBase for BindTrackerRequest {

    fn set_uid(&mut self, uid: UID) {
        self.uid = Some(uid);
    }

    fn get_uid(&self) -> Option<UID> {
        self.uid
    }

    fn set_transaction_id(&mut self, tid: [u8; TID_LENGTH]) {
        self.tid = tid;
    }

    fn get_transaction_id(&self) -> &[u8; TID_LENGTH] {
        &self.tid
    }

    fn set_public(&mut self, public: SocketAddr) {
        self.public = Some(public);
    }

    fn get_public(&self) -> Option<SocketAddr> {
        self.public
    }

    fn set_destination(&mut self, destination: SocketAddr) {
        self.destination = Some(destination);
    }

    fn get_destination(&self) -> Option<SocketAddr> {
        self.destination
    }

    fn set_origin(&mut self, origin: SocketAddr) {
        self.origin = Some(origin);
    }

    fn get_origin(&self) -> Option<SocketAddr> {
        self.origin
    }

    fn get_type(&self) -> MessageType {
        MessageType::ReqMsg
    }

    fn encode(&self) -> BencodeObject {
        let mut ben = BencodeObject::new();

        ben.put(TID_KEY, self.tid.clone());
        ben.put("v", "1.0");
        ben.put(TYPE_KEY, self.get_type().rpc_type_name());

        ben.put(self.get_type().rpc_type_name(), self.get_method());
        ben.put(self.get_type().inner_key(), BencodeObject::new());
        ben.get_object_mut(self.get_type().inner_key()).unwrap().put("id", self.uid.unwrap().bytes().clone());

        if let Some(key) = &self.key {
            ben.get_object_mut(self.get_type().inner_key()).unwrap().put("k", key.public_key_to_der().unwrap());
        }

        if let Some(port) = self.port {
            ben.get_object_mut(self.get_type().inner_key()).unwrap().put("p", port);
        }

        ben
    }

    fn decode(&mut self, ben: &BencodeObject) -> Result<(), MessageException> {
        if !ben.contains_key(self.get_type().inner_key()) {
            return Err(MessageException::new("Protocol Error, such as a malformed packet.", 203));
        }

        match ben.get_object(self.get_type().inner_key()).unwrap().get_bytes("id") {
            Ok(id) => {
                let mut bid = [0u8; ID_LENGTH];
                bid.copy_from_slice(&id[..ID_LENGTH]);
                self.uid = Some(UID::from(bid));
            }
            _ => return Err(MessageException::new("Protocol Error, such as a malformed packet.", 203))
        }

        match ben.get_object(self.get_type().inner_key()).unwrap().get_bytes("k") {
            Ok(key) => self.key = Some(Rsa::public_key_from_der(key).unwrap()),
            _ => return Err(MessageException::new("Protocol Error, such as a malformed packet.", 203))
        }

        match ben.get_object(self.get_type().inner_key()).unwrap().get_number::<u16>("p") {
            Ok(port) => self.port = Some(port),
            _ => return Err(MessageException::new("Protocol Error, such as a malformed packet.", 203))
        }

        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl MethodMessageBase for BindTrackerRequest {

    fn get_method(&self) -> &str {
        "bind_tracker"
    }

    fn upcast(&self) -> &dyn MessageBase {
        self
    }

    fn upcast_mut(&mut self) -> &mut dyn MessageBase {
        self
    }

    fn dyn_clone(&self) -> Box<dyn MethodMessageBase> {
        Box::new(self.clone())
    }
}
