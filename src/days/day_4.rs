use rayon::prelude::*;

#[allow(unused)]
pub fn part_1(cards: &str) -> u32 {
    let first = cards.lines().next().unwrap();
    let index_c = first.chars().position(|c| c == ':').unwrap();
    let index_v = first.chars().position(|c| c == '|').unwrap();

    cards.par_lines()
        .map(|line| calc_points(line, index_c, index_v))
        .sum()
}

fn calc_points(card: &str, index_c: usize, index_v: usize) -> u32 {
    let mut numbers_w: Vec<&str> = Vec::with_capacity(8);

    for i in 0..((index_v-index_c-2)/3) {
        let ind = index_c + 2 + i*3;
        numbers_w.push(&card[ind..ind+2]);
    }

    let result = (0..(card.len()-index_v as usize-1)/3).filter(|i| {
        let ind = index_v + 2 + i*3;
        let num: &str = &card[ind..ind+2];

        numbers_w.contains(&num)
    }).count() as u32;

    if result == 0 {
        result
    } else {
        2u32.pow(result - 1)
    }
}

// #[allow(unused)]
// pub fn part_2(cards: &str) -> u32 {
//     let first = cards.lines().next().unwrap();
//     let index_c = first.chars().position(|c| c == ':').unwrap();
//     let index_v = first.chars().position(|c| c == '|').unwrap();
//     // (count, time to live)
//     let mut history: Vec<(u32, u32)> = vec![];
//
//     cards.lines()
//         .map(|line: &str| {
//             let matches = calc_count(line, index_c, index_v);
//             let count_sum: u32 = history.iter()
//                 .map(|h| h.0)
//                 .sum();
//
//             history.retain_mut(|h|{ if h.1 > 0 {
//                 h.1 -= 1;
//                 true
//             } else { false }
//             });
//             if matches > 0 {
//                 history.push((count_sum+1, matches-1))
//             }
//             count_sum+1
//         })
//         .sum()
// }

fn calc_count(card: &str, index_c: usize, index_v: usize) -> u32 {
    let mut numbers_w: Vec<&str> = Vec::with_capacity(8);

    for i in 0..((index_v-index_c-2)/3) {
        let ind = index_c + 2 + i*3;
        numbers_w.push(&card[ind..ind+2]);
    }

    let result = (0..(card.len()-index_v as usize-1)/3).filter(|i| {
        let ind = index_v + 2 + i*3;
        let num: &str = &card[ind..ind+2];

        numbers_w.contains(&num)
    }).count() as u32;

    result
}

#[allow(unused)]
pub fn part_2(cards: &str) -> u32 {
    let first = cards.lines().next().unwrap();
    let index_c = first.chars().position(|c| c == ':').unwrap();
    let index_v = first.chars().position(|c| c == '|').unwrap();

    let card_matches: Vec<u32> = cards.par_lines()
        .map(|line| calc_count(line, index_c, index_v))
        .collect();

    let mut history: Vec<(u32, u32)> = Vec::with_capacity(8);

    card_matches.iter()
        .map(|value| {
            let matches = *value;
            let count_sum: u32 = history.iter()
                .map(|h| h.0)
                .sum();

            history.retain_mut(|h|{ if h.1 > 0 {
                h.1 -= 1;
                true
            } else { false }
            });
            if matches > 0 {
                history.push((count_sum+1, matches-1))
            }
            count_sum+1
        })
        .sum()
}