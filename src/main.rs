pub mod database;

use clap::Parser;
use petgraph::algo::astar;

use crate::database::create_database;

/// Calculate best teleport route between towns
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Starting location
    #[clap(value_parser)]
    from: String,

    /// Destination
    #[clap(value_parser)]
    to: String,
}

fn main() {
    let args = Args::parse();

    let db = create_database();

    let shortest = astar(
        &db,
        &args.from,
        |finish| finish == args.to,
        |(_, _, &price)| price,
        |_| 0,
    );

    if let Some((price, path)) = shortest {
        println!(
            "The shortest path from {} to {} is costs {price} adena",
            args.from, args.to
        );
        println!();
        let mut path_iter = path.iter().peekable();
        while let Some(town) = path_iter.next() {
            if path_iter.peek().is_some() {
                print!("{town} -> ");
            } else {
                println!("{town}");
            }
        }
    } else {
        println!("{} is not reachable from {}", args.to, args.from);
    }
}
