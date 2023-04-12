use clap::Parser;
use graph::*;
use itertools::Itertools;

pub mod graph;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The number of nodes
    #[arg(short, long)]
    nodes: usize,

    /// The number of edges
    #[arg(short, long)]
    edges: usize,

    /// k
    #[arg(short)]
    k: usize
}

fn main() {
    let args = Args::parse();

    while 'condition: {
        let edges = generate_random_graph_edges(args.nodes, args.edges);
        if args.edges <= args.k {
            println!("All graph of {} edges is not {}-edge-connected (not defined).", args.edges, args.k);
            return
        }
        for sub in edges.clone().into_iter().combinations(args.edges - args.k) {
            if !Graph::new(args.nodes, sub).is_connected() {
                break 'condition true;
            }
        }
        println!("{edges:?} is {}-edge-connected.", args.k);
        false
    } {}
}
