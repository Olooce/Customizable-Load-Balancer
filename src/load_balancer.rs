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
        let server = Server { id, name, address };
        self.servers.push(server);
        println!("Server added: {}", name);
    }

    pub fn remove_server(&mut self, id: u64) {
        self.servers.retain(|server| server.id != id);
        println!("Server with id {} removed", id);
    }

    pub fn rep(&self) -> Option<&Server> {
        self.servers.first()
    }
}
