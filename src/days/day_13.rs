use ndarray::{Array, Array2};
use rayon::prelude::*;
#[allow(unused)]
pub fn part_1(pat: &str) -> usize {
    let mut patterns: Vec<_> = pat.lines().collect();

    let patterns: Vec<Vec<Vec<bool>>> = patterns.split(|&l| l.is_empty())
        .map(|chunk| chunk.iter().map(|&l| l.chars().map(|c| c=='#').collect()).collect())
        .collect();

    let mut pats: Vec<Array2<bool>> = patterns.iter()
        .map(|pattern| Array::from_elem((pattern.first().unwrap().len(),pattern.len()),false))
        .collect();

    for (pat_i,pat) in patterns.iter().enumerate() {
        for (y_i, lines) in pat.iter().enumerate() {
            for (x_i, b) in lines.iter().enumerate() {
                if patterns[pat_i][y_i][x_i] {
                    *pats.get_mut(pat_i).unwrap().get_mut((x_i,y_i)).unwrap() = true;
                }
            }
        }
    }

    let mut result: usize = 0;
    for pattern in pats.iter(){
        let cols: Vec<_> = pattern.rows().into_iter().map(|r| r.iter().map(|b|b.clone()).collect::<Vec<_>>()).collect();
        let mut is_col = false;

        for i in 1..cols.len() {
            if cols[..i].iter().rev().zip(&cols[i..])
                .all(|(from, to)| to.eq(from)) {
                result += i;
                is_col = true;
                break
            }
        }
        if is_col {continue}

        let rows: Vec<_> = pattern.columns().into_iter().map(|r| r.iter().map(|b|b.clone()).collect::<Vec<_>>()).collect();

        for i in 1..rows.len() {
            if rows[..i].iter().rev().zip(&rows[i..])
                .all(|(from, to)| to.eq(from)) {
                result += i*100;
                break
            }
        }
    }
    result
}


#[allow(unused)]
pub fn part_2(pat: &str) -> usize {
    let mut patterns: Vec<_> = pat.par_lines().collect();

    let patterns: Vec<Vec<Vec<bool>>> = patterns.split(|&l| l.is_empty())
        .map(|chunk| chunk.iter().map(|&l| l.chars().map(|c| c=='#').collect()).collect())
        .collect();

    let mut pats: Vec<Array2<bool>> = patterns.iter()
        .map(|pattern| Array::from_elem((pattern.first().unwrap().len(),pattern.len()),false))
        .collect();

    for (pat_i,pat) in patterns.iter().enumerate() {
        for (y_i, lines) in pat.iter().enumerate() {
            for (x_i, b) in lines.iter().enumerate() {
                if patterns[pat_i][y_i][x_i] {
                    *pats.get_mut(pat_i).unwrap().get_mut((x_i,y_i)).unwrap() = true;
                }
            }
        }
    }

    let mut result: usize = 0;
    for pattern in pats.iter(){
        let cols: Vec<_> = pattern.rows().into_iter().map(|r| r.iter().map(|b|b.clone()).collect::<Vec<_>>()).collect();
        let mut is_col = false;

        'outer:for i in 1..cols.len() {
            for cmp_index in 0..i.min(*&cols[i..].len()){
                if cols[..i].iter().rev().zip(&cols[i..])
                    .enumerate()
                    .all(|(cmp_i, (from, to))| if cmp_i==cmp_index {eq_any_with_flip(from,to)} else {to.eq(from)} ) {
                    result += i;
                    is_col = true;
                    // println!("Row: {i}, Flipped Row: {cmp_index}");
                    break 'outer;
                }
            }
        }
        if is_col {continue}

        let rows: Vec<_> = pattern.columns().into_iter().map(|r| r.iter().map(|b|b.clone()).collect::<Vec<_>>()).collect();

        'outer:for i in 1..rows.len() {
            for cmp_index in 0..i.min(*&rows[i..].len()){
                if rows[..i].iter().rev().zip(&rows[i..])
                    .enumerate()
                    .all(|(cmp_i, (from, to))| if cmp_i==cmp_index {eq_any_with_flip(from,to)} else {to.eq(from)} ) {
                    result += i*100;
                    // println!("Col: {i}, Flipped Col: {cmp_index}");
                    break 'outer;
                }
            }
        }
    }
    result
}

fn eq_any_with_flip(a: &Vec<bool>, b: &Vec<bool>) -> bool {
    let mut misses: usize = 0;
    for i in 0..a.len() {
        if a[i]!=b[i]{
            misses += 1;
            if misses>1 {return false}
        }
    }
    misses==1
}