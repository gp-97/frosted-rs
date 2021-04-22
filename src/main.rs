mod args_cleanup;
mod effect;
use photon_rs::native::{open_image, save_image};
use photon_rs::PhotonImage;
use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args_cleanup::clean(&mut args);

    let img_path = args[1].as_str();
    let op_path = args[2].as_str();

    let img = open_image(img_path).expect("No such file found");

    let img: PhotonImage = effect::frosted_glass(&img);
    save_image(img, op_path);
}
