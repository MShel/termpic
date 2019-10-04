extern crate websocket;
extern crate json;
extern crate rand;
extern crate base64;

pub mod frame_receiver {
	use websocket::sync::Server;
	use websocket::sync::Client;
	use websocket::OwnedMessage;
	use std::thread;
    use websocket::ws::dataframe::DataFrame;
	use std::net::TcpStream;
	use std::fs::File;
	use super::base64::{encode};
	use ascii_frame::ascii_frame;
    use std::io::prelude::*;
    use std::io::SeekFrom;

	pub fn start_web_socker_server() {
        let server = Server::bind("0.0.0.0:9999").unwrap();
        for request in server.filter_map(Result::ok) {
		thread::spawn(|| {
			if !request.protocols().contains(&"rust-websocket".to_string()) {
				request.reject().unwrap();
				return;
			}
			let mut client:Client<TcpStream> = request.use_protocol("rust-websocket").accept().unwrap();
			let client_id = get_unique_client_id(&client);
			let stream_name:String = encode(&client_id) + ".txt";
			//sending clientId back
    		let message = OwnedMessage::Text(stream_name.clone());
			client.send_message(&message).unwrap();
			let (mut receiver, mut sender) = client.split().unwrap();
			for message in receiver.incoming_messages() {
				let message = message.unwrap();
   
				match message {
					OwnedMessage::Close(_) => {
						let message = OwnedMessage::Close(None);
						sender.send_message(&message).unwrap();
						return;
					}
					OwnedMessage::Ping(ping) => {
						let message = OwnedMessage::Pong(ping);
		                println!("{}","*****");
                        println!("{:?}", message);
		                println!("{}","*****");
					}
					OwnedMessage::Text(text) => {
						// /**
						//  * we need to spin the th
						//  */
                        // let message: OwnedMessage = OwnedMessage::Text(text);
						// let bin_payload = &message.take_payload();
                        // let s = match std::str::from_utf8(bin_payload) {
                        //      Ok(v) => v,
                        //      Err(_) => "Invalid UTF-8 sequence"
                        // };
                        // let pixels_data = json::parse(s).unwrap();
						// let frame_to_record:String = ascii_frame::turn_js_image_data_to_ascii(pixels_data["data"].entries(), 220);
						// write_frame(frame_to_record, &stream_name)
                    }
					_ => sender.send_message(&message).unwrap(),
				}
			}
		});
       }
    }

	pub fn write_frame(ascii_frame:String, file_name: &String) {
		let mut file = File::create(file_name).unwrap();
		match file.set_len(0) {
			Ok(_)=>println!("file cleaned"),
			Err(err)=>eprintln!("{:?}", err)
		}
		match file.sync_all() {
			Ok(_)=>println!("changes synced w os"),
			Err(err)=>eprintln!("{:?}", err)
		}
		match file.write_all(ascii_frame.as_bytes()) {
			Ok(_) => println!("dumped frames contents"),
			Err(err)=>eprintln!("{:?}", err)
		}
		match file.seek(SeekFrom::Start(0)) {
			Ok(_) => println!("File pointer back to first line"),
			Err(err)=>eprintln!("{:?}", err)			
		}
	}
	 pub fn get_unique_client_id(ref client:&Client<TcpStream>)->String {
	 	 client.peer_addr().unwrap().to_string()
	 }
}