use macroquad::prelude::*;
use std::io::prelude::*;
fn main() {
    let ascii = "`^\",:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
    let ascii_vec: Vec<char> = ascii.chars().collect();
    
    let max_brightness: f32 = 765.0;
    let weight_brightness: f32 = ascii_vec.len() as f32 / max_brightness;
    let image = Image::from_file_with_format(include_bytes!("image.png"), Some(ImageFormat::Png));
    let h = image.height() as u32;
    let w = image.width() as u32;
   
    let mut lines: Vec<String> = Vec::with_capacity(2);

    for y in (0..h).step_by(6){
        let mut line = String::from("");
        for x in (0..w).step_by(6){
            let pixel = image.get_pixel(x, y);
            let mut pixel_brightness = pixel.r + pixel.g + pixel.b;
            pixel_brightness = pixel_brightness * 255.0;
            let index = (pixel_brightness as f32 * weight_brightness) - 1 as f32;
            line.push(ascii_vec[index as usize]);
        }
        line.push('\n');
        print!("{}", line);
        lines.push(line);
    }
    let mut f = std::fs::File::create("text.txt").unwrap();
    for str in lines.iter(){
        f.write_all(str.as_bytes()).unwrap();
    }
}
