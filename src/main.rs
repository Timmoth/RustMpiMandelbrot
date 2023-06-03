use mpi::topology::SystemCommunicator;
use mpi::traits::*;

use std::process;
use std::thread::sleep;
use std::time;
use std::time::{Duration, SystemTime};

fn main() {
    // Config
    let height: usize = 8000;
    let width: usize = 8000;
    let iterations = 12000;
    let zoomFactor: f64 = 20000.0;

    // Open MPI
    let universe = mpi::initialize().unwrap();
    let world = universe.world();

    if (world.rank() == 0) {
        RunMaster(width, height, iterations, zoomFactor, world);
    } else {
        RunWorker(width, height, iterations, zoomFactor, world);
    }
}

fn RunMaster(
    width: usize,
    height: usize,
    iterations: usize,
    zoomFactor: f64,
    world: SystemCommunicator,
) {
    // Begin
    println!(
        "Start: {}, {}x{}, {}, {}",
        world.size(),
        width,
        height,
        iterations,
        zoomFactor
    );
    let size = world.size() as usize;
    let rank = world.rank() as usize;
    let mut linesOut: i32 = 0;
    let mut linesIn: i32 = 0;

    let mut imageBuffer = image::ImageBuffer::new(width as u32, height as u32);
    let mut data: Vec<u8> = vec![0u8; width];

    loop {
        let r = world.any_process().receive_into(&mut data);

        if (r.tag() != 0) {
            linesIn += 1;
            println!("{} Finished line: {}", r.source_rank(), r.tag());
            let y = r.tag() as u32;
            for x in 0..width {
                let pixel = imageBuffer.get_pixel_mut(x as u32, y);
                let shade = data[x];
                *pixel = image::Rgb([getR(shade), getG(shade), getB(shade)]);
            }
        }

        world.process_at_rank(r.source_rank()).send(&mut linesOut);
        linesOut += 1;

        if (linesIn >= (height - 1) as i32) {
            println!("Exit");
            // Save image to file
            let file_name = format!(
                "fractal-{}x{}-{}-{}.png",
                width, height, zoomFactor, iterations
            );
            imageBuffer.save(file_name).unwrap();
            world.abort(0);
            break;
        }
    }
}

fn RunWorker(
    width: usize,
    height: usize,
    iterations: usize,
    zoomFactor: f64,
    world: SystemCommunicator,
) {
    // Center point
    let cX = -1.74995768370609350360221450607069970727110579726252077930242837820286008082972804887218672784431700831100544507655659531379747541999999995;
    let cY =  0.0000000000000000027879370656337940217829475379094436492708505450016308137904393065018938684976520216947747055;

    let size = world.size() as usize;
    let rank = world.rank() as usize;
    let root_process = world.process_at_rank(0);

    let scale = 1.0 / zoomFactor;

    let mut lineIndex = 0;
    // Used to store pixel values

    let mut data: Vec<u8> = vec![0u8; width];

    root_process.send_with_tag(&data, lineIndex);
    root_process.receive_into(&mut lineIndex);

    loop {
        if (lineIndex >= height as i32) {
            println!("Exit node {}", rank);
            break;
        }

        let mut y = lineIndex;

        // The y component for the current pixel being calculated
        let mut y1: f64 = (height as f64 / -2.0) * scale + cY + scale * y as f64;
        println!("{} Working on line: {}", rank, y);

        // Calculate each pixel
        let timer = SystemTime::now();

        // The x component for the current pixel being calculated
        let mut x1: f64 = (width as f64 / -2.0) * scale + &cX;

        for x in 0..width {
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
            data[x] = (i % 16) as u8;

            // Move across to the next pixel
            x1 += &scale;
        }

        // Status update
        match timer.elapsed() {
            Ok(elapsed) => {
                println!("Rank{} {}/{} {}ms", rank, y, height, elapsed.as_millis());
            }
            Err(e) => {
                println!("Error: {e:?}");
            }
        }

        root_process.send_with_tag(&data, y);
        root_process.receive_into(&mut lineIndex);
    }
}

// Calculate R color component from iteration count
fn getR(x: u8) -> u8 {
    return match x {
        1 => 66,
        2 => 25,
        3 => 9,
        4 => 4,
        5 => 0,
        6 => 12,
        7 => 24,
        8 => 57,
        9 => 134,
        10 => 211,
        11 => 241,
        12 => 248,
        13 => 255,
        14 => 204,
        15 => 153,
        16 => 106,
        _ => 0,
    };
}

// Calculate G color component from iteration count
fn getG(x: u8) -> u8 {
    return match x {
        1 => 30,
        2 => 7,
        3 => 1,
        4 => 4,
        5 => 7,
        6 => 44,
        7 => 82,
        8 => 125,
        9 => 181,
        10 => 236,
        11 => 233,
        12 => 201,
        13 => 170,
        14 => 128,
        15 => 87,
        16 => 52,
        _ => 0,
    };
}

// Calculate B color component from iteration count
fn getB(x: u8) -> u8 {
    return match x {
        1 => 15,
        2 => 26,
        3 => 47,
        4 => 73,
        5 => 100,
        6 => 138,
        7 => 177,
        8 => 209,
        9 => 229,
        10 => 248,
        11 => 191,
        12 => 95,
        13 => 0,
        14 => 0,
        15 => 0,
        16 => 3,
        _ => 0,
    };
}
