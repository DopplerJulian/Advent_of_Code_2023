struct Race {
    duration: usize,
    record: usize,
} impl Race {
    fn new(duration: usize, record: usize) -> Self {
        Race{duration,record}
    }

    fn calc_margin(&self) -> usize {
        let inner = ((self.duration.pow(2)-4*self.record) as f64).sqrt();
        let range_start =   0.5*(self.duration as f64 - inner);
        let range_start = if range_start.fract() != 0.0 {range_start.ceil() as usize} else {range_start.ceil() as usize+1};
        let range_end =     (0.5*(self.duration as f64 + inner)).ceil() as usize;
        range_end - range_start
    }
}

#[allow(unused)]
pub fn part_1(races: &str) -> usize {
    let mut iter = races.lines();
    let durations: Vec<&str> = iter.next().unwrap()
        .split(':')
        .nth(1).unwrap()
        .trim_start()
        .split_ascii_whitespace().collect();

    iter.next().unwrap()
        .split(':')
        .nth(1).unwrap()
        .trim_start()
        .split_ascii_whitespace()
        .enumerate()
        .map(|(i, highscore)| Race::new(durations[i].parse::<usize>().unwrap(), highscore.parse::<usize>().unwrap()))
        .map(|r| r.calc_margin())
        .reduce(|acc, e| acc*e).unwrap()
}

#[allow(unused)]
pub fn part_2(races: &str) -> usize {
    let mut iter = races.lines();
    let duration = iter.next().unwrap()
        .split(':')
        .nth(1).unwrap()
        .trim_start()
        .replace(' ', "")
        .parse::<usize>().unwrap();

    let highscore = iter.next().unwrap()
        .split(':')
        .nth(1).unwrap()
        .trim_start()
        .replace(' ', "")
        .parse::<usize>().unwrap();

    Race::new(duration,highscore).calc_margin()
}