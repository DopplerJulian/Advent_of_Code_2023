use std::ops::Div;

pub fn part_1(plan: &str) -> usize {
    let mut last_point = (0i32, 0i32);
    let mut area = 0i32;
    let mut perimeter = 0i32;

    for line in plan.lines() {
        let line_b = line.as_bytes();
        let amount = if line_b[3] == b' ' {
            (line_b[2]-48) as i32
        } else {
            ((line_b[2]-48)*10 + line_b[3] - 48) as i32
        };
        perimeter += amount;
        let old_point = last_point;

        match line_b[0] {
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
    let mut last_point = (0i64,0i64);
    let mut area = 0i64;
    let mut perimeter = 0i64;

    for line in plan.lines() {
        let line_b = line.as_bytes();
        let amount = i64::from_str_radix(&line[line.len()-7..line.len()-2], 16).unwrap();
        perimeter += amount;
        let old_point = last_point;

        match line_b[line.len()-2] {
            b'3' => {
                last_point.1 += amount;
            },
            b'1' => {
                last_point.1 -= amount;
            },
            b'2' => {
                last_point.0 -= amount;
            },
            b'0' => {
                last_point.0 += amount;
            },
            d => panic!("Direction not found: {d}"),
        }
        area += old_point.0*last_point.1 - old_point.1*last_point.0;
    }
    area = area.abs().div(2);

    (area + perimeter / 2 + 1) as usize
}