use std::str::FromStr;
use num::Complex;
use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;

fn main() {
    println!("Hello, world!");
}

fn _square_loop(mut x: u64) {
    loop {
        x = x * x;
    }
}

fn _square_add_loop(c: f64) {
    let mut x = 0.;
    loop {
        x = x * x + c;
    }
}

fn _complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
}

fn _escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}

fn _parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(_parse_pair::<i32>("", ','), None);
    assert_eq!(_parse_pair::<i32>("10", ','), None);
    assert_eq!(_parse_pair::<i32>(",10", ','), None);
    assert_eq!(_parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(_parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(_parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(_parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

fn _parse_complex(s: &str) -> Option<Complex<f64>> {
    match _parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        _ => None
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(_parse_complex("1.25,-6.23"), Some(Complex { re: 1.25, im: -6.23 }));
    assert_eq!(_parse_complex(",-0.0625"), None);
}

fn _pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (lower_right.re - upper_left.re, upper_left.im - lower_right.im);

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(_pixel_to_point(
        (100, 200),
        (25, 175),
        Complex { re: -1.0, im: 1.0 },
        Complex { re: 1.0, im: -1.0 }),
               Complex { re: -0.5, im: -0.75 }
    );
}

fn _render(pixels: &mut [u8],
          bounds: (usize, usize),
          upper_left: Complex<f64>,
          lower_right: Complex<f64>) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = _pixel_to_point(bounds, (column, row), upper_left, lower_right);

            pixels[row * bounds.0 + column] =
                match _escape_time(point, 255) {
                    None => 0,
                    Some(count) => 255 - count as u8
                };
        }
    }
}

fn _write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {
    let output = match File::create(filename) {
        Ok(f) => f,
        Err(e) => {
            return Err(e);
        }
    };
    let encoder = PNGEncoder::new(output);
    encoder.encode(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;

    Ok(())
}

 // test 3