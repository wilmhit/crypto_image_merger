use image::open;
use clap::{App, Arg};

mod processing;

fn main() {
    let command_matches = App::new("Visual crytography tool")
        .version("1.0")
        .author("Jcob Chodubski")
        .arg(Arg::with_name("encrypt")
            .long("encrypt")
            .short("e")
            .takes_value(true)
            .required_unless("decrypt")
            .conflicts_with("decrypt")
            .value_name("INPUT")
            .number_of_values(1))
        .arg(Arg::with_name("decrypt")
            .long("decrypt")
            .short("d")
            .takes_value(true)
            .required_unless("encrypt")
            .conflicts_with("encrypt")
            .value_names(&["INPUT1", "INPUT2"])
            .number_of_values(2))
        .get_matches();

    if let Some(vals) = command_matches.values_of("decrypt") {
        let values: Vec<&str> = vals.collect();
        decrypt(values[0], values[1]);
    }
    if let Some(val) = command_matches.value_of("encrypt") {
        encrypt(val);
    }
}

fn encrypt(filename: &str) {
    let input = open(filename).unwrap().into_luma8();
    let (x, y) = input.dimensions();
    let noise = processing::generate_noise(x*2, y*2);
    let average = processing::get_average_brightness(&input);
    processing::imprint_image(&noise, &input, average).save("part1.png").unwrap();
    noise.save("part2.png").unwrap();
}

fn decrypt(filename1: &str, filename2: &str) {
    let image1 = open(filename1).unwrap().into_luma8();
    let image2 = open(filename2).unwrap().into_luma8();
    let noisy = processing::merge_images(&image1, &image2);
    processing::remove_noise(&noisy).save("a.png").unwrap();
}