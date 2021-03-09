mod gen;
mod puzzle;
mod heur;
mod load;

use std::cmp::Ordering::*;
use std::path::PathBuf;

use gumdrop::Options;

pub use gen::*;
pub use puzzle::*;
pub use heur::*;
pub use load::*;

#[derive(Debug, Options)]
struct Opts {
    #[options(short = "n", help = "size (e.g. 3, 8, 15, 24)")]
    size: Option<usize>,

    #[options(help = "heuristic (dijkstra/0, manhattan)")]
    heuristic: Option<String>,

    #[options(help = "load puzzle from given path")]
    file: Option<PathBuf>,

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
            "dijkstra" | "0" => dijkstra,
            "manhattan" => manhattan,
            _ => {
                eprintln!("unknown heuristic {}", h);
                std::process::exit(1);
            }
        }
    } else {
        dijkstra
    };

    let mut p = if let Some(path) = opts.file {
        load_puzzle(path).unwrap_or_else(|e| {
            eprintln!("puzzle load failed: {}", e);
            std::process::exit(1);
        })
    } else {
        Puzzle::generate(n)
    };
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
