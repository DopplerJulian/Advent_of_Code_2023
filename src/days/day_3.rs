use rayon::prelude::*;

#[allow(unused)]
pub fn part_1(schematic: Vec<&str>) -> u32 {
    let mut symbol: Vec<(usize, usize)> = Vec::new();

    for (y, &line) in schematic.iter().enumerate() {
        for (x, char) in line.trim().char_indices() {
            if !char.is_numeric() && char != '.' {
                symbol.push((x, y));
            }
        }
    }
    let symbol = symbol;

    schematic.par_iter()
        .enumerate()
        .map(|(y, &line)| {
        if let Some(mut start) = line.chars().position(|c| c.is_numeric()) {
            let mut last: usize = start;
            let mut sum: u32 = 0;
            for (x, _) in line.char_indices().filter(|c| c.1.is_numeric()) {
                if last < x.saturating_sub(1) {
                    if next_to_symbols(start, last, y, symbol.as_slice()) {
                        sum += line[start..=last].parse::<u32>().unwrap();
                    }
                    start = x;
                }
                last = x;
            }

            if next_to_symbols(start, last, y, symbol.as_slice()) {
                sum += line[start..=last].parse::<u32>().unwrap();
            }
            sum
        } else { 0 }
    }).sum()
}

fn next_to_symbols(start: usize, end: usize, y: usize, symbols: &[(usize, usize)]) -> bool {
    for sym in symbols.iter() {
        if y <= sym.1+1 && y+1 >= sym.1 &&
            end+1 >= sym.0 && start <= sym.0+1 {
                return true
        }
    }
    false
}


#[allow(unused)]
pub fn part_2(schematic: Vec<&str>) -> usize {
    let mut symbol: Vec<(usize, usize)> = Vec::new();

    schematic.iter()
        .enumerate()
        .for_each(|(y, &line)|{
            line.char_indices().filter(|c|c.1 == '*')
                .for_each(|(x,_)| symbol.push((x,y)));
        });
    let mut count: Vec<(usize,usize)> = vec![(0,1);symbol.len()];

    schematic.iter()
        .enumerate().for_each(|(y, &line)| {
        if let Some(mut start) = line.chars().position(|c| c.is_numeric()) {
            let mut last: usize = start;
            for (x, _) in line.char_indices().filter(|c| c.1.is_numeric()) {
                if last < x.saturating_sub(1) {
                    if let Some(pos) = symbol.iter().position(|&sym| y <= sym.1 + 1 && y + 1 >= sym.1 &&
                        last + 1 >= sym.0 && start <= sym.0 + 1) {
                        count[pos] = (count[pos].0+1,count[pos].1*line[start..=last].parse::<u32>().unwrap() as usize);
                    }
                    start = x;
                }
                last = x;
            }

            if let Some(pos) = symbol.iter().position(|&sym| y <= sym.1 + 1 && y + 1 >= sym.1 &&
                last + 1 >= sym.0 && start <= sym.0 + 1) {
                count[pos] = (count[pos].0+1,count[pos].1*line[start..=last].parse::<u32>().unwrap() as usize);
            }
        }
    });

    count.iter()
        .filter_map(|s| if s.0==2 {Some(s.1)} else { None })
        .sum()
}