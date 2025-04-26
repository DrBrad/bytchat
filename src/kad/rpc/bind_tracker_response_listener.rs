use std::sync::{Arc, Mutex};
use rlibdht::routing::inter::routing_table::RoutingTable;
use rlibdht::rpc::events::inter::message_event::MessageEvent;
use rlibdht::rpc::events::inter::response_callback::ResponseCallback;
use rlibdht::rpc::events::response_event::ResponseEvent;
use rlibdht::rpc::events::stalled_event::StalledEvent;

#[derive(Clone)]
pub struct BindTrackerResponseListener {
    routing_table: Arc<Mutex<dyn RoutingTable>>
}

impl BindTrackerResponseListener {

    pub fn new(routing_table: Arc<Mutex<dyn RoutingTable>>) -> Self {
        Self {
            routing_table
        }
    }
}

impl ResponseCallback for BindTrackerResponseListener {

    fn on_response(&self, _event: ResponseEvent) {
        self.routing_table.lock().unwrap().insert(_event.get_node());
        println!("BIND RESPONSE");
    }

    fn on_stalled(&self, _event: StalledEvent) {
        if _event.has_node() {
            _event.get_node().mark_stale(); //WILL THIS ACT CORRECTLY...? - THIS GOES FOR JAVA AS WELL...
        }
    }
}
