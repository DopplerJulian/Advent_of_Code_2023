#[allow(unused)]
pub fn part_1(sequence: &str) -> usize {
    sequence.split(',').map(|to_hash| {
        to_hash.chars().fold(0usize,|acc,e| ((acc+(e as usize))*17)%256 )
    }).sum::<usize>()
}