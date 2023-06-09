use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::sync::Arc;
use clap::{App, Arg};
use colored::*;

fn handle_client(mut client: TcpStream, server_addr: Arc<String>) {
    let mut server = TcpStream::connect(&*server_addr).unwrap();
    
    let mut client_to_server = client.try_clone().unwrap();
    let mut server_to_client = server.try_clone().unwrap();
    
    let client_to_server_thread = thread::spawn(move || {
        let mut buffer = [0; 1024];
        loop {
            let request_time = std::time::SystemTime::now();
            let bytes_read = client_to_server.read(&mut buffer).unwrap();
            if bytes_read == 0 { return; } // Client closed connection
            println!("{}", format!(
                "{}{}{:?}{}{}{}",
                "received ".blue().bold(),
                "response at ",
                request_time,
                " of size ",
                bytes_read.to_string().red(),
                " bytes"
            ));
            server.write(&buffer[..bytes_read]).unwrap();
        }
    });

    let server_to_client_thread = thread::spawn(move || {
        let mut buffer = [0; 1024];
        loop {
            let bytes_read = server_to_client.read(&mut buffer).unwrap();
            let request_time = std::time::SystemTime::now();
            if bytes_read == 0 { return; } // Server closed connection
            println!("{}", format!(
                "{}{}{:?}{}{}{}",
                "sent ".yellow().bold(),
                "response at ",
                request_time,
                " of size ",
                bytes_read.to_string().red(),
                " bytes"
            ));
            client.write(&buffer[..bytes_read]).unwrap();
        }
    });

    client_to_server_thread.join().unwrap();
    server_to_client_thread.join().unwrap();
}

fn main() {
    let matches = App::new("My Program")
        .arg(Arg::with_name("target-host")
            .short('h')
            .long("target-host")
            .takes_value(true)
            .required(true)
            .help("Target Host"))
        .arg(Arg::with_name("target-port")
            .short('p')
            .long("target-port")
            .takes_value(true)
            .required(true)
            .help("Target Port"))
        .arg(Arg::with_name("listener-port")
            .short('l')
            .long("listener-port")
            .takes_value(true)
            .required(true)
            .help("Listener Port"))
        .get_matches();

    let target_host = matches.value_of("target-host").unwrap();
    let target_port = matches.value_of("target-port").unwrap();
    let listener_port = matches.value_of("listener-port").unwrap();

    let server_addr = Arc::new(format!("{}:{}", target_host, target_port));
    let listener = TcpListener::bind(format!("localhost:{}", listener_port)).unwrap();
    let local_address = listener.local_addr().unwrap();

    println!("{}", format!(
        "{}{}:{}{}{}",
        "local proxy initialized. Pointing traffic from ".green(),
        local_address.ip().to_string().red().bold(),
        local_address.port().to_string().red().bold(),
        " to ".green(),
        server_addr.red().bold()
    ));

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let server_addr = Arc::clone(&server_addr);
        thread::spawn(|| {
            handle_client(stream, server_addr)
        });
    }
}
