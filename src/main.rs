use image::png::PngEncoder;
use image::{DynamicImage, GenericImageView};
use rand::SeedableRng;
use rand::seq::SliceRandom;
use rand_pcg::Pcg64;

fn scramble_pixels(img: &DynamicImage, salt: u32) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut scrambled = image::ImageBuffer::new(width, height);
    let mut rng = Pcg64::seed_from_u64(salt as u64);
    let mut positions: Vec<(u32, u32)> = (0..width).flat_map(|x| (0..height).map(move |y| (x, y))).collect();
    
    positions.shuffle(&mut rng); // Corrected this line
    
    for (idx, (x, y)) in positions.iter().enumerate() {
        let (new_x, new_y) = (idx as u32 % width, idx as u32 / width);
        let pixel = img.get_pixel(*x, *y);
        scrambled.put_pixel(new_x, new_y, pixel);
    }
    
    DynamicImage::ImageRgba8(scrambled)
}

fn unscramble_pixels(img: &DynamicImage, salt: u32) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut unscrambled = image::ImageBuffer::new(width, height);
    let mut rng = Pcg64::seed_from_u64(salt as u64);
    let mut positions: Vec<(u32, u32)> = (0..width).flat_map(|x| (0..height).map(move |y| (x, y))).collect();
    
    positions.shuffle(&mut rng); // Corrected this line
    
    for (idx, (x, y)) in positions.iter().enumerate() {
        let (new_x, new_y) = (idx as u32 % width, idx as u32 / width);
        let pixel = img.get_pixel(new_x, new_y);
        unscrambled.put_pixel(*x, *y, pixel);
    }
    
    DynamicImage::ImageRgba8(unscrambled)
}

fn encode_image(img: &DynamicImage) -> Vec<u8> {
    let mut buf = Vec::new();
    let encoder = PngEncoder::new(&mut buf);
    encoder.encode(&img.to_bytes(), img.width(), img.height(), img.color())
        .expect("Error encoding image");
    buf
}

fn decode_image(buf: &[u8]) -> DynamicImage {
    let img = image::load_from_memory(buf).expect("Error decoding image");
    img
}

fn main() {
    let path = std::env::args().nth(1).expect("No path given");
    let name = std::path::Path::new(&path).file_name().expect("Invalid path");
    let name_str = name.to_str().expect("Invalid path");
    let name_no_ext = name_str.split('.').next().expect("Invalid path");
    let img = image::open(&path).expect("Error opening image");
    println!("Encode or decode? (e/d)");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    if input == "e" {
        let mut salt = String::new();
        println!("Enter salt value (integer):");
        std::io::stdin().read_line(&mut salt).unwrap();
        // transform salt to number
        let salt: u32 = salt.trim().parse().unwrap();
        let scrambled = scramble_pixels(&img, salt); // use some salt value
        let buf = encode_image(&scrambled);
        std::fs::write(format!("{}-scrambled.png", name_no_ext), buf).unwrap();
        println!("Saved scrambled image as {}-scrambled.png", name_no_ext);
    } else if input == "d" {
        let mut salt = String::new();
        println!("Enter salt value (integer):");
        std::io::stdin().read_line(&mut salt).unwrap();
        let salt: u32 = salt.trim().parse().unwrap();
        let buf = std::fs::read(format!("{}.png", name_no_ext)).unwrap();
        let img2 = decode_image(&buf);
        let unscrambled = unscramble_pixels(&img2, salt); // use the same salt value
        unscrambled.save(format!("{}-unscrambled.png", name_no_ext)).unwrap();  
        println!("Saved unscrambled image as {}-unscrambled.png", name_no_ext);
    } else {
        println!("Invalid input");
    }
}

