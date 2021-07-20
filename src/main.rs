extern crate image;

fn main() {
    let imgx = 1000;
    let imgy = 1000;

    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.6 * y as f32) as u8;
        let g = (0.2 * y as f32) as u8;

        if x > 0 && y > 0 && (x * 2) % y == 1 {
            *pixel = image::Rgb([255, 255, 255]);
        } else {
            *pixel = image::Rgb([r, g, b])
        }
    }

    imgbuf.save("fractal.png").unwrap();
}
