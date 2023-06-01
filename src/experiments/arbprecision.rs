//! An example of generating julia fractals.
use mpi::request::WaitGuard;
use mpi::traits::*;

use std::ops::Add;
use std::ops::Mul;
use std::ops::Neg;

use bigdecimal::BigDecimal;
use bigdecimal::FromPrimitive;
use std::time::{Duration, SystemTime};
use std::thread::sleep;
use std::time;

use mpi::datatype::PartitionMut;
use mpi::Count;
use mpi::datatype::{MutView, UserDatatype, View};



fn main() {
    
    let universe = mpi::initialize().unwrap();
    let world = universe.world();

    let size = world.size() as u32;
    let rank = world.rank();
    let root_process = world.process_at_rank(0);

    println!("{}x{}", size, rank);

    let height:u32 = 2240;
    let width:u32 = 2240;

    let imgx = width as u32;
    let imgy = (height / size) as u32;

    let count = world.size() as usize;

    let precision = 256;
    let iterations = 400;
    let zoomFactor: f64 = 20000.0;
    println!("{}x{}, {}, {}, {}", imgx, imgy, precision, iterations, zoomFactor);

    let scalexf = 1.0 / zoomFactor as f64;
    let scaleyf = 1.0 / zoomFactor as f64;
    let scalex = bigdecimal::BigDecimal::from_f64(scalexf).unwrap().with_prec(precision);
    let scaley = bigdecimal::BigDecimal::from_f64(scaleyf).unwrap().with_prec(precision);

    let cX = bigdecimal::BigDecimal::from_f64(-1.74995768370609350360221450607069970727110579726252077930242837820286008082972804887218672784431700831100544507655659531379747541999999995).unwrap().with_prec(precision);
    let cY =  bigdecimal::BigDecimal::from_f64(0.0000000000000000027879370656337940217829475379094436492708505450016308137904393065018938684976520216947747055).unwrap().with_prec(precision);

    let limit = bigdecimal::BigDecimal::from_u32(4).unwrap().with_prec(precision);

    let mut data: Vec<u8> = vec![0u8; (imgx * imgy) as usize];
    let mut y1: BigDecimal = bigdecimal::BigDecimal::from_f64((height as f64 / -2.0) * scaleyf).unwrap().add(&cY).with_prec(precision);

    y1 += bigdecimal::BigDecimal::from_f64(scaleyf * rank as f64 * imgy as f64).unwrap().with_prec(precision);

    let colors: Vec<u8> = vec![0u8; 16];

    fn getR(x: u8) -> u8 {
        return match x {
            0=>66,
            1=>25,
            2=>9,
            3=>4,
            4=>0,
            5=>12,
            6=>24,
            6=>57,
            6=>134,
            6=>211,
            6=>241,
            6=>248,
            6=>255,
            6=>204,
            6=>153,
            6=>106,
            _=>0
        }
    }

    fn getG(x: u8) -> u8 {
        return match x {
            0=> 30,
            1=> 7,
            2=> 1,
            3=> 4,
            4=> 7,
            5=> 44,
            6=> 82,
            6=> 125,
            6=> 181,
            6=> 236,
            6=> 233,
            6=> 201,
            6=> 170,
            6=> 128,
            6=> 87,
            6=> 52,
            _=>0
        }
    }

    fn getB(x: u8) -> u8 {
        return match x {
            0=> 15,
            1=> 26,
            2=> 47,
            3=> 73,
            4=> 100,
            5=> 138,
            6=> 177,
            6=> 209,
            6=> 229,
            6=> 248,
            6=> 191,
            6=> 95,
            6=> 0,
            6=> 0,
            6=> 0,
            6=> 3,
            _=>0
        }
    }

    let mut p: usize = 0;
    for y in 0..imgy{
        let mut x1 = bigdecimal::BigDecimal::from_f64((imgx as f64 / -2.0) * scalexf).unwrap().add(&cX).with_prec(precision);

        for x in 0..imgx {
            let mut zX1 = x1.clone();
            let mut zY1 = y1.clone();

            let mut zX2 = &zX1 * &zX1;
            let mut zY2 = &zY1 * &zY1;

            let mut i = 0;

            while i < iterations && &zX2 + &zY2 < limit {
                zX2 = &zX1 * &zX1;
                zY2 = &zY1 * &zY1;
                
                zY1 = (&zX1 * &zY1 + &zX1 * &zY1 + &y1).with_prec(precision);
                zX1 = (&zX2 - &zY2 + &x1).with_prec(precision);
                i += 1;
            }
            
            data[p] = (i % 16) as u8;
            p+=1;
            x1 += &scalex;
        }
        y1 += &scaley;

        println!("Rank {} {}/{}", rank, y, imgy);
    }
   
    if world.rank() == 0 {
        let mut imgbuf = image::ImageBuffer::new(width, height);


        let mut t = vec![0u8; (width * height) as usize];
        root_process.gather_into_root(&data[..], &mut t[..]);
            data = t;
        println!("Root gathered table: {}", &data.len());

            let mut pp:usize = 0;
            for y in 0..height {
                for x in 0..width {
                    let pixel = imgbuf.get_pixel_mut(x, y);
                    let shade = data[(y * imgx + x) as usize];
                    *pixel = image::Rgb([getR(shade), getG(shade), getB(shade)]);
                }
            }
        let fileName = format!("fractal-{}x{}-{}-{}.png", width, height, zoomFactor, iterations);
        imgbuf.save(fileName).unwrap();
        
    } else {

        root_process.gather_into(&data[..]);
    }

}