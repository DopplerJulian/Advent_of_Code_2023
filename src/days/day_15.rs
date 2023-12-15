use std::collections::VecDeque;
use rayon::prelude::*;

#[allow(unused)]
pub fn part_1(sequence: &str) -> usize {
    sequence.par_split(',').map(|to_hash| {
        // to_hash.chars().fold(0usize,|acc,e| ((acc+(e as usize))*17)%256 )
        hash(to_hash)
    }).sum::<usize>()
}

#[inline]
fn hash(to_hash: &str) -> usize {
    to_hash.chars().fold(0usize,|acc,e| ((acc+(e as usize))*17)%256 )
}

#[allow(unused)]
pub fn part_2(sequence: &str) -> usize {
    const _INIT: VecDeque<(&str, usize)> = VecDeque::new();
    let mut boxes: [VecDeque<(&str, usize)>; 256] = [_INIT; 256];

    sequence.split(',').for_each(|lens| {
        // let lens_split =
        let rm = lens.contains('-');
        let lens_split = lens.split_once(&['-', '=']).unwrap();
        let box_i = lens_split.0.chars().fold(0usize, |acc, e| ((acc + (e as usize)) * 17) % 256);

        let index = boxes[box_i].iter().position(|l| l.0 == lens_split.0);
        if rm {
            if let Some(ind) = index {
                boxes[box_i].remove(ind);
            }
        } else if let Some(ind) = index {
            boxes[box_i][ind].1 = lens_split.1.parse::<usize>().unwrap();
        } else {
            boxes[box_i].push_back((lens_split.0, lens_split.1.parse::<usize>().unwrap()))
        }
    });

    boxes.par_iter().enumerate()
        .filter(|(_, b)| !b.is_empty())
        .map(|(i, b)| {
            b.iter().enumerate().map(|(l_i, (_, f))| (i + 1) * (l_i + 1) * f).sum::<usize>()
        })
        .sum()
}