use std::{net::{TcpListener, TcpStream}, io::{Read, Write}};
use clap::Parser;

#[derive(Parser)]
struct Config {
	target_host: String,
	target_port: u16,
	bind_port: u16 
}

fn main() {
	let args = Config::parse();

	let local_listener = match TcpListener::bind(("127.0.0.1", args.bind_port)) {
		Ok(listener) => listener,
		Err(err) => { 
			eprintln!("There was an error binding the local listener: {}", err);

			std::process::exit(1);
		}
	};

	for incoming_stream in local_listener.incoming() {
		let mut stream = match incoming_stream {
		  Ok(stream) => stream,
			Err(err) => {
				eprintln!("There was an error processing an incoming stream: {}", err);

				continue;
			}
		};

		let mut target_stream = match TcpStream::connect(
			(args.target_host.clone(), args.target_port.clone())
		) {
			Ok(target_stream) => target_stream,
			Err(err) => {
				eprintln!("There was an error opening the remote stream: {}", err);

				continue;
			}
		};

		let mut copy_data = [0u8; 128];  

		if let Err(err) = stream.read(&mut copy_data) {
			eprintln!("There was an error reading the data to forward: {}", err);

			continue;
		}

		if let Err(err) = target_stream.write(&copy_data) {
			eprintln!("There was an error forwarding the data: {}", err);

			continue;
		}

		if let Err(err) = target_stream.flush() {
			eprintln!("There was an error sending the data: {}", err);

			continue;
		}

		let mut receiving_data = [0u8; 128];

		if let Err(err) = target_stream.read(&mut receiving_data) {
			eprintln!("There was an error receiving the data: {}", err);

			continue;
		}

		if let Err(err) = stream.write(&receiving_data) {
			eprintln!("There was an error returning the data: {}", err);

			continue;
		}

		if let Err(err) = stream.flush() {
			eprintln!("There was an error sending the return data: {}", err);

			continue;
		}
	}
}
