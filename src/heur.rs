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

pub fn manhattan(p: &Puzzle, interference: bool) -> usize {
    let mut sum = 0;
    let n = p.n;
    for r in 0..n {
        for c in 0..n {
            let tile = p.tiles[(r, c)];
            if tile == 0 {
                continue;
            }
            let (tr, tc) = p.target(tile);
            sum += abs_diff(r, tr);
            sum += abs_diff(c, tc);
            if interference {
                if r == tr {
                    for xc in c+1..n {
                        let tile = p.tiles[(r, xc)];
                        let (yc, yr) = p.target(tile);
                        if yr == r && yc < tc {
                            sum += 2;
                        }
                    }
                }
                if c == tc {
                    for xr in r+1..n {
                        let tile = p.tiles[(xr, c)];
                        let (yc, yr) = p.target(tile);
                        if yc == c && yr < tr {
                            sum += 2;
                        }
                    }
                }
            }
        }
    }
    sum
}
