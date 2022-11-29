use std::f64::consts::PI;
use rayon::prelude::*;
use image::{ImageBuffer, RgbImage};
use num_complex::{Complex64};
mod mandlebrot;

fn main() {
    let num_frames = 1;
    let start_frame = 0;
    
    let max_iter = 255;
    let size = 512;
    let sx = size;
    let sy = size;

    let ww = 3.0; // window width
    let wh = 3.0; // window height

    let cx = 2.2; // center
    // let cy = 1.5;

    // let cx = 1.5; // center for julia
    let cy = 1.5;

    let dx = ww / sx as f64;
    let dy = wh / sy as f64;
    
    let dynamic = mandlebrot::Mandlebrot::new(|x, c| -> Complex64 {
        x * x + c.get(0).unwrap() // mandlebrot
    });

    // for animation
    let range = start_frame..(start_frame+num_frames);

    range.into_par_iter().for_each(|i| {
        // create black image
        let mut img: RgbImage = ImageBuffer::new(sx, sy);

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let re = (x as f64) * dx - cx;
            let im = (y as f64) * dy - cy;

            let n = (i as f64 / num_frames as f64) * PI;
            let z_0 = Complex64::new(n.sin(), n.sin());

            let c = Complex64::new(re, im);
            
            let t = dynamic.iterate(&z_0, &vec![c], max_iter);
            // let t = dynamic.iterate(&c, &vec![z_0], max_iter); // julia

            let (r,g,b) = mandlebrot::shade((2.0 * t + 0.5) % 1.0);

            *pixel = image::Rgb([r,g,b]);
        }
        
        img.save(format!("out/image-{}.png", i)).unwrap();
        println!("completed image-{}.png", i);
    });
}
