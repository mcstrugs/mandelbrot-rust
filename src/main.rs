use num_complex::Complex64;

fn main() {
    asciiset(1000,500,1000);
}

fn asciiset(w: u32,h: u32,count: u32) {
    let diff_x: f64 = 4f64 / w as f64;
    let diff_y: f64 = 4f64 / h as f64;
    for i in 0..h {
        for j in 0..w {
            let comp = Complex64::new(-2.0 + diff_x * j as f64,2.0 - diff_y * i as f64);
            if in_set(comp, count) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }
}

fn in_set(c: Complex64, iter_count: u32) -> bool {
    let mut i = 0;
    let mut z = Complex64::new(0f64,0f64);
    loop {
        if (z.re*z.re + z.im*z.im) >= 4.0 {
            return false
        }
        i = i + 1;
        if i == iter_count {
            return true
        }
        z = z*z + c;
    }
}