mod server;

use std::net::{TcpListener, SocketAddr, TcpStream, Shutdown, SocketAddrV4, Ipv4Addr, IpAddr};
use std::str::FromStr;
use std::thread::{spawn, JoinHandle};
use std::io::{BufReader, BufWriter, Read, Write};
use std::time::Duration;

fn main() {
    let login = server::server::Server {
        name: "login".to_string(),
        local_port: 6901,
        target: SocketAddr::new(IpAddr::from(Ipv4Addr::new(127, 0, 0, 1)), 6900)
    };
    let char = server::server::Server {
        name: "char".to_string(),
        local_port: 6123,
        target: SocketAddr::new(IpAddr::from(Ipv4Addr::new(127, 0, 0, 1)), 6121)
    };
    let map = server::server::Server {
        name: "map".to_string(),
        local_port: 6124,
        target: SocketAddr::new(IpAddr::from(Ipv4Addr::new(127, 0, 0, 1)), 6122)
    };
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    &handles.push(login.proxy());
    &handles.push(char.proxy());
    &handles.push(map.proxy());

    for handle in handles {
        handle.join();
    }
}