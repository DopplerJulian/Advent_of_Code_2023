use rayon::prelude::*;

struct Pull{
    red: u32,
    green: u32,
    blue: u32,
}

fn max_cubes_of_game(pulls: &str) -> Pull {
    let mut max = Pull{red: 0, green: 0, blue: 0};
    for cube in pulls.split(&[';', ',']) {
        let cube: Vec<&str> = cube[1..].split(' ').collect();
        let amount: u32 = cube[0].parse().unwrap();

        match cube[1].chars().next().unwrap() {
            'r' => max.red = amount.max(max.red),
            'g' => max.green = amount.max(max.green),
            'b' => max.blue = amount.max(max.blue),
            _ => panic!(),
        }
    }
    max
}

#[allow(unused)]
pub fn part_1(document: Vec<String>) -> u64 {
    document.par_iter()
        .filter_map(|l|{
            if l.is_empty(){return None}
            let game: Vec<&str> = l.trim().split(':').collect();
            let id = game[0].trim().split(' ').last().unwrap().parse::<u64>().unwrap();
            let max_pull = max_cubes_of_game(game[1]);
            if max_pull.red <= 12 && max_pull.green <= 13 && max_pull.blue <= 14 {
                Some(id)
            } else {
                None
            }
        }).sum::<u64>()
}

#[allow(unused)]
pub fn part_2(document: Vec<&str>) -> u64 {
    document.par_iter()
        .map(|l|{
            if l.is_empty(){return 0}
            let min_pull = max_cubes_of_game(l.trim().split(':').last().unwrap());

            (min_pull.red * min_pull.green * min_pull.blue) as u64
        }).sum::<u64>()
}