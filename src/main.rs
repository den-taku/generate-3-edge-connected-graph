use clap::Parser;
use graph::*;
use itertools::Itertools;

pub mod graph;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The number of nodes
    #[arg(short, long, default_value_t = 1)]
    nodes: usize,

    /// The number of edges
    #[arg(short, long, default_value_t = 0)]
    edges: usize,
}

fn main() {
    let args = Args::parse();

    while 'condition: {
        let edges = generate_random_graph_edges(args.nodes, args.edges);
        println!("{edges:?}");
        for sub in edges.clone().into_iter().combinations(args.edges - 3) {
            if !Graph::new(args.nodes, sub).is_connected() {
                break 'condition true;
            }
        }
        println!("{edges:?}");
        false
    } {}
}
