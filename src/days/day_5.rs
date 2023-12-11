use std::time::Instant;
use rayon::prelude::*;

#[derive(Eq,PartialEq,Clone)]
struct Map {
    from: u64,
    to: u64,
    range: u64,
}

struct Mapping {
    maps: Vec<Map>
} impl Mapping {
    fn from_lines(lines: &[&str]) -> Self {
        let maps: Vec<Map> = lines.iter()
            .map(|&line| {
                let mut i = line.split_ascii_whitespace();
                let to = i.next().unwrap().parse::<u64>().unwrap();
                let from = i.next().unwrap().parse::<u64>().unwrap();
                let range = i.next().unwrap().parse::<u64>().unwrap();
                Map{from,to,range}
            })
            .collect();

        Mapping{maps}
    }
    
    fn map_vec(&self, seeds: &mut Vec<u64>) {
        seeds.par_iter_mut()
            .for_each(|seed|{
            if let Some(m) = self.maps.iter().find(|&m| m.from<=*seed && m.from+m.range>*seed) {
                *seed = (*seed + m.to) - m.from;
            }
        })
    }

    // fn map_vec_retain(&self, seeds: &mut Vec<u64>, ) {
    //     seeds.par_iter_mut()
    //         .for_each(|seed|{
    //             if let Some(m) = self.maps.iter().find(|&m| m.from<=*seed && m.from+m.range>*seed) {
    //                 *seed = (*seed + m.to) - m.from;
    //             }
    //         })
    // }
}

#[allow(unused)]
pub fn part_1(almanac: &str) -> u64 {
    let mut iter = almanac.lines();
    let mut seeds: Vec<u64> = iter.next().unwrap()[7..].split_ascii_whitespace()
        .map(|n| n.parse::<u64>().unwrap()).collect();
    _ = iter.next();

    let alm: Vec<&str> = iter.collect();
    let mappings: Vec<Mapping> = alm.split(|&l| l.is_empty())
        .map(|line_slice| Mapping::from_lines(&line_slice[1..]))
        .collect();

    for m in mappings.iter(){
        m.map_vec(&mut seeds)
    }

    *seeds.iter().min().unwrap()
}


#[allow(unused)]
pub fn part_2(almanac: &str) -> u64 {
    let mut iter = almanac.lines();
    let chunks: Vec<&str> = iter.next().unwrap()[7..].split_ascii_whitespace().collect();
    _ = iter.next();

    let start = Instant::now();

    let alm: Vec<&str> = iter.collect();
    let mappings: Vec<Mapping> = alm.split(|&l| l.is_empty())
        .map(|line_slice| Mapping::from_lines(&line_slice[1..]))
        .collect();

    println!("Time for generating mappings: {}ms", start.elapsed().as_millis());
    let start = Instant::now();

    let above_range = mappings.iter()
        .map(|maps| maps.maps.iter().map(|m| m.from+m.range).max().unwrap())
        .max().unwrap();
    let mut seeds: Vec<u64> = chunks.chunks_exact(2)
        .map(|n| (n[0].parse::<u64>().unwrap()..
            above_range.min(n[0].parse::<u64>().unwrap()+n[1].parse::<u64>().unwrap())).collect::<Vec<u64>>())
        .par_bridge()
        .flatten()
        .collect();

    println!("Time for generating seeds: {}ms", start.elapsed().as_millis());
    let start = Instant::now();

    for m in mappings.iter(){
        m.map_vec(&mut seeds)
    }

    println!("Time for calculating seed mappings: {}ms", start.elapsed().as_millis());
    let start = Instant::now();

    let result = *seeds.par_iter().min().unwrap();
    println!("Time for calculating min seed mappings: {}ms", start.elapsed().as_millis());
    result
}