use std::ops::{Add, Div};

pub fn part_1(plan: &str) -> usize {
    let mut  points: Vec<(i32,i32)> = Vec::with_capacity(16);
    let mut last_point = (0i32,0i32);
    points.push(last_point);
    let mut lines = plan.lines();
    let mut perimeter = 0i32;

    while let Some(line) = lines.next() {
        let mut it = line.split_ascii_whitespace();
        let dir = it.next().unwrap();
        let amount = it.next().unwrap().parse::<i32>().unwrap();
        perimeter += amount;

        match dir {
            "U" => {
                last_point.1 += amount;
            },
            "D" => {
                last_point.1 -= amount;
            },
            "L" => {
                last_point.0 -= amount;
            },
            "R" => {
                last_point.0 += amount;
            },
            d => panic!("Direction not found: {d}"),
        }
        points.push(last_point);
    }

    let a = points.windows(2).map(|pairs| {
        pairs[0].0*pairs[1].1 - pairs[0].1*pairs[1].0
    }).sum::<i32>().abs().div(2);

    (a + perimeter / 2 + 1) as usize
}