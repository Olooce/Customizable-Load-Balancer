use std::sync::{Arc, RwLock};

#[derive(Clone)] // This line allows Server instances to be cloned
pub struct Server {
    pub id: u64,
    pub name: String,
    pub address: String,
}

pub struct LoadBalancer {
    servers: Vec<Server>,
}

impl LoadBalancer {
    pub fn new() -> Self {
        LoadBalancer {
            servers: Vec::new(),
        }
    }

    pub fn add_server(&mut self, id: u64, name: String, address: String) {
        let server_name = name.clone(); // Clone the name before moving it
        let server = Server { id, name, address };
        self.servers.push(server);
        println!("Server added: {}", server_name);
    }

    pub fn remove_server(&mut self, id: u64) {
        self.servers.retain(|server| server.id != id);
        println!("Server with id {} removed", id);
    }

    pub fn rep(&self) -> Option<Server> {
              self.servers.first().cloned() 
    }
}

// Wrapper functions to interact with the LoadBalancer

pub fn add_server(lb: Arc<RwLock<LoadBalancer>>, id: u64, name: String, address: String) {
    let mut lb = lb.write().unwrap();
    lb.add_server(id, name, address);
}

pub fn remove_server(lb: Arc<RwLock<LoadBalancer>>, id: u64) {
    let mut lb = lb.write().unwrap();
    lb.remove_server(id);
}

pub fn rep(lb: Arc<RwLock<LoadBalancer>>) -> Option<Server> {
    let lb = lb.read().unwrap();
    lb.rep()
}
