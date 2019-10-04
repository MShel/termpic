extern crate image;

mod ascii_frame;
pub use ascii_frame::ascii_frame::turn_rgb_frame_to_ascii;
use std::collections::HashMap;

use std::env;

fn main() {
   let args: Vec<String> = env::args().collect();
   open_image(args);
}


fn open_image(params: Vec<String>) {
    if params.len() < 2 {
        println!("{}", get_usage());
        std::process::exit(255);
    }

    let image_result =  image::open(&params[1]);
    match image_result {
        Ok (unwrapped_image) => {
            let mut term_symbols_cache = HashMap::new();
            let aciid_image:String = turn_rgb_frame_to_ascii(unwrapped_image, &mut term_symbols_cache);
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
        termpic {filename} \n
        === \n
        Ex. termpic example.png 
    ");
    usage
}




