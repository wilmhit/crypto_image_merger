use image::{ImageBuffer, GrayImage, Luma, open};


fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Invalid arguments!");
        std::process::exit(1);
    }
    std::process::exit(merge_images(&args[1], &args[2]));
}

fn merge_images(filename1: &String, filename2: &String) -> i32 {
    let image1 = open(filename1).unwrap().into_luma8();
    let image2 = open(filename2).unwrap().into_luma8();
    let mut output_image: GrayImage = ImageBuffer::new(100, 100);
    if image1.dimensions() != (100, 100) || image2.dimensions() != (100, 100) {
        println!("Images are wring sizes!");
        return 1;
    }
    let mut x: u32 = 0;
    let mut y: u32 = 0;
    loop {
        if x == 100 {
            x = 0;
            y += 1;
            if y == 100 {
                break;
            }
        }
        let p1 = image1.get_pixel(x, y);
        let p2 = image2.get_pixel(x, y);
        let mut out_pixel: u8 = p1[0]/2;
        out_pixel = out_pixel.saturating_add(p2[0]/2);
        output_image.put_pixel(x, y, Luma([out_pixel]));
        println!("{}", out_pixel);
        x += 1;
    }
    output_image.save("a.png").unwrap();
    return 0;
}