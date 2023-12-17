use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use ndarray::Array2;

#[allow(unused)]
pub fn part_1(city: &str) -> usize {
    let nrows = city.lines().count();
    let ncols = city.lines().next().unwrap().len();
    let mut blocks: Array2<u8> = Array2::zeros((nrows,ncols));
    let mut visited: Array2<Visited>  = Array2::from_elem((nrows,ncols), Visited::default());

    for (row_i, line) in city.lines().enumerate() {
        for (col_i, c) in line.char_indices() {
            blocks[[row_i,col_i]] = c.to_digit(10).unwrap() as u8; // could cast directly from byte
        }
    }

    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(Reverse(Crucible{cost: blocks[[0,1]].clone() as usize, origin: Origin::Left, position: Position{x:1,y:0}, time_to_live: 2 }));
    priority_queue.push(Reverse(Crucible{cost: blocks[[1,0]].clone() as usize, origin: Origin::Left, position: Position{x:0,y:1}, time_to_live: 2 }));

    while let Some(crucible) = priority_queue.pop() {
        let crucible = crucible.0;
        if visited[[crucible.position.y as usize, crucible.position.x as usize]].did_visit(&crucible.origin, crucible.time_to_live) {
            continue
        }
        if crucible.position.x as usize == ncols-1 && crucible.position.y as usize == nrows-1 {
            return crucible.cost
        }

        if crucible.time_to_live != 0{
            if let Some(next_pos) = crucible.next(nrows,ncols) {
                let next_row = next_pos.y as usize;
                let next_col = next_pos.x as usize;
                let n = Crucible{cost: crucible.cost + blocks[[next_row,next_col]] as usize, origin: crucible.origin, position: next_pos, time_to_live: crucible.time_to_live-1 };
                priority_queue.push(Reverse(n))
            }
        }
        if let Some(next_pos) = crucible.next_left(nrows,ncols) {
            let next_row = next_pos.y as usize;
            let next_col = next_pos.x as usize;
            let n = Crucible{cost: crucible.cost + blocks[[next_row,next_col]] as usize, origin: crucible.origin.rotate(false), position: next_pos, time_to_live: 2 };
            priority_queue.push(Reverse(n))
        }
        if let Some(next_pos) = crucible.next_right(nrows,ncols) {
            let next_row = next_pos.y as usize;
            let next_col = next_pos.x as usize;
            let n = Crucible{cost: crucible.cost + blocks[[next_row,next_col]] as usize, origin: crucible.origin.rotate(true), position: next_pos, time_to_live: 2 };
            priority_queue.push(Reverse(n))
        }
    }

    panic!("Path not found!")
}

#[derive(Eq, Copy, Clone, Debug)]
struct Crucible {
    cost: usize,
    time_to_live: u8, // 0-2
    origin: Origin,
    position: Position,
} impl Crucible {
    fn next(&self, nrows: usize, ncols: usize) -> Option<Position> {
        let next_pos: Position = self.position.move_dir( self.origin.flip().direction() );
        if next_pos.x < 0 || next_pos.y < 0 || next_pos.x >= ncols as i32 || next_pos.y >= nrows as i32 {
            return None
        }
        Some(next_pos)
    }
    fn next_left(&self, nrows: usize, ncols: usize) -> Option<Position> {
        let next_pos: Position = self.position.move_dir( self.origin.rotate(true).direction() );
        if next_pos.x < 0 || next_pos.y < 0 || next_pos.x >= ncols as i32 || next_pos.y >= nrows as i32 {
            return None
        }
        Some(next_pos)
    }
    fn next_right(&self, nrows: usize, ncols: usize) -> Option<Position> {
        let next_pos: Position = self.position.move_dir( self.origin.rotate(false).direction() );
        if next_pos.x < 0 || next_pos.y < 0 || next_pos.x >= ncols as i32 || next_pos.y >= nrows as i32 {
            return None
        }
        Some(next_pos)
    }
}

impl Ord for Crucible {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for Crucible {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Crucible {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

#[derive(Eq, PartialEq,Ord, PartialOrd, Copy, Clone, Debug)]
enum Origin{
    Top,
    Bottom,
    Left,
    Right,
} impl Origin {
    fn rotate(&self, clockwise: bool) -> Self {
        if clockwise {
            match self {
                Origin::Top => Origin::Right,
                Origin::Right => Origin::Bottom,
                Origin::Bottom => Origin::Left,
                Origin::Left => Origin::Top,
            }
        } else {
            match self {
                Origin::Top => Origin::Left,
                Origin::Left => Origin::Bottom,
                Origin::Bottom => Origin::Right,
                Origin::Right => Origin::Top,
            }
        }
    }

    fn flip(&self) -> Self {
        match self {
            Origin::Top => Origin::Bottom,
            Origin::Bottom => Origin::Top,
            Origin::Left => Origin::Right,
            Origin::Right => Origin::Left,
        }
    }

    fn direction(&self) -> (i32,i32) {
        match self {
            Origin::Top => (0, -1),
            Origin::Bottom => (0, 1),
            Origin::Left => (-1, 0),
            Origin::Right => (1, 0),
        }
    }
}
#[derive(Eq, PartialEq,Ord, PartialOrd, Copy, Clone, Debug)]
struct Position{
    x: i32,
    y: i32,
}

impl Position {
    fn move_dir(&self, dir: (i32, i32)) -> Self {
        Position{x: self.x + dir.0, y: self.y + dir.1}
    }
}

#[derive(Clone)]
struct Visited{
    left: (bool, bool, bool), // 0, 1, 2
    right: (bool, bool, bool),
    top: (bool, bool, bool),
    bottom: (bool, bool, bool),
} impl Visited {
    fn default() -> Self {
        let b = (false, false, false);
        Visited {left: b, right: b, top: b, bottom: b}
    }

    fn did_visit(&mut self, from: &Origin, ttl: u8) -> bool {
        match from {
            Origin::Left => {
                match ttl {
                    0 => if self.left.0 {return true} else {self.left.0 = true},
                    1 => if self.left.1 {return true} else {self.left.1 = true},
                    2 => if self.left.2 {return true} else {self.left.2 = true},
                    _ => (),
                }
            },
            Origin::Right => {
                match ttl {
                    0 => if self.right.0 {return true} else {self.right.0 = true},
                    1 => if self.right.1 {return true} else {self.right.1 = true},
                    2 => if self.right.2 {return true} else {self.right.2 = true},
                    _ => (),
                }
            },
            Origin::Top => {
                match ttl {
                    0 => if self.top.0 {return true} else {self.top.0 = true},
                    1 => if self.top.1 {return true} else {self.top.1 = true},
                    2 => if self.top.2 {return true} else {self.top.2 = true},
                    _ => (),
                }
            },
            Origin::Bottom => {
                match ttl {
                    0 => if self.bottom.0 {return true} else {self.bottom.0 = true},
                    1 => if self.bottom.1 {return true} else {self.bottom.1 = true},
                    2 => if self.bottom.2 {return true} else {self.bottom.2 = true},
                    _ => (),
                }
            },
        }
        false
    }
}