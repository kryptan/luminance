#![allow(non_snake_case)]

extern crate image;

use std::path::Path;
use image::GenericImage;

fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn clamp<T: PartialOrd>(a: T, minimum: T, maximum: T) -> T {
    max(minimum, min(maximum, a))
}

// sRGB electro-optical transfer function.
fn sRGB_eotf(e: f32) -> f32 {
    if e < 0.04045 {
        e/12.92
    } else {
        ((e + 0.055)/1.055).powf(2.4)
    }
}

// sRGB opto-electrical transfer function.
fn sRGB_oetf(o: f32) -> f32 {
    if o < 0.0031308 {
        o*12.92
    } else {
        1.055*o.powf(1.0/2.4) - 0.055
    }
}

fn sRGB_eotf_8bit(e: u8) -> f32 {
    sRGB_eotf((e as f32)*(1.0/255.0))
}

fn discretize_to_u8(v: f32) -> u8 {
    clamp(v*255.0, 0.0, 255.0).round() as u8
}

fn sRGB_oetf_8bit(o: f32) -> u8 {
    discretize_to_u8(sRGB_oetf(o))
}

fn main() {    // Use the open function to load an image from a Path.
    // ```open``` returns a dynamic image.
    let mut img = image::open(&Path::new("dark-purple-flower.png")).unwrap();

    // The dimensions method returns the images width and height
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's ColorType
    println!("{:?}", img.color());

    let gray = image::imageops::grayscale(&img);

    for pixel in img.as_mut_rgb8().unwrap().pixels_mut() {
        let r = sRGB_eotf_8bit(pixel.data[0]);
        let g = sRGB_eotf_8bit(pixel.data[1]);
        let b = sRGB_eotf_8bit(pixel.data[2]);
        let luminance = 0.2126*r + 0.7152*g + 0.0722*b;
        pixel.data[0] = sRGB_oetf_8bit(luminance);
        pixel.data[1] = sRGB_oetf_8bit(luminance);
        pixel.data[2] = sRGB_oetf_8bit(luminance);
    }

    // Write the contents of this image to the Writer in PNG format.
    let _ = gray.save(&Path::new("gray-luma.png")).unwrap();

    let _ = img.as_rgb8().unwrap().save(&Path::new("gray-luminance.png")).unwrap();
}
