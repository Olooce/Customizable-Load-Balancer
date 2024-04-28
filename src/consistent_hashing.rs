use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

const NUM_SERVER_CONTAINERS: usize = 3; // N
const TOTAL_SLOTS: usize = 512; // #slots
const VIRTUAL_SERVERS_PER_CONTAINER: usize = 9; // K

// Hash function for request mapping
fn hash_request(req_id: usize) -> usize {
    (req_id + 2 * req_id + 17) % TOTAL_SLOTS
}

// Hash function for virtual server mapping
fn hash_virtual_server(container_id: usize, vs_index: usize) -> usize {
    (container_id + vs_index + 2 * vs_index + 25) % TOTAL_SLOTS
}

// ServerContainer represents a single server container in the load balancer
#[derive(Clone)]
pub struct ServerContainer {
    pub id: usize,
    pub name: String,
}

// VirtualServer represents a virtual server in the consistent hash map
pub struct VirtualServer {
    pub server_container: Arc<ServerContainer>,
    pub slot: usize,
}

// ServerPool maintains a consistent hash map and manages server containers
pub struct ServerPool {
    servers: Vec<Arc<ServerContainer>>,
    hash_map: Arc<RwLock<BTreeMap<usize, Arc<VirtualServer>>>>,
    slots: usize,
}

impl ServerPool {

}