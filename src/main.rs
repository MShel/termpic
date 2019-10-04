extern crate rand;
extern crate image;
extern crate num;
extern crate websocket;
extern crate serde;
use serde::{Deserialize, Serialize};

mod ascii_frame;
mod ws_socket_server;
pub use ascii_frame::ascii_frame::turn_image_data_to_ascii;
pub use ascii_frame::ascii_frame::turn_rgb_frame_to_ascii;
pub use ws_socket_server::frame_receiver::start_web_socker_server;
use std::net::{TcpListener};
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

extern crate rmp_serde;
extern crate rmpv;
extern crate hex;
mod rgb_ansi;
use std::env;

fn main() {
   let args: Vec<String> = env::args().collect();
   open_image(args);
   //start_tcp_server();
}

fn open_image(params: Vec<String>) {
    println!("{:?}", params);
    if params.len() < 2 {
        println!("{}", get_usage());
        std::process::exit(255);
    }

    let imageResult =  image::open(&params[1]);
    match imageResult {
        Ok (mut unwrapped_image) => {
            let term_color_map = rgb_ansi::ansi_color::populate_term_colors_map();
            let mut color_cache = HashMap::new();
            let aciid_image:String = turn_rgb_frame_to_ascii(unwrapped_image, &term_color_map, &mut color_cache);
            print!("{}", aciid_image);
        }
        Err(message) => {
            println!("{:?}", message)
        }
    }
}


fn get_usage() -> String {
    let usage:String = String::from("
        Usage: \n
        picview {filename} \n
        === \n
        Ex. picview example.png 
    ");

    usage
}

fn start_tcp_server() ->std::io::Result<()> {
    let mut listener = TcpListener::bind("0.0.0.0:12345")?;
    let mut buf = Vec::new(); 
    let term_color_map = rgb_ansi::ansi_color::populate_term_colors_map();
    let mut color_cache = HashMap::new();
    loop {
       for stream in listener.incoming() {
           match stream {
                Ok(mut stream) => {
                    print!("{}[2J", 27 as char);
                    stream.read_to_end(&mut buf); 
                    let value:rmpv::Value = rmp_serde::from_slice(&buf[..]).unwrap();
                    let frame_to_record:String = turn_image_data_to_ascii(value.as_array().unwrap().to_vec(), &term_color_map, &mut color_cache);
                    print!("{}", frame_to_record);
                }
                Err(_) => {}
                }
       }
    }

    Ok(())
}




