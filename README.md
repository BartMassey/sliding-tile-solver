# sliding-tile-solver: State-Space Search Sliding-Tile Puzzle Solver
Bart Massey 2021

This codebase is a Rust implementation of State Space search
for the Sliding Tile Puzzle problem. It currently implements
A\* search with various heuristics — further algorithms and
heuristics are planned.

The code is basically undocumented and poorly tested.  Run
the program with

    cargo run --release -- --help

to get information on arguments.

The directory `p/` contains a few sample puzzles to try
out. On my box, `p/p2-15-60.txt` runs in about 15 seconds
with the `interference` heuristic.

Huge thanks to Grant Baker and Tu Vu for providing some of
the research, consulting and advising on this as we studied
the problem together.

This code is available under the "MIT License". Please see
the file `LICENSE` in this distribution for license terms.
