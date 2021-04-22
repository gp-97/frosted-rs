use perlin2d::PerlinNoise2D;
use photon_rs::native::{open_image, save_image};
use photon_rs::PhotonImage;

fn main() {
    let img_path = "/home/gp/Projects/filter/assets/image0.jpg";
    let op_path = "/home/gp/Projects/filter/assets/image0_result.jpg";
    let img = open_image(img_path).expect("Bruh");
    let width = img.get_width();
    let height = img.get_height();

    let perlin = PerlinNoise2D::new(2, 10.0, 10.0, 10.0, 1.0, (100.0, 100.0), 0.5, 101);
    let img = distort(&img, width, height, &perlin);
    save_image(img, op_path);
}

fn distort(img: &PhotonImage, width: u32, height: u32, perlin: &PerlinNoise2D) -> PhotonImage {
    let img_orig_buf = img.get_raw_pixels();
    let mut img_buf = Vec::<u8>::new();
    let end = width * height * 4;
    Vec::resize(&mut img_buf, end as usize, 0_u8);

    for pixel in (0..end).step_by(4) {
        let x = (pixel / 4) / width;
        let y = (pixel / 4) % width;

        let res = [
            perlin.get_noise(x as f64, y as f64) - 0.5,
            (perlin.get_noise(100.0 + x as f64, y as f64) - 0.5) * 4.0,
        ];

        let x_new = f64::clamp(f64::floor(x as f64 + res[0]), 0.0, height as f64 - 1.0);
        let x_new = x_new as usize;
        let y_new = f64::clamp(f64::floor(y as f64 + res[1]), 0.0, width as f64 - 1.0);
        let y_new = y_new as usize;

        let pixel_new = (x_new * width as usize + y_new) * 4;
        if pixel_new > end as usize {
            continue;
        }
        img_buf[pixel as usize] = img_orig_buf[pixel_new as usize];
        img_buf[pixel as usize + 1] = img_orig_buf[pixel_new as usize + 1];
        img_buf[pixel as usize + 2] = img_orig_buf[pixel_new as usize + 2];
        img_buf[pixel as usize + 3] = img_orig_buf[pixel_new as usize + 3];
    }

    PhotonImage::new(img_buf, width, height)
}
