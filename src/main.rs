use oping::{Ping, PingResult};

fn main() {
    let mut ping = Ping::new();
    try!(ping.set_timeout(5));
    try!(ping.add_host("192.168.32.1"));

    let responses = try!(ping.send());
    for resp in responses {
        if resp.dropped > 0 {
            println!("No response from host: {}", resp.hostname);
        } else {
            println!("Response from host {} (address {}): latency {} ms",
                resp.hostname, resp.address, resp.latency_ms);
            println!("    all details: {:?}", resp);
        }
    }
}