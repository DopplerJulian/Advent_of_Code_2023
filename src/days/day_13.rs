use ndarray::{Array, Array2};
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
        let rows: Vec<_> = pattern.rows().into_iter().map(|r| r.iter().map(|b|b.clone()).collect::<Vec<_>>()).collect();
        let mut is_row = false;

        for i in 1..rows.len() {
            if rows[..i].iter().rev().zip(&rows[i..])
                .all(|(from, to)| to.eq(from)) {
                result += i;
                is_row = true;
                break
            }
        }
        if is_row {continue}

        let col: Vec<_> = pattern.columns().into_iter().map(|r| r.iter().map(|b|b.clone()).collect::<Vec<_>>()).collect();

        for i in 1..col.len() {
            if col[..i].iter().rev().zip(&col[i..])
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
        let rows: Vec<_> = pattern.rows().into_iter().map(|r| r.iter().map(|b|b.clone()).collect::<Vec<_>>()).collect();
        let mut is_row = false;

        'outer:for i in 1..rows.len() {
            for cmp_index in 0..i.min(*&rows[i..].len()){
                if rows[..i].iter().rev().zip(&rows[i..])
                    .enumerate()
                    .all(|(cmp_i, (from, to))| if cmp_i==cmp_index {eq_any_with_flip(from,to)} else {to.eq(from)} ) {
                    result += i;
                    is_row = true;
                    // println!("Row: {i}, Flipped Row: {cmp_index}");
                    break 'outer;
                }
            }
        }
        if is_row {continue}

        let col: Vec<_> = pattern.columns().into_iter().map(|r| r.iter().map(|b|b.clone()).collect::<Vec<_>>()).collect();

        'outer:for i in 1..col.len() {
            for cmp_index in 0..i.min(*&col[i..].len()){
                if col[..i].iter().rev().zip(&col[i..])
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
    (0..a.len()).any(|flip_i| {
        (0..a.len()).all(|i| if flip_i==i { a[i]!=b[i] } else { a[i] == b[i] })
    })
}