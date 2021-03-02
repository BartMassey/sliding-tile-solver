mod gen;

use ndarray::prelude::*;

#[derive(Debug)]
pub struct Puzzle {
    n: usize,
    tiles: Array2<u8>,
    blank_pos: (usize, usize),
}


impl Puzzle {
    pub fn moves(&self) -> impl Iterator<Item=(usize, usize)> {
        const DELTAS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let (r, c) = self.blank_pos;
        let r = r as isize;
        let c = c as isize;
        let n = self.n;
        DELTAS
            .iter()
            .filter_map(move |&(dr, dc)| {
                if r + dr >= 0 && r + dr < n as isize && c + dc >= 0 && c + dc < n as isize {
                    Some(((r + dr) as usize, (c + dc) as usize))
                } else {
                    None
                }
            })
    }

    pub fn make_move(&mut self, m: (usize, usize)) {
        let blank_pos = self.blank_pos;
        self.tiles.swap(m, blank_pos);
        self.blank_pos = m;
    }
}

fn main() {
    println!("{:?}", Puzzle::generate(3));
}
