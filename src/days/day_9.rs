use rayon::prelude::*;

#[allow(unused)]
pub fn part_1(history: &str) -> isize {
    history.par_lines()
        .map(|l| l.trim().split_ascii_whitespace().map(|n|n.parse::<isize>().unwrap()).collect::<Vec<isize>>()) // map &str to Vec<isize>
        .map(|mut h| {
            let mut result: isize = 0;
            'outer: loop {
                let mut it = h.iter_mut().rev();
                let mut last = *it.next().unwrap();
                let mut all_zero = true;
                result += last;

                it.for_each(|n| {
                    let next_last = *n;
                    *n = last - *n;
                    last = next_last;
                    all_zero = all_zero && *n == 0;
                });
                if all_zero {break 'outer}
                h.pop();
            }
            result
        }).sum()
}

#[allow(unused)]
pub fn part_2(history: &str) -> isize {
    history.par_lines()
        .map(|l| l.trim().split_ascii_whitespace().map(|n|n.parse::<isize>().unwrap()).collect::<Vec<isize>>()) // map &str to Vec<isize>
        .map(|mut h| {
            let mut result: Vec<isize> = Vec::new();
            h.reverse();
            'outer: loop {
                let mut it = h.iter_mut().rev();
                let mut first = *it.next().unwrap();
                let mut all_zero = true;
                result.push(first);

                it.for_each(|n| {
                    let next_first = *n;
                    *n -= first;
                    first = next_first;
                    all_zero = all_zero && *n == 0;
                });
                if all_zero {break 'outer}
                h.pop();
            }
            result.iter().rev().fold(0, |acc, e| e-acc)
        }).sum()
}