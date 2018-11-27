#![feature(int_to_from_bytes)]

use std::f32::consts;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let n: usize = 1024;
    let oname = "sin.bin";

    let inc: f32 = consts::FRAC_PI_2 / (n as f32);
    let bytes: Vec<[u8; 4]> = (0..n)
        .map(|n| (n as f32) * inc)
        .map(|f| f.sin())
        .map(|f| f.to_bits().to_le_bytes())
        .collect();

    let bytes: Vec<u8> = bytes.iter()
        .flat_map(|b| b.iter())
        .map(|n| *n)
        .collect();

    let mut out = File::create(oname).unwrap();
    out.write_all(&bytes).unwrap();

    println!("Hello, world!");
}
