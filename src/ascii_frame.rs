extern crate image;
extern crate terminal_size;

pub mod ascii_frame {
    use super::image;
    use super::image::DynamicImage;
    use super::image::GenericImageView;
    use super::terminal_size::{Width, Height, terminal_size};
    
    pub fn turn_rgb_frame_to_ascii(frame:DynamicImage, term_symbols_cache: &mut std::collections::HashMap<image::Rgba<u8>, String>) -> String {
        let mut asciid_frame = String::new(); 
        let size = terminal_size();
        let resized_frame;
        if let Some((Width(w), Height(h))) = size {
            resized_frame = frame.resize(w as u32, h as u32, image::FilterType::Nearest);
        } else {
            resized_frame = frame.resize(240, 320, image::FilterType::Nearest);
        }

        for row in 0..resized_frame.height() {
            let mut ascii_row = String::new();
            for column in 0..resized_frame.width() {
                let mut _ascii_symb = String::new();
                let pixel = resized_frame.get_pixel(column,row);
                match term_symbols_cache.get(&pixel) {
                    Some(symb) => _ascii_symb = symb.to_string(),
                    None => _ascii_symb = get_ascii_symb_for_pixel(pixel, term_symbols_cache)
                }
                ascii_row += &_ascii_symb;
            }
            asciid_frame += &ascii_row;
            asciid_frame += "\n";
        }
        asciid_frame
    }

    fn get_ascii_symb_for_pixel(
            pixel:image::Rgba<u8>,
            term_symbols_cache: &mut std::collections::HashMap<image::Rgba<u8>,String>
            ) -> String {
        let rgb_arr = turn_pixel_to_arr(pixel);
        let mut ascii_symb = String::from("\x1B[48;2;");
        ascii_symb+= &String::from(format!("{};{};{}",pixel[0],pixel[1],pixel[2]));
        ascii_symb+= &String::from("m");
        ascii_symb+= &get_ascii_for_rgb_arr(rgb_arr).to_string();
        ascii_symb+= &String::from("\x1B[0m");
        term_symbols_cache.insert(pixel, ascii_symb.to_string());
        ascii_symb
    }

    fn turn_pixel_to_arr(rgb_pix:image::Rgba<u8>) -> [f64;3]{
        [rgb_pix[1] as f64, rgb_pix[2] as f64, rgb_pix[3] as f64]
    }

    pub fn  get_ascii_for_rgb_arr(rgb_color:[f64;3]) -> char {
        let chars = b"MNHQ$OC?7>!:-;. ";
        let luminosity:f64 = 0.21 * rgb_color[0]+ 0.72 * rgb_color[1] + 0.07 * rgb_color[2];
        return chars[( luminosity / 256.0 * chars.len() as f64) as usize] as char;
    }
}
