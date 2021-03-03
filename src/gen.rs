use ndarray::prelude::*;
use rand::prelude::*;

use crate::*;

impl Puzzle {

    pub fn generate(n: usize) -> Self {
        // Generate solved board.
        let mut tiles = Array2::zeros((n, n));
        let mut tile_number = 1;
        for r in 0..n {
            for c in 0..n {
                tiles[(r, c)] = tile_number;
                tile_number += 1;
            }
        }
        let blank_pos = (n - 1, n - 1);
        tiles[blank_pos] = 0;
        let mut puzzle = Puzzle::new(n, tiles);

        // Stir "sufficiently".
        let mut prng = rand::thread_rng();
        for _ in 0..n.pow(8) {
            let m = puzzle.moves().choose(&mut prng).unwrap();
            puzzle.make_move(m);
        }

        puzzle
    }
}
