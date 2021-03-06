use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt;

use ndarray::prelude::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Puzzle {
    pub n: usize,
    pub tiles: Array2<u8>,
    pub blank_pos: (usize, usize),
}


impl Puzzle {
    pub fn new(n: usize, tiles: Array2<u8>) -> Self {
        for r in 0..n {
            for c in 0..n {
                if tiles[(r, c)] == 0 {
                    return Puzzle { n, tiles, blank_pos: (r, c) }
                }
            }
        }
        panic!("could not find blank")
    }

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

    pub fn is_goal(&self) -> bool {
        let n = self.n;
        let mut count = 1;
        for r in 0..n {
            for c in 0..n {
                if count as usize == n * n && self.tiles[(r, c)] == 0 {
                    return true;
                }
                if self.tiles[(r, c)] != count {
                    return false;
                }
                count += 1;
            }
        }
        panic!("internal error: is_goal fell off end");
    }

    pub fn solve_astar<H>(&self, h: H) -> Option<Vec<(usize, usize)>>
        where H: Fn(&Puzzle) -> usize
    {

        // Inspired by the Rust documentation.
        #[derive(Clone, PartialEq, Eq)]
        struct State {
            g: usize,
            cost: usize,
            position: Puzzle,
            prev_blank: Option<(usize, usize)>,
        }

        impl PartialOrd for State {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for State {
            fn cmp(&self, other: &Self) -> Ordering {
                other.cost.cmp(&self.cost)
            }
        }

        let mut closed = HashMap::new();
        let mut q = BinaryHeap::new();
        let cost = h(self);
        q.push(State { position: self.clone(), g: 0, cost, prev_blank: None });
        while let Some(mut s) = q.pop() {
            if s.position.is_goal() {
                let mut result = Vec::with_capacity(s.g);
                result.push(s.position.blank_pos);
                let mut prev_blank = s.prev_blank;
                while let Some(m) = prev_blank {
                    result.push(m);
                    s.position.make_move(m);
                    prev_blank = closed[&s.position];
                }
                let _ = result.pop();
                result.reverse();
                return Some(result);
            }
            for m in s.position.moves() {
                let mut p = s.position.clone();
                let prev_blank = Some(p.blank_pos);
                p.make_move(m);
                if !closed.contains_key(&p) {
                    let cost = s.g + 1 + h(&p);
                    q.push(State { g: s.g + 1, cost, position: p, prev_blank })
                }
            }
            closed.insert(s.position, s.prev_blank);
        }
        None
    }
}


impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (space, sep, width) = if self.n <= 3 {(
            " ".to_string(),
            "".to_string(),
            1,
        )} else {(
            "  ".to_string(),
            " ".to_string(),
            2,
        )};
        for r in 0..self.n {
            for c in 0..self.n {
                if c > 0 {
                    write!(f, "{}", sep)?;
                }
                let t = self.tiles[(r, c)];
                if t == 0 {
                    write!(f, "{}", space)?;
                } else {
                    write!(f, "{:01$}", t, width)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
