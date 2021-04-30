use num_complex::Complex64;
use array2d::Array2D;
extern crate image;

#[derive(Clone,Copy)]
struct Point {
    in_set: bool,
    iters: u32,
}

fn main() {
    let height: u32 = 1000;
    let width: u32 = 1000;
    let start_x: f64 = -1.78;
    let start_y: f64 = 0.01905;
    let box_w: f64 = 0.037;
    let box_h: f64 = box_w;
    let iters: u32 = 1000;
    let set = point_set(height,width,start_x,start_y,box_w,box_h,iters);
    draw_points(set);
}

#[allow(dead_code)]
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

#[allow(dead_code)]
fn print_set(set: Array2D<bool>) {
    for row in set.rows_iter() {
        for element in row {
            if *element {
                print!("#");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }
}

fn draw_set(set: Array2D<bool>) {
    let imgy = set.column_len() as u32;
    let imgx = set.row_len() as u32;
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
    for (x,y,pix) in imgbuf.enumerate_pixels_mut() {
        let is_shaded = *set.get(y as usize, x as usize).unwrap();
        if is_shaded {
            *pix = image::Rgb([255 as u8,255 as u8,255 as u8]);
        }
    }

    imgbuf.save("img.png").unwrap();
}

fn draw_points(set: Array2D<Point>) {
    let imgy = set.column_len() as u32;
    let imgx = set.row_len() as u32;
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
    for (x,y,pix) in imgbuf.enumerate_pixels_mut() {
        let pnt = *set.get(y as usize, x as usize).unwrap();
        if pnt.in_set {
            *pix = image::Rgb([255 as u8,255 as u8,255 as u8]);
        } else {
            let b: u8 = (pnt.iters % 255) as u8;
            *pix = image::Rgb([b,b,b]);
        }
    }

    imgbuf.save("img.png").unwrap();
}

#[allow(dead_code)]
fn bool_set(w: u32, h: u32, start_x: f64, start_y: f64, box_width: f64, box_height: f64, count: u32) -> Array2D<bool> {
    let mut set = Array2D::filled_with(false, h as usize,w as usize);
    let diff_x: f64 = box_width / w as f64;
    let diff_y: f64 = box_height / h as f64;
    for i in 0..h {
        for j in 0..w {
            let comp = Complex64::new(start_x + diff_x * j as f64,start_y - diff_y * i as f64);
            if in_set(comp, count) {
                set.set(i as usize,j as usize,true);
            }
        }
    }
    set
}

#[allow(dead_code)]
fn point_set(w: u32, h: u32, start_x: f64, start_y: f64, box_width: f64, box_height: f64, count: u32) -> Array2D<Point> {
    let mut set = Array2D::filled_with(Point{in_set: false, iters: 0}, h as usize,w as usize);
    let diff_x: f64 = box_width / w as f64;
    let diff_y: f64 = box_height / h as f64;
    for i in 0..h {
        for j in 0..w {
            let comp = Complex64::new(start_x + diff_x * j as f64,start_y - diff_y * i as f64);
            set.set(i as usize, j as usize, get_point(comp,count));
        }
    }
    set
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

fn get_point(c: Complex64, iter_count: u32) -> Point {
    let mut i = 0;
    let mut z = Complex64::new(0f64,0f64);
    loop {
        if (z.re * z.re + z.im*z.im) >= 4.0 {
            return Point{in_set: false, iters: i};
        }
        i = i + 1;
        if i == iter_count {
            return Point{in_set: true, iters: 0};
        }
        z = z*z + c;
    }
}