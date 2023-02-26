use std::fs::File;
use std::io::prelude::*;
use std::env;

fn main() {
    let width;
    let height;
    let max_iterations;

    let mut buffer = String::new();

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        width = 250;
        height = 100;
        max_iterations = 50;
    } else {
        width = match args[1].parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("\tTroubles to parse your argument to \"i32\" ");
                return;
            }
        };
        height = match args[2].parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("\tTroubles to parse your argument to \"i32\" ");
                return;
            }
        };
        max_iterations = match args[3].parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("\tTroubles to parse your argument to \"i32\" ");
                return;
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            let cx = (x as f64 - width as f64 / 2.0) * 4.0 / width as f64;
            let cy = (y as f64 - height as f64 / 2.0) * 4.0 / width as f64;

            let mut zx = 0.0;
            let mut zy = 0.0;
            let mut i = 0;

            while i < max_iterations && zx * zx + zy * zy < 4.0 {
                let zx_temp = zx * zx - zy * zy + cx;
                zy = 2.0 * zx * zy + cy;
                zx = zx_temp;
                i += 1;
            }

            let c = if i == max_iterations {
                ' '
            } else {
                let palette_index = i % 8;
                match palette_index {
                    0 => '.',
                    1 => ':',
                    2 => '-',
                    3 => '=',
                    4 => '+',
                    5 => '*',
                    6 => '#',
                    _ => '%',
                }
            };
            buffer.push(c);
        }
        buffer.push('\n');
    }

    let mut file = File::create("mandelbrot.txt").unwrap();
    file.write_all(buffer.as_bytes()).unwrap();
}
