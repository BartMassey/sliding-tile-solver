mod gen;
mod puzzle;

use std::cmp::Ordering::*;

use gumdrop::Options;

pub use gen::*;
pub use puzzle::*;

#[derive(Debug, Options)]
struct Opts {
    #[options(short = "n", help = "size (e.g. 3, 8, 15, 24)")]
    size: Option<usize>,

    #[options(help = "heuristic (dijkstra/0)")]
    heuristic: Option<String>,

    #[options(help = "show moves")]
    moves: bool,

    #[options(help = "help")]
    help: bool,
}

fn main() {
    let opts = Opts::parse_args_default_or_exit();

    let mut n = 3;
    if let Some(m) = opts.size {
        for i in 2.. {
            match m.cmp(&(i * i - 1)) {
                Greater => (),
                Equal => { n = i; break; },
                Less => {
                    eprintln!("unknown puzzle size {}", m);
                    std::process::exit(1);
                },
            }
        }
    }

    let h = if let Some(h) = opts.heuristic {
        match h.as_str() {
            "dijkstra" | "0" => |_: &Puzzle| 0,
            _ => {
                eprintln!("unknown heuristic {}", h);
                std::process::exit(1);
            }
        }
    } else {
        |_: &Puzzle| 0
    };

    let mut p = Puzzle::generate(n);
    println!("{}", p);

    if let Some(moves) = p.solve_astar(h) {
        println!("{} moves", moves.len());
        if opts.moves {
            for m in moves {
                println!("{}", p);
                p.make_move(m);
            }
            println!("{}", p);
        }
    } else {
        println!("no solution");
    }
}
