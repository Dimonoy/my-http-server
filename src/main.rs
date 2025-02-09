use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::error::Error;
use std::io::{BufReader, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let exit_code = main_impl()?;
    std::process::exit(exit_code);
}

fn main_impl() -> Result<i32, Box<dyn Error>> {
    let addr = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        8000,
    );
    let listener = TcpListener::bind(addr)?;

    loop {
        match listener.accept() {
            Ok((stream, client_addr)) => handle_connection(stream, client_addr)?,
            Err(_) => break,
        }
    }

    Ok(0)
}

fn handle_connection(stream: TcpStream, client_addr: SocketAddr) -> Result<(), Box<dyn Error>>{
    let mut buf_reader = BufReader::new(&stream);
    let mut request = String::new();

    let _ = buf_reader.read_to_string(&mut request)?;


    println!("Client {client_addr:?}: {}", request.trim());
    
    Ok(())
}
