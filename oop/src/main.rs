fn main() {

    let channel_a = CommunicationChannel::create("usb4", 897);
    channel_a.send("message 1");
    {
        let channel_b = CommunicationChannel::create("eth1", 12000);
        channel_b.send("message 2");
    }
    channel_a.send("message 3")
    
}

/* destructors
called when the object of
that type is deallocated
similar to defer for go lang
drop under rust
*/

struct CommunicationChannel{
    address: String,
    port: u16,
}

impl Drop for CommunicationChannel{
    fn drop(&mut self){
        println!("closing port {} : {}", self.address, self.port);
    }
}

impl CommunicationChannel{
fn create(address: &str, port:u16) -> CommunicationChannel{
    println!("Opening port {} : {}", address, port);
    CommunicationChannel{
            address:address.to_string(),
            port:port,
        }
    }

    fn send(&self, msg:&str){
        println!("sent to {}: {} the message {}", self.address, self.port, msg);
    }
}
