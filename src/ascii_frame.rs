extern crate image;
extern crate json;
extern crate rmpv;
extern crate  terminal_size;

pub mod ascii_frame {
    use super::image;
    use super::image::DynamicImage;
    use super::image::GenericImageView;
    use super::json::iterators::{Entries};
    use super::json::number::Number;
    use super::rmpv::Value;
    use rgb_ansi::ansi_color;
    use std::collections::HashMap;
    use super::image::Pixel;
    use super::terminal_size::{Width, Height, terminal_size};

    pub fn turn_image_data_to_ascii(pixels_data:Vec<Value>, term_colors:&Vec<([f64;3],String)>, color_map: &mut std::collections::HashMap<String,String>) -> String {
        let mut asciid_frame = String::new(); 
        let mut rgb_arr:[f64;3];
        let mut color:String;
        for row in pixels_data {
            let mut ascii_row = String::new();
            for rgba_arr_vals in row.as_array().unwrap() {
                let mut ascii_symb = String::new();
                if color_map.contains_key(&rgba_arr_vals.to_string()) {
                    match (color_map.get(&rgba_arr_vals.to_string())){
                            Some(symb) => ascii_symb = symb.to_string(),
                            None => ()
                    }
                } else {
                    rgb_arr = turn_vec_vals_to_arr(rgba_arr_vals.as_array().unwrap().to_vec());
                    color = ansi_color::get_ansi_color(rgb_arr, term_colors);
                    ascii_symb+= &String::from("\x1B[48;5;");
                    ascii_symb+=&color;
                    ascii_symb+=&String::from("m");
                    ascii_symb+=&get_ascii_for_rgb_arr(rgb_arr).to_string();
                    ascii_symb+= &String::from("\x1B[0m");
                    color_map.insert(rgba_arr_vals.to_string(), ascii_symb.to_string());
                }
                ascii_row += &ascii_symb;
            }
            asciid_frame += &ascii_row;
            asciid_frame += "\n";
        }               

        asciid_frame
    }

    fn turn_vec_vals_to_arr(rgb_vec:Vec<Value>) -> [f64;3]{
        [rgb_vec[0].as_f64().unwrap(), rgb_vec[1].as_f64().unwrap(), rgb_vec[2].as_f64().unwrap()]
    }
    
    pub fn turn_rgb_frame_to_ascii(frame:DynamicImage, term_colors:&Vec<([f64;3],String)>, color_map: &mut std::collections::HashMap<image::Rgba<u8>, String>) -> String {
        let mut asciid_frame = String::new(); 
        let size = terminal_size();
        let resized_frame;
        if let Some((Width(w), Height(h))) = size {
            println!("w {}, h {} ", w, h);
            resized_frame = frame.resize(w as u32, h as u32, image::FilterType::Nearest);
        } else {
            resized_frame = frame.resize(240, 320, image::FilterType::Nearest);
        }
        let mut rgb_arr:[f64;3];
        let mut color:String;
        println!("frame width {}, frame height {} ", frame.width(), frame.height());
        println!("frame width {}, frame height {} ", resized_frame.width(), resized_frame.height());
        
        //return String::from(" ")
        for row in 0..resized_frame.height() {
            let mut ascii_row = String::new();
            for column in 0..resized_frame.width() {
                let mut ascii_symb = String::new();
                let pixel = resized_frame.get_pixel(column,row);
                match color_map.get(&pixel) {
                    Some(symb) => ascii_symb = symb.to_string(),
                    None => ascii_symb = get_ascii_symb_for_pixel(pixel, color_map, term_colors)
                }
                ascii_row += &ascii_symb;
            }
            asciid_frame += &ascii_row;
            asciid_frame += "\n";
        }
        asciid_frame
    }

    fn get_ascii_symb_for_pixel(
            pixel:image::Rgba<u8>,
            color_map: &mut std::collections::HashMap<image::Rgba<u8>,String>,
            term_colors:&Vec<([f64;3],String)>
            ) -> String {
        let rgb_arr = turn_pixel_to_arr(pixel);
        //let color = ansi_color::get_ansi_color(rgb_arr, term_colors);
        let mut ascii_symb = String::from("\x1B[48;2;");
        ascii_symb+=&String::from(format!("{};{};{}",pixel[0],pixel[1],pixel[2]));

        ascii_symb+=&String::from("m");
        ascii_symb+=&get_ascii_for_rgb_arr(rgb_arr).to_string();
        ascii_symb+= &String::from("\x1B[0m");
        color_map.insert(pixel, ascii_symb.to_string());
        ascii_symb
    }

    fn turn_pixel_to_arr(rgb_pix:image::Rgba<u8>) -> [f64;3]{
        [rgb_pix[1] as f64, rgb_pix[2] as f64, rgb_pix[3] as f64]
    }

    pub fn  get_ascii_for_color(color:image::Rgba<u8>) -> char {
        let char_vec:Vec<char> = "MNHQ$OC?7>!:-;. ".chars().collect();
        let luminosity:f32 = (0.21 * (color[1] as f32) + 0.72 * (color[2] as f32) + 0.07 * (color[3] as f32)) as f32;
        return char_vec[( luminosity / 256.0 * char_vec.len() as f32) as usize];
    }

    pub fn  get_ascii_for_rgb_arr(rgb_color:[f64;3]) -> char {
        let char_vec:Vec<char> = "MNHQ$OC?7>!:-;. ".chars().collect();
        let luminosity:f64 = 0.21 * rgb_color[0]+ 0.72 * rgb_color[1] + 0.07 * rgb_color[2];
        return char_vec[( luminosity / 256.0 * char_vec.len() as f64) as usize];
    }

}


#[test]
fn test_get_ascii_color(){
    use ascii_frame::ascii_frame::get_ascii_for_color;

    let test_red = image::Rgba {
        data: [255, 0, 0, 0]
    };
    assert_eq!('M',get_ascii_for_color(test_red));

    let test_bright_green = image::Rgba {
        data: [12, 255, 124, 0]
    };
    assert_eq!('7',get_ascii_for_color(test_bright_green));

    let deep_blue = image::Rgba {
        data: [125, 0, 255, 0]
    };
    assert_eq!(':',get_ascii_for_color(deep_blue));
}