use std::{fs::File, io::BufReader, time::Instant};

use rand::{distributions::Uniform, prelude::Distribution, thread_rng};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
struct CoordPair {
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Pairs {
    pairs: Vec<CoordPair>,
}

fn haversine_degrees(c: CoordPair, r: f32) -> f32 {
    let dy = (c.y1 - c.y0).to_radians();
    let dx = (c.x1 - c.x0).to_radians();
    let y0 = c.y0.to_radians();
    let y1 = c.y1.to_radians();
    let root_term = (dy / 2.).sin().powi(2) + y0.cos() * y1.cos() * (dx / 2.).sin().powi(2);
    (2. * r) * (root_term.sqrt()).asin()
}

pub fn write_random() {
    let lat_dist = Uniform::new(-90., 90.);
    let long_dist = Uniform::new(-180., 180.);
    let mut rng = thread_rng();
    let pairs = (0..1_000_000)
        .map(|_| CoordPair {
            x0: long_dist.sample(&mut rng),
            x1: long_dist.sample(&mut rng),
            y0: lat_dist.sample(&mut rng),
            y1: lat_dist.sample(&mut rng),
        })
        .collect::<Vec<_>>();
    let pairs = Pairs { pairs };
    let out = File::create("input.json").expect("Failed to open file");
    serde_json::to_writer(out, &pairs).expect("Failed to serialize JSON");
}

fn main() {
    let now = Instant::now();
    let reader = BufReader::new(File::open("input.json").unwrap());
    let data: Pairs = simd_json::from_reader(reader).expect("Failed to deserialize JSON");
    let now2 = Instant::now();
    println!("Deserialize Time {:?}", now2 - now);
    let radius = 6371.;
    let sum = data
        .pairs
        .into_par_iter()
        .map(|c| haversine_degrees(c, radius))
        .sum::<f32>();

    let now3 = Instant::now();
    println!("Haversine Time {:?}", now3 - now2);
    println!("Sum {}", sum);
}
