use num_complex::{Complex64};

pub struct Mandlebrot {
    func: fn(&Complex64, &Vec<Complex64>) -> Complex64
}

impl Mandlebrot {
    /*
     new
     
     Returns a new holomorphic function
    */
    pub fn new(f: fn(&Complex64, &Vec<Complex64>) -> Complex64) -> Self {
        Self {
            func: f
        }
    }
    /*
     iterate the mandlebrot function on initial input x with assignment of constants c

     Higher depth will produce more accurate results

     returns iteration number that function diverged at
    */
    pub fn iterate(&self, x: &Complex64, c: &Vec<Complex64>, depth: u32) -> f64 {
        let mut i = 0;
        
        let mut z = (self.func)(x, c);

        while i < depth && z.norm() < 32.0 {
            z = (self.func)(&z, c);
            i += 1;
        }

        return (i as f64 - z.norm().log2().log2()) / (depth as f64);
    }
}

pub fn shade(t: f64) -> (u8, u8, u8) {
    let a = (0.5, 0.5, 0.5);
    let b = (0.5, 0.5, 0.5);
    let c = (1.0, 1.0, 1.0);
    let d = (0.0, 0.10, 0.20);

    let r = (b.0 * (6.28318 * (c.0 * t + d.0)).cos() + a.0) * 255.0;
    let g = (b.1 * (6.28318 * (c.1 * t + d.1)).cos() + a.1) * 255.0;
    let b = (b.2 * (6.28318 * (c.2 * t + d.2)).cos() + a.2) * 255.0;

    (r as u8, g as u8, b as u8)
}