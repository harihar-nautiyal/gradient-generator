use image::{ImageBuffer};

fn main() {
    let mut img = ImageBuffer::new(1900, 1000);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r = (0.2 * x as f32) as u8;
        let b = (0.2 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    img.save("gradient.png").unwrap()
}
