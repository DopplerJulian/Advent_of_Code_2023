use std::collections::BTreeMap;
use rayon::prelude::*;

#[derive(Eq, PartialEq, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[allow(unused)]
pub fn calculate(galaxies: &str, expansion_size: usize) -> usize {
    let mut points: Vec<Point> = galaxies.lines().enumerate().map(|(y, line)| {
        line.char_indices().filter(|(_, c)| *c == '#')
            .map(|(x, _)| Point { x, y })
            .collect::<Vec<_>>()
    }).flatten().collect();

    let mut grouped_x: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    let mut grouped_y: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    points.iter().enumerate().for_each(|(i, p)| {
        grouped_x.entry(p.x).or_insert(vec![]).push(i);
        grouped_y.entry(p.y).or_insert(vec![]).push(i);
    });

    grouped_x.iter().enumerate().for_each(|(i, (x, vec))| {
        vec.iter().for_each(|ind| points[*ind].x += ((x - i) * (expansion_size - 1)))
    });

    grouped_y.iter().enumerate().for_each(|(i, (y, vec))| {
        vec.iter().for_each(|ind| points[*ind].y += ((y - i) * (expansion_size - 1)))
    });

    points.par_iter().enumerate().map(|(i, p)| {
        points.par_iter().skip(i+1)
            .map(|other_p| p.x.abs_diff(other_p.x) + p.y.abs_diff(other_p.y))
            .sum::<usize>()
    }).sum()
}