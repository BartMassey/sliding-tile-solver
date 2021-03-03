mod gen;
mod puzzle;

pub use gen::*;
pub use puzzle::*;

fn main() {
    let mut p = Puzzle::generate(3);
    println!("{}", p);
    if let Some(moves) = p.solve_dijkstra() {
        println!("{} moves", moves.len());
        for m in moves {
            println!("{}", p);
            p.make_move(m);
        }
        println!("{}", p);
    } else {
        println!("no solution");
    }
}
