use std::ops::Div;
// use rayon::prelude::*;

pub fn part_1(plan: &str) -> usize {
    let mut last_point = (0i32,0i32);
    let mut area = 0i32;
    let mut perimeter = 0i32;

    for line in plan.lines() {
        let mut it = line.bytes();
        let dir = it.next().unwrap();
        let tens = it.nth(1).unwrap();
        let ones = it.next().unwrap();
        let amount = if ones == b' ' {
            (tens-48) as i32
        } else {
            ((tens-48)*10 + ones - 48) as i32
        };
        perimeter += amount;
        let old_point = last_point;

        match dir {
            b'U' => {
                last_point.1 += amount;
            },
            b'D' => {
                last_point.1 -= amount;
            },
            b'L' => {
                last_point.0 -= amount;
            },
            b'R' => {
                last_point.0 += amount;
            },
            d => panic!("Direction not found: {d}"),
        }
        area += old_point.0*last_point.1 - old_point.1*last_point.0;
    }
    area = area.abs().div(2);

    (area + perimeter / 2 + 1) as usize
}


pub fn part_2(plan: &str) -> usize {
    let mut  points: Vec<(i64,i64)> = Vec::with_capacity(16);
    let mut last_point = (0i64,0i64);
    points.push(last_point);
    let mut lines = plan.lines();
    let mut perimeter = 0i64;

    while let Some(line) = lines.next() {
        let amount = i64::from_str_radix(&line[line.len()-7..line.len()-2], 16).unwrap();
        let dir = &line[line.len()-2..=line.len()-2];
        perimeter += amount;

        match dir {
            "3" => {
                last_point.1 += amount;
            },
            "1" => {
                last_point.1 -= amount;
            },
            "2" => {
                last_point.0 -= amount;
            },
            "0" => {
                last_point.0 += amount;
            },
            d => panic!("Direction not found: {d}"),
        }
        points.push(last_point);
    }

    let a = points.windows(2).map(|pairs| {
        pairs[0].0*pairs[1].1 - pairs[0].1*pairs[1].0
    }).sum::<i64>().abs().div(2);

    (a + perimeter / 2 + 1) as usize
}