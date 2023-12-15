use std::collections::HashMap;
use rayon::prelude::*;
use ndarray::*;

#[allow(unused)]
pub fn part_1_large(rocks: &str) -> usize {
    let mut ncols = 0usize;
    let mut nrows = 0usize;
    let mut round_rocks: Vec<bool> = Vec::new();
    let mut rock_mask: Vec<bool> = Vec::new();

    for n in rocks.lines() {
        if ncols == 0 {ncols = n.len()}
        let mut round: Vec<bool> = Vec::with_capacity(n.len());
        let mut mask: Vec<bool> = Vec::with_capacity(n.len());

        for c in n.chars(){
            round.push(c=='O');
            mask.push(c=='#');
        }
        round_rocks.extend(round);
        rock_mask.extend(mask);
        nrows += 1;
    }

    let mut round_rocks: Array2<bool> = Array2::from_shape_vec((nrows, ncols), round_rocks).unwrap();
    let rock_mask: Array2<bool> = Array2::from_shape_vec((nrows, ncols), rock_mask).unwrap();

    round_rocks.axis_iter_mut(Axis(1)).into_par_iter().enumerate()
        .for_each(|(col_i, mut col)| {
            let mut changed = true;
            while changed {
                changed = false;
                for row in 0..nrows-1{
                    if !rock_mask[[row,col_i]] && !col[row] && col[row+1] {
                        col[row] = !col[row];
                        col[row+1] = !col[row+1];
                        changed = true;
                    }
                }
            }
        });

    round_rocks.rows().into_iter().enumerate()
        .map(|(i,n)| (nrows-i) * n.iter().filter(|&b| *b).count() as usize)
        .sum()
}


// max cols is 128
#[allow(unused)]
pub fn part_1(rocks: &str) -> usize {
    let nrows = rocks.lines().count();
    let ncol = rocks.lines().next().unwrap().len();
    if ncol > 128 {panic!("More than 128 rows")}

    let mut round_rocks: Vec<u128> = Vec::with_capacity(nrows);
    let mut rock_mask: Vec<u128> = Vec::new();

    for n in rocks.lines() {
        let mut round: u128 = 0;
        let mut mask: u128 = 0;

        for (i,c) in n.char_indices() {
            round = round | (((c=='O') as u128) << i);
            mask = mask | (((c=='#') as u128) << i);
        }
        round_rocks.push(round);
        rock_mask.push(mask);
    }

    move_all_rocks(&mut round_rocks,&rock_mask);

    round_rocks.into_iter().enumerate()
        .map(|(i,n)| (nrows-i) * n.count_ones() as usize)
        .sum()
}

// (to_new, from_new)
// ((to or from) and not mask, (from and to) or (from and mask)
fn move_all_rocks(round_rocks: &mut Vec<u128>, rock_mask: &Vec<u128>){
    for boundary in 0..rock_mask.len()-1{
        for i in 0..round_rocks.len()-1-boundary{
            (round_rocks[i],round_rocks[i+1]) = ((round_rocks[i] | round_rocks[i+1]) & !rock_mask[i],
                                                 (round_rocks[i+1] & round_rocks[i]) | (round_rocks[i+1] & rock_mask[i]));
        }
    }
}

fn move_all_rocks_down(round_rocks: &mut Vec<u128>, rock_mask: &Vec<u128>){
    for boundary in 0..rock_mask.len()-1{
        for i in (0+boundary..round_rocks.len()-1).rev(){
            (round_rocks[i+1],round_rocks[i]) = ((round_rocks[i+1] | round_rocks[i]) & !rock_mask[i+1],
                                                 (round_rocks[i] & round_rocks[i+1]) | (round_rocks[i] & rock_mask[i+1]));
        }
    }
}

fn move_all_rocks_left(round_rocks: &mut Vec<u128>, rock_mask: &Vec<u128>, ncols: usize) {
    round_rocks.par_iter_mut().enumerate()
        .for_each(|(row,num)| {
            let mut changed = true;
            while changed {
                changed = false;
                for col in 0..ncols-1{
                    if (rock_mask[row] >> col & 1)==0 &&
                        (*num >> col & 1)==0 &&
                        (*num >> (col+1) & 1)==1 {
                        *num = *num ^ (1 << col);
                        *num = *num ^ (1 << (col+1));
                        changed = true;
                    }
                }
            }
        })
}

fn move_all_rocks_right(round_rocks: &mut Vec<u128>, rock_mask: &Vec<u128>, ncols: usize) {
    round_rocks.par_iter_mut().enumerate()
        .for_each(|(row,num)| {
            let mut changed = true;
            while changed {
                changed = false;
                for col in (0..ncols-1).rev(){
                    if (rock_mask[row] >> (col+1) & 1)==0 &&
                        (*num >> col & 1)==1 &&
                        (*num >> (col+1) & 1)==0 {
                        *num = *num ^ (1 << col);
                        *num = *num ^ (1 << (col+1));
                        changed = true;
                    }
                }
            }
        })
}


#[allow(unused)]
pub fn part_2(rocks: &str) -> usize {
    let nrows = rocks.lines().count();
    let ncol = rocks.lines().next().unwrap().len();
    if ncol > 128 {panic!("More than 128 rows")}

    let mut round_rocks: Vec<u128> = Vec::with_capacity(nrows);
    let mut rock_mask: Vec<u128> = Vec::new();

    for n in rocks.lines() {
        let mut round: u128 = 0;
        let mut mask: u128 = 0;

        for (i,c) in n.char_indices() {
            round = round | (((c=='O') as u128) << i);
            mask = mask | (((c=='#') as u128) << i);
        }
        round_rocks.push(round);
        rock_mask.push(mask);
    }
    let mut cycle_trail: HashMap<Vec<u128>, usize> = HashMap::new();
    let mut count: usize = 0;

    loop {
        count+=1;
        move_all_rocks(&mut round_rocks,&rock_mask);
        move_all_rocks_left(&mut round_rocks,&rock_mask, ncol);
        move_all_rocks_down(&mut round_rocks,&rock_mask);
        move_all_rocks_right(&mut round_rocks,&rock_mask, ncol);

        if let Some(val) = cycle_trail.get(&round_rocks) {
            let cycle_length = count-val;
            println!("loop: {cycle_length}");
            println!("loop start: {val}");
            println!("loop repeat: {count}");
            let target_state: usize = ((1_000_000_000-val) % cycle_length) + val;
            return cycle_trail.iter().find(|(_,&v)| v==target_state).unwrap().0
                .iter().enumerate()
                .map(|(i,n)| (nrows-i) * n.count_ones() as usize)
                .sum()
        } else {
            cycle_trail.insert(round_rocks.clone(),count);
        }
    }
}