use image::{ImageBuffer, GrayImage, Luma};

const BLACK: u8 = 0;
const WHITE: u8 = 255;

pub fn merge_images(image1: &GrayImage, image2: &GrayImage) -> GrayImage {
    let (size_x, size_y) = image1.dimensions();
    let mut output_image: GrayImage = ImageBuffer::new(size_x, size_y);
    if image1.dimensions() != image2.dimensions() {
        panic!("Images are wring sizes!");
    }
    for x in 0..size_x {
        for y in 0..size_y {
            let p1 = image1.get_pixel(x, y)[0];
            let p2 = image2.get_pixel(x, y)[0];
            let mut out_pixel: u8 = p1;
            if p2 < p1 {
                out_pixel = p2;
            }
            output_image.put_pixel(x, y, Luma([out_pixel]));
        }
    }
    return output_image;
}

pub fn generate_noise(size_x: u32, size_y: u32) -> GrayImage {
    let mut image = GrayImage::new(size_x, size_y);
    for x in (0..size_x).step_by(2) {
        for y in (0..size_y).step_by(2) {
            let pixels: Vec<_> = random_4().iter()
                .map(|is_white| {
                    if *is_white {
                        return Luma([WHITE]);
                    } else {
                        return Luma([BLACK]);
                    }
                })
                .collect();
            image.put_pixel(x, y, pixels[0]);
            image.put_pixel(x, y+1, pixels[1]);
            image.put_pixel(x+1, y, pixels[2]);
            image.put_pixel(x+1, y+1, pixels[3]);
        }
    }
    return image;
}

fn random_4() -> Vec<bool> {
    let mut val = vec![false, false, false, false];
    while val.iter().filter(|&x| *x == true).count() != 2 {
        for x in val.iter_mut() {
            *x = rand::random::<bool>();
        }
    }
    return val;
}

pub fn imprint_image(noise: &GrayImage, orginal: &GrayImage) -> GrayImage {
    let (size_x, size_y) = orginal.dimensions();
    let mut image =  GrayImage::new(size_x*2, size_y*2);
    for x in 0..size_x {
        for y in 0..size_y {
            let nx = x*2;
            let ny = y*2;
            let color = orginal.get_pixel(x, y)[0] == WHITE;
            let should_be: Vec<_> = vec![
                noise.get_pixel(nx, ny),
                noise.get_pixel(nx, ny+1),
                noise.get_pixel(nx+1, ny),
                noise.get_pixel(nx+1, ny+1)
            ]
            .iter()
            .map(|&pix| color_of_pixel(color, pix[0] == WHITE))
            .collect();
            image.put_pixel(nx, ny, Luma([should_be[0]]));
            image.put_pixel(nx, ny+1, Luma([should_be[1]]));
            image.put_pixel(nx+1, ny, Luma([should_be[2]]));
            image.put_pixel(nx+1, ny+1, Luma([should_be[3]]));
        }
    }
    return image;
}

fn color_of_pixel(org_white: bool, noise_white: bool) -> u8 {
    if org_white && noise_white {
        return WHITE;
    }
    if !org_white && !noise_white {
        return WHITE;
    }
    return BLACK;
}

pub fn remove_noise(noisy_image: &GrayImage) -> GrayImage {
    let (mut size_x, mut size_y) = noisy_image.dimensions();
    size_x /= 2;
    size_y /= 2;
    let mut denoised = GrayImage::new(size_x, size_y);
    for x in 0..size_x {
        for y in 0..size_y {
            let nx = x*2;
            let ny = y*2;
            let pixels = [
                noisy_image.get_pixel(nx, ny)[0],
                noisy_image.get_pixel(nx, ny+1)[0],
                noisy_image.get_pixel(nx+1, ny)[0],
                noisy_image.get_pixel(nx+1, ny+1)[0],
            ];
            let all_black = pixels.iter().filter(|&x| *x == WHITE).count() == 0;
            if all_black{
                denoised.put_pixel(x, y, Luma([BLACK]));
            } else {
                denoised.put_pixel(x, y, Luma([WHITE]));
            }
        }
    }
    return denoised;
}