use crate::*;

pub fn dijkstra(_: &Puzzle) -> usize {
    0
}

impl Puzzle {
    fn target(&self, tile: u8) -> (usize, usize) {
        if tile == 0 {
            return (self.n - 1, self.n - 1);
        }
        let p = tile as usize - 1;
        (p / self.n, p % self.n)
    }
}

fn abs_diff(x: usize, y: usize) -> usize {
    if x <= y {
        y - x
    } else {
        x - y
    }
}

pub fn manhattan(p: &Puzzle) -> usize {
    let mut sum = 0;
    for r in 0..p.n {
        for c in 0..p.n {
            let tile = p.tiles[(r, c)];
            if tile == 0 {
                continue;
            }
            let (tr, tc) = p.target(tile);
            sum += abs_diff(r, tr);
            sum += abs_diff(c, tc);
        }
    }
    sum
}
