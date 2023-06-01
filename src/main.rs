use mpi::traits::*;

use std::time::{Duration, SystemTime};
use std::thread::sleep;
use std::time;

fn main() {
    
    // Config
    let height:usize = 2240;
    let width:usize = 2240;
    let iterations = 8000;
    let zoomFactor: f64 = 200000000000000.0;
    let scale = 1.0 / zoomFactor;
    // Center point
    let cX = -1.74995768370609350360221450607069970727110579726252077930242837820286008082972804887218672784431700831100544507655659531379747541999999995;
    let cY =  0.0000000000000000027879370656337940217829475379094436492708505450016308137904393065018938684976520216947747055;

    // Open MPI
    let universe = mpi::initialize().unwrap();
    let world = universe.world();

    let size = world.size() as usize;
    let rank = world.rank() as usize;
    let root_process = world.process_at_rank(0);

    // Begin
    println!("Start node: {}/{}, {}x{}, {}, {}", rank, size, width, height, iterations, zoomFactor);

    // Size of image this node should calculate
    let imgx = width as usize;
    let imgy = (height / size) as usize;

    // Used to store pixel values
    let mut data: Vec<u8> = vec![0u8; (imgx * imgy) as usize];

    // The y component for the current pixel being calculated
    let mut y1: f64 = (height as f64 / -2.0) * scale + cY + scale * (imgy * rank) as f64;

    // Calculate each pixel
    for y in 0..imgy{
        let timer = SystemTime::now();

        // The x component for the current pixel being calculated
        let mut x1: f64 = (imgx as f64 / -2.0) * scale + &cX;

        for x in 0..imgx {

            // Mandelbrot escape time algorithm
            let mut zX1 = x1;
            let mut zY1 = y1;

            let mut zX2 = &zX1 * &zX1;
            let mut zY2 = &zY1 * &zY1;

            let mut i = 0;
            while i < iterations && &zX2 + &zY2 < 4.0 {
                zX2 = &zX1 * &zX1;
                zY2 = &zY1 * &zY1;
                
                zY1 = &zX1 * &zY1 + &zX1 * &zY1 + &y1;
                zX1 = &zX2 - &zY2 + &x1;
                i += 1;
            }
            
            // Store the color value based on the escape time
            data[y * imgx + x] = (i % 16) as u8;
            
            // Move across to the next pixel
            x1 += &scale;
        }

        // Move down to the next row
        y1 += &scale;

        // Status update
        match timer.elapsed() {
            Ok(elapsed) => {
                println!("Rank{} {}/{} {}ms", rank, y, imgy, elapsed.as_millis());
            }
            Err(e) => {
                println!("Error: {e:?}");
            }
        }
    }
   

    if world.rank() == 0 {
        // Collect data on the root node
        let mut t = vec![0u8; width * height];
        root_process.gather_into_root(&data[..], &mut t[..]);

        // Draw collected pixel data to the image
        let mut imageBuffer = image::ImageBuffer::new(width as u32, height as u32);
        for y in 0..height {
            for x in 0..width {
                let pixel = imageBuffer.get_pixel_mut(x as u32, y as u32);
                let shade = t[(y * imgx + x) as usize];
                *pixel = image::Rgb([getR(shade), getG(shade), getB(shade)]);
            }
        }

        // Save image to file
        let file_name = format!("fractal-{}x{}-{}-{}.png", width, height, zoomFactor, iterations);
        imageBuffer.save(file_name).unwrap();
        
    } else {
        // Send data to root node
        root_process.gather_into(&data[..]);
    }
}

// Calculate R color component from iteration count
fn getR(x: u8) -> u8 {
    return match x {
        1=>66,
        2=>25,
        3=>9,
        4=>4,
        5=>0,
        6=>12,
        7=>24,
        8=>57,
        9=>134,
        10=>211,
        11=>241,
        12=>248,
        13=>255,
        14=>204,
        15=>153,
        16=>106,
        _=>0
    }
}

// Calculate G color component from iteration count
fn getG(x: u8) -> u8 {
    return match x {
        1=> 30,
        2=> 7,
        3=> 1,
        4=> 4,
        5=> 7,
        6=> 44,
        7=> 82,
        8=> 125,
        9=> 181,
        10=> 236,
        11=> 233,
        12=> 201,
        13=> 170,
        14=> 128,
        15=> 87,
        16=> 52,
        _=>0
    }
}

// Calculate B color component from iteration count
fn getB(x: u8) -> u8 {
    return match x {
        1=> 15,
        2=> 26,
        3=> 47,
        4=> 73,
        5=> 100,
        6=> 138,
        7=> 177,
        8=> 209,
        9=> 229,
        10=> 248,
        11=> 191,
        12=> 95,
        13=> 0,
        14=> 0,
        15=> 0,
        16=> 3,
        _=>0
    }
}