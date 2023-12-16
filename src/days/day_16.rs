use std::collections::{BTreeMap, VecDeque};
use naga::FastHashMap;
use ndarray::{Array2, ArrayViewMut, Ix1, s};

#[derive(Eq, PartialEq,Ord, PartialOrd, Copy, Clone, Debug)]
enum Element{
     SplitterVert,
     SplitterHor,
     MirrorUpLeft,
     MirrorUpRight,
 } impl Element {
    fn from(val: char) -> Option<Self> {
        match val {
            '|' => Some(Element::SplitterVert),
            '-' => Some(Element::SplitterHor),
            '\\' => Some(Element::MirrorUpLeft),
            '/' => Some(Element::MirrorUpRight),
            _ => None,
        }
    }
}
#[derive(Eq, PartialEq,Ord, PartialOrd, Copy, Clone, Debug)]
enum BeamOrigin{
    Up,
    Down,
    Left,
    Right,
} impl BeamOrigin {
    fn aligns(&self, other: &EnergyOrigin) -> bool {
        match self {
            BeamOrigin::Up => other.up,
            BeamOrigin::Down => other.down,
            BeamOrigin::Left => other.left,
            BeamOrigin::Right => other.right,
        }
    }
}
#[derive(Eq, PartialEq,Ord, PartialOrd, Copy, Clone, Debug)]
struct EnergyOrigin {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
} impl EnergyOrigin {
    const fn new() -> Self {
        EnergyOrigin {up:false,down:false,left:false,right:false}
    }

    fn set(&mut self, from: &BeamOrigin) {
        match from {
            BeamOrigin::Left => self.left = true,
            BeamOrigin::Right => self.right = true,
            BeamOrigin::Up => self.up = true,
            BeamOrigin::Down => self.down = true,
        }
    }
    fn any(&self) -> bool {
        self.up || self.down || self.left || self.right
    }
}
#[derive(Eq, PartialEq,Ord, PartialOrd, Copy, Clone, Debug)]
struct Beam {
    start_row: usize,
    start_col: usize,
    origin: BeamOrigin,
}


pub fn part_1(tiles: &str) -> usize{
    let mut row_elements: FastHashMap<usize,BTreeMap<usize,Element>> = FastHashMap::default();
    let mut col_elements: FastHashMap<usize,BTreeMap<usize,Element>> = FastHashMap::default();

    let mut energized: Array2<EnergyOrigin> = Array2::from_elem((tiles.lines().count(), tiles.lines().next().unwrap().len()), EnergyOrigin::new());
    let nrows: usize = energized.nrows();
    let ncols: usize = energized.ncols();

    for (ri, row) in tiles.lines().enumerate() {
        for (ci, col) in row.char_indices() {
            if let Some(element) = Element::from(col) {
                row_elements.entry(ri).or_insert(BTreeMap::new()).insert(ci, element.clone());
                col_elements.entry(ci).or_insert(BTreeMap::new()).insert(ri, element);
            }
        }
    }

    // ((start_row,start_col), origin)
    let mut beams: VecDeque<Beam> = VecDeque::new();
    beams.push_back(Beam{start_row:0,start_col:0,origin:BeamOrigin::Left});

    while let Some(current_beam) = beams.pop_front() {
        // println!(" ");
        // println!("{:?}", current_beam);
        if current_beam.origin.aligns(&energized[[current_beam.start_row,current_beam.start_col]]){
            continue
        }
        let mut slice: ArrayViewMut<EnergyOrigin,Ix1>;

        let intersection = match current_beam.origin {
            BeamOrigin::Left => {
                row_elements[&current_beam.start_row].range(current_beam.start_col..).find(|e|e.1!=&Element::SplitterHor)
            },
            BeamOrigin::Right => {
                row_elements[&current_beam.start_row].range(..=current_beam.start_col).rev().find(|e|e.1!=&Element::SplitterHor)
            },
            BeamOrigin::Up => {
                col_elements[&current_beam.start_col].range(current_beam.start_row..).find(|e|e.1!=&Element::SplitterVert)
            },
            BeamOrigin::Down => {
                col_elements[&current_beam.start_col].range(..=current_beam.start_row).rev().find(|e|e.1!=&Element::SplitterVert)
            },
        };

        if let Some((end,element)) = intersection {
            let trow: usize;
            let tcol: usize;
            match current_beam.origin {
                BeamOrigin::Left | BeamOrigin::Right => {
                    trow = current_beam.start_row;
                    tcol = *end;
                    slice = energized.slice_mut(s![current_beam.start_row ,current_beam.start_col.min(tcol)..=current_beam.start_col.max(tcol)])
                },
                BeamOrigin::Up | BeamOrigin::Down => {
                    trow = *end;
                    tcol = current_beam.start_col;
                    slice = energized.slice_mut(s![current_beam.start_row.min(trow)..= current_beam.start_row.max(trow) ,current_beam.start_col])
                }
            }
            match current_beam.origin{
                BeamOrigin::Up => {
                    match element {
                        Element::SplitterVert => panic!("vert should be filterd"),
                        Element::SplitterHor => {
                            if tcol < ncols-1 { beams.push_back(Beam { start_row: trow, start_col: tcol + 1, origin: BeamOrigin::Left }) }
                            if tcol > 0 { beams.push_back(Beam { start_row: trow, start_col: tcol - 1, origin: BeamOrigin::Right }) }
                        }
                        Element::MirrorUpLeft => if tcol < ncols-1 { beams.push_back(Beam { start_row: trow, start_col: tcol + 1, origin: BeamOrigin::Left }) },
                        Element::MirrorUpRight => if tcol > 0 { beams.push_back(Beam { start_row: trow, start_col: tcol - 1, origin: BeamOrigin::Right }) },
                    }
                },
                BeamOrigin::Down => {
                    match element {
                        Element::SplitterVert => panic!("vert should be filterd"),
                        Element::SplitterHor => {
                            if tcol < ncols-1 { beams.push_back(Beam { start_row: trow, start_col: tcol + 1, origin: BeamOrigin::Left }) }
                            if tcol > 0 { beams.push_back(Beam { start_row: trow, start_col: tcol - 1, origin: BeamOrigin::Right }) }
                        }
                        Element::MirrorUpLeft => if tcol > 0 { beams.push_back(Beam { start_row: trow, start_col: tcol - 1, origin: BeamOrigin::Right }) },
                        Element::MirrorUpRight => if tcol < ncols-1 { beams.push_back(Beam { start_row: trow, start_col: tcol + 1, origin: BeamOrigin::Left }) },
                    }
                },
                BeamOrigin::Left => {
                    match element {
                        Element::SplitterHor => panic!("hor should be filterd"),
                        Element::SplitterVert => {
                            if trow < nrows-1 { beams.push_back(Beam { start_row: trow + 1, start_col: tcol, origin: BeamOrigin::Up }) }
                            if trow > 0 { beams.push_back(Beam { start_row: trow - 1, start_col: tcol, origin: BeamOrigin::Down }) }
                        }
                        Element::MirrorUpLeft => if trow < nrows-1 { beams.push_back(Beam { start_row: trow + 1, start_col: tcol, origin: BeamOrigin::Up }) },
                        Element::MirrorUpRight => if trow > 0 { beams.push_back(Beam { start_row: trow - 1, start_col: tcol, origin: BeamOrigin::Down }) },
                    }
                },
                BeamOrigin::Right => {
                    match element {
                        Element::SplitterHor => panic!("hor should be filterd"),
                        Element::SplitterVert => {
                            if trow < nrows-1 { beams.push_back(Beam { start_row: trow + 1, start_col: tcol, origin: BeamOrigin::Up }) }
                            if trow > 0 { beams.push_back(Beam { start_row: trow - 1, start_col: tcol, origin: BeamOrigin::Down }) }
                        }
                        Element::MirrorUpLeft => if trow > 0 { beams.push_back(Beam { start_row: trow - 1, start_col: tcol, origin: BeamOrigin::Down }) },
                        Element::MirrorUpRight => if trow < nrows-1 { beams.push_back(Beam { start_row: trow + 1, start_col: tcol, origin: BeamOrigin::Up }) },
                    }
                }
            }
            // println!("row: {trow}, col: {tcol} > {:?}", element);
        } else {
            slice = match current_beam.origin {
                BeamOrigin::Left => energized.slice_mut(s![current_beam.start_row,current_beam.start_col..]),
                BeamOrigin::Right => energized.slice_mut(s![current_beam.start_row,..=current_beam.start_col]),
                BeamOrigin::Up => energized.slice_mut(s![current_beam.start_row..,current_beam.start_col]),
                BeamOrigin::Down => energized.slice_mut(s![..=current_beam.start_row,current_beam.start_col]),
            };
            // println!("no other tiles hit");
        }
        // println!("{:?}", slice);
        slice.par_map_inplace(|e| e.set(&current_beam.origin));
    }

    energized.iter()
        .filter(|e| e.any())
        .count()
}
