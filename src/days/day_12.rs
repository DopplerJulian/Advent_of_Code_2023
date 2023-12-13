use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

#[allow(unused)]
pub fn part_1(springs: &str) -> usize {
    let springs:Vec<&str> = springs.lines().collect();

    springs.par_iter().enumerate().map(|(_line_index,&line)| {
        let split = line.split_once(' ').unwrap();
        let mut record: String = split.0.to_string();
        let mut combination: Vec<_> = split.1.split(',').map(|n|n.parse::<usize>().unwrap()).collect();

        let question_marks: Vec<_> = record.char_indices()
            .filter(|(_,c)| *c=='?')
            .map(|(i,_)| i).collect();
        let amount_hashtag = combination.iter().sum::<usize>()-record.chars().filter(|c| *c=='#').count();
        let mut permutation: u128 = 2u128.pow(amount_hashtag as u32)-1;
        let mut counter: usize = 0;
        if !record.is_ascii() {panic!("record not ascii")}

        loop {
            if (permutation >> (question_marks.len()+1) & 1) == 1 {break}

            let mut record_ref: &mut [u8] = unsafe { record.as_bytes_mut() };
            for (nth_q, i) in question_marks.iter().enumerate(){
                record_ref[*i] = if (permutation >> nth_q & 1) == 1 {'#' as u8} else {'.' as u8}
            }
            let local_combination: Vec<_> = record.split('.').filter(|&s|!s.is_empty()).collect();
            if local_combination.len() == combination.len() && local_combination.iter().enumerate().all(|(i,&r)| combination[i]==r.len()){
                counter += 1;
            }
            permutation = bit_twiddle_permute(permutation);
        }
        counter
    }).sum()
}

#[allow(unused)]
pub fn part_2(springs: &str) -> usize {
    let springs:Vec<&str> = springs.lines().collect();
    let done: AtomicUsize = AtomicUsize::new(1);

    springs.par_iter().enumerate().map(|(line_index,&line)| {
        println!("Starting Line: {}", line_index+1);
        let split = line.split_once(' ').unwrap();
        let mut record: String = split.0.to_string();
        record.push('?');
        record = record.repeat(4);
        record.push_str(split.0);
        let mut combination: Vec<_> = split.1.split(',').map(|n|n.parse::<usize>().unwrap()).collect();
        let c_clone = combination.clone();
        for _ in 0..4 {
            combination.extend(c_clone.clone())
        }

        let question_marks: Vec<_> = record.char_indices()
            .filter(|(_,c)| *c=='?')
            .map(|(i,_)| i).collect();
        let amount_hashtag = combination.iter().sum::<usize>()-record.chars().filter(|c| *c=='#').count();
        let mut permutation: u128 = 2u128.pow(amount_hashtag as u32)-1;
        let mut counter: usize = 0;
        if !record.is_ascii() {panic!("record not ascii")}
        // println!("{:?}", combination);
        // println!("{}", record);

        loop {
            if (permutation >> (question_marks.len()+1) & 1) == 1 {break}

            let mut record_ref: &mut [u8] = unsafe { record.as_bytes_mut() };
            for (nth_q, i) in question_marks.iter().enumerate(){
                record_ref[*i] = if (permutation >> nth_q & 1) == 1 {'#' as u8} else {'.' as u8}
            }
            let local_combination: Vec<_> = record.split('.').filter(|&s|!s.is_empty()).collect();
            if local_combination.len() == combination.len() && local_combination.iter().enumerate().all(|(i,&r)| combination[i]==r.len()){
                counter += 1;
            }
            permutation = bit_twiddle_permute(permutation);
        }
        println!("Lines done: {}", done.fetch_add(1,Ordering::Relaxed));
        counter
    }).sum()
}

#[inline]
#[allow(arithmetic_overflow)]
pub fn bit_twiddle_permute(v: u128) -> u128 {
    let t = v | (v.wrapping_sub(1));
    (t + 1) | (((!t & (!t).wrapping_neg()).wrapping_sub(1)) >> (v.trailing_zeros() + 1))
}

