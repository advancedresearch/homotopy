//! Helper methods for rendering and visualizing homotopy.

use std::io;

/// Creates a file name for a frame.
pub fn file_name(file: &str, frame: u32) -> String {
    format!("{}-{:04}.png", file, frame)
}

/// Clears an image with white color.
#[allow(non_snake_case)]
pub fn clear__file_size(file: &str, size: u32) -> io::Result<()> {
    use image::{open, Rgba, RgbaImage};

    let mut image = match open(file) {
        Ok(img) => img.to_rgba(),
        Err(_) => RgbaImage::new(size, size)
    };
    let (width, height) = image.dimensions();
    for y in 0..height {
        for x in 0..width {
            image.put_pixel(x, y, Rgba {data: [255; 4]});
        }
    }
    image.save(file)
}

/// Exports 2D plot, where x-axis and y-axis are normalized.
#[allow(non_snake_case)]
pub fn export2d__file_function_size_aabb_resolution<F: Fn(f64) -> [f64; 2]>(
    file: &str,
    fun: F,
    size: u32,
    aabb: ([f64; 2], [f64; 2]),
    resolution: u32,
) -> io::Result<()> {
    use image::{Rgba, RgbaImage};

    let mut image = RgbaImage::new(size, size);

    for y in 0..size {
        for x in 0..size {
            image.put_pixel(x, y, Rgba {data: [255; 4]});
        }
    }
    for i in 0..resolution {
        let f = i as f64 / resolution as f64;
        let pos = fun(f);
        if pos[0] < aabb.0[0] ||
           pos[1] < aabb.0[1] ||
           pos[0] >= aabb.1[0] ||
           pos[1] >= aabb.1[1] {continue};
        let x = (pos[0] - aabb.0[0]) / (aabb.1[0] - aabb.0[0]) * size as f64;
        let y = (pos[1] - aabb.0[1]) / (aabb.1[1] - aabb.0[1]) * size as f64;
        image.put_pixel(x as u32, y as u32, Rgba {data: [0, 0, 0, 255]});
    }
    image.save(file)
}

/// Adds plots to existing image.
#[allow(non_snake_case)]
pub fn overlay2d__file_function_aabb_resolution<F: Fn(f64) -> [f64; 2]>(
    file: &str,
    fun: F,
    aabb: ([f64; 2], [f64; 2]),
    resolution: u32,
) -> io::Result<()> {
    use std::io::ErrorKind;
    use image::{open, Rgba};

    let mut image = open(file)
        .map_err(|_| io::Error::new(ErrorKind::Other, "Could not open image"))?.to_rgba();
    let (width, height) = image.dimensions();

    for i in 0..resolution + 1 {
        let f = i as f64 / resolution as f64;
        let pos = fun(f);
        if pos[0] < aabb.0[0] ||
           pos[1] < aabb.0[1] ||
           pos[0] >= aabb.1[0] ||
           pos[1] >= aabb.1[1] {continue};
        let x = (pos[0] - aabb.0[0]) / (aabb.1[0] - aabb.0[0]) * width as f64;
        let y = (pos[1] - aabb.0[1]) / (aabb.1[1] - aabb.0[1]) * height as f64;
        image.put_pixel(x as u32, y as u32, Rgba {data: [0, 0, 0, 255]});
    }
    image.save(file)
}

/// Do something with a specified resolution.
pub fn resolution<F: Fn(f64)>(n: u32, fx: F) {
    for i in 0..n + 1 {
        let f = i as f64 / n as f64;
        fx(f)
    }
}
