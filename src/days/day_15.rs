use std::collections::HashMap;
use indexmap::IndexMap;
use rayon::prelude::*;
use array_init::array_init;

#[allow(unused)]
pub fn part_1(sequence: &str) -> usize {
    sequence.par_split(',').map(|to_hash| {
        to_hash.chars().fold(0usize,|acc,e| ((acc+(e as usize))*17)%256 )
    }).sum::<usize>()
}

#[inline]
fn hash(to_hash: &str) -> usize {
    to_hash.chars().fold(0usize,|acc,e| ((acc+(e as usize))*17)%256 )
}

#[allow(unused)]
pub fn part_2(sequence: &str) -> usize {
    let mut boxes: [IndexMap<&str, usize>; 256] = array_init(|_| IndexMap::with_capacity(8));

    sequence.split(',').for_each(|lens| {
        let rm = lens.chars().last().unwrap()=='-';
        let label = if rm { &lens[..lens.len()-1] } else { &lens[..lens.len()-2] };
        let box_i = hash(label);

        if rm {
            boxes[box_i].shift_remove(label);
        } else {
            boxes[box_i].insert(label, lens[lens.len()-1..].parse::<usize>().unwrap());
        }
    });

    boxes.par_iter().enumerate()
        .filter(|(_, b)| !b.is_empty())
        .map(|(i, b)| {
            b.iter().enumerate().map(|(l_i, (_, f))| (i + 1) * (l_i + 1) * f).sum::<usize>()
        })
        .sum()
}

#[allow(unused)]
pub fn part_2_speedy(sequence: &str) -> usize {
    //  boxes[(box_instructions<label, (instruction_index,focal_length)>, was_inserted_to)]
    //  instruction index starts with 1, 0 means the item is removed
    let mut boxes: [(HashMap<&str, (usize,usize)>); 256] = array_init(|_| (HashMap::new()));
    // let mut hash_cache: HashMap<&str, usize> = HashMap::with_capacity(128);

    sequence.split(',').enumerate().for_each(|(instruction_i,lens)| {
        let rm = lens.chars().last().unwrap()=='-';
        let label = if rm { &lens[..lens.len()-1] } else { &lens[..lens.len()-2] };
        let box_i = hash(label);

        if rm {
            boxes[box_i].entry(label).and_modify(|v| v.0 = 0);
        } else {
            let focal = lens[lens.len()-1..].parse::<usize>().unwrap();
            let entry = boxes[box_i].entry(label).or_insert((instruction_i,focal));
            entry.1 = focal;
            if entry.0 == 0 {entry.0=instruction_i}
        }
    });

    boxes.into_par_iter().enumerate()
        .filter(|(_, b)| !b.is_empty())
        .map(|(box_index,(current_box))| {

        let mut lenses: Vec<(usize,usize)> = current_box.into_values()
            .filter(|l| l.0!=0)
            .map(|l|l)
            .collect();

        lenses.sort_unstable_by(|a,b| a.0.cmp(&b.0));

        lenses.iter().enumerate().map(|(lens_i, (_,focal))|{
            (box_index + 1) * (lens_i + 1) * focal
        }).sum::<usize>()
    }).sum()
}