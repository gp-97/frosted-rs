use perlin2d::PerlinNoise2D;
use photon_rs::PhotonImage;

pub fn frosted_glass(img: &PhotonImage) -> PhotonImage {
    let perlin = PerlinNoise2D::new(2, 10.0, 10.0, 10.0, 1.0, (100.0, 100.0), 0.5, 101);

    let img_orig_buf = img.get_raw_pixels();
    let end = img_orig_buf.len();
    let width = img.get_width();
    let height = img.get_height();

    let mut img_buf = Vec::<u8>::new();
    Vec::resize(&mut img_buf, end as usize, 0_u8);

    for pixel in (0..end).step_by(4) {
        let x = (pixel / 4) / width as usize;
        let y = (pixel / 4) % width as usize;

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
