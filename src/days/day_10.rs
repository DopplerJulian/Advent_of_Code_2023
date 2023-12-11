#[derive(Eq, PartialEq, Clone)]
enum Direction{
    UP,
    DOWN,
    LEFT,
    RIGHT,
} impl Direction {
    fn coord_dir(&self,x: usize, y: usize) -> (usize,usize) {
        match self {
            Direction::UP => (x,y-1),
            Direction::DOWN => (x,y+1),
            Direction::LEFT => (x-1,y),
            Direction::RIGHT => (x+1,y),
        }
    }
    fn opposite(&self)->Self{
        match self {
            Direction::UP => Direction::DOWN,
            Direction::DOWN => Direction::UP,
            Direction::LEFT => Direction::RIGHT,
            Direction::RIGHT => Direction::LEFT,
        }
    }
}
fn get_dirs(pipe: char) -> Option<[Direction;2]> {
    match pipe {
        '|' => Some([Direction::UP, Direction::DOWN]),
        '-' => Some([Direction::LEFT, Direction::RIGHT]),
        'L' => Some([Direction::UP, Direction::RIGHT]),
        'J' => Some([Direction::UP, Direction::LEFT]),
        '7' => Some([Direction::DOWN, Direction::LEFT]),
        'F' => Some([Direction::DOWN, Direction::RIGHT]),
        _ => None,
    }
}
// (next_position, last_direction)
fn start_dir(grid: &Vec<&str>, start_pos: &(usize,usize)) -> ((usize,usize),Direction) {
    for dir in [Direction::UP,Direction::DOWN,Direction::LEFT,Direction::RIGHT] {
        if (dir == Direction::UP && start_pos.1==0) || (dir == Direction::LEFT && start_pos.0==0) ||
            (dir == Direction::DOWN && start_pos.1==grid.len()) || (dir == Direction::RIGHT && start_pos.0==grid[0].len()){
            continue
        }

        let next_pos = dir.coord_dir(start_pos.0,start_pos.1);
        let next = grid[next_pos.1].chars().nth(next_pos.0).unwrap();
        if let Some(dirs) = get_dirs(next) {
            if dirs.iter().any(|d| d.opposite() == dir) {
                return (next_pos,dir.opposite())
            }
        }
    }
    panic!("Start has no connecting pipes");
}

#[allow(unused)]
pub fn part_1(grid: &str) -> u32 {
    let grid: Vec<&str> = grid.lines().collect();
    let start = grid.iter().enumerate().find_map(|(y,&l)|{
        if let Some(x) = l.chars().position(|c| c=='S') { Some((x,y)) } else { None }
    }).unwrap();

    let mut counter = 0u32;
    let (mut cur_pos, mut last_dir) = start_dir(&grid,&start);
    loop {
        if cur_pos == start { break }
        let cur_char = grid[cur_pos.1].chars().nth(cur_pos.0).unwrap();
        let dirs = get_dirs(cur_char).unwrap();
        let next_dir = dirs.iter().find(|&d| d != &last_dir).unwrap();

        cur_pos = next_dir.coord_dir(cur_pos.0,cur_pos.1);
        last_dir = next_dir.opposite();
        counter+=1;
    }
    ((counter as f64) / 2.0).ceil() as u32
}

#[allow(unused)]
pub fn part_2(grid: &str) -> u32 {
    let grid: Vec<&str> = grid.lines().collect();
    let start = grid.iter().enumerate().find_map(|(y,&l)|{
        if let Some(x) = l.chars().position(|c| c=='S') { Some((x,y)) } else { None }
    }).unwrap();

    let mut pipes: Vec<(i64,i64)> = vec![];
    let mut verts: Vec<(i64,i64)> = vec![];
    let (mut cur_pos, mut last_dir) = start_dir(&grid,&start);
    loop {
        let cur_char = grid[cur_pos.1].chars().nth(cur_pos.0).unwrap();
        pipes.push((cur_pos.0 as i64,cur_pos.1 as i64));
        if ['J','F','7','L','S'].contains(&cur_char) {verts.push((cur_pos.0 as i64,cur_pos.1 as i64))}

        if cur_pos == start { break }

        let dirs = get_dirs(cur_char).unwrap();
        let next_dir = dirs.iter().find(|&d| d != &last_dir).unwrap();

        cur_pos = next_dir.coord_dir(cur_pos.0,cur_pos.1);
        last_dir = next_dir.opposite();
    }

    verts.insert(0,(start.0 as i64,start.1 as i64));
    let mut edges: Vec<((i64,i64),(i64,i64))> = vec![];
    verts.windows(2).for_each(|w| edges.push((w[0], w[1])));

    let s = (start.0 as i64, start.1 as i64);
    let (bound_min,bound_max) = verts.iter().fold((s,s), |acc,e| ((e.0.min(acc.0.0),e.1.min(acc.0.1)),(e.0.max(acc.1.0),e.1.max(acc.1.1))));

    let mut counter: u32 = 0;
    for x in bound_min.0+1..bound_max.0 {
        for y in bound_min.1+1..bound_max.1 {
            if !pipes.contains(&(x,y)) {
                if is_inside(&edges,(&(x,y))) {
                    counter += 1;
                }
            }
        }
    }

    counter
}

fn is_inside(edges: &Vec<((i64,i64),(i64,i64))>, point: &(i64,i64)) -> bool {
    let mut counter = 0;
    let (xp,yp) = point;
    for ((x1,y1),(x2,y2)) in edges.iter() {
        let mg = *x1 as f64 + ((yp-y1) as f64/(y2-y1) as f64)*(x2-x1) as f64;
        if (yp < y1) != (yp < y2) && (*xp as f64) < mg {
            counter += 1;
        }
    }
    (counter % 2) == 1
}