pub struct Server {
    addr: String,
}
impl Server {
    pub fn new(address: String) -> Server {
        Server {
            addr: address
        }
    }
    pub fn run(self) {
        println!("Listening on {}", self.addr)
    }
}