use clap::Parser;
use graph::*;

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
    k: usize,

    /// enumerate all k-edge-connected induced subgraphs
    #[arg(short, long)]
    subgraphs: bool,
}

fn main() {
    let args = Args::parse();

    while {
        let edges = generate_random_graph_edges(args.nodes, args.edges);
        if args.edges <= args.k {
            println!(
                "All graph of {} edges is not {}-edge-connected (not defined).",
                args.edges, args.k
            );
            return;
        }
        !Graph::new(args.nodes, edges.clone()).is_k_edge_connected(args.k) || {
            println!("{edges:?} is {}-edge-connected.", args.k);
            if args.subgraphs {
                enumerate_k_edge_connected_induced_subgraphs(args.nodes, edges, args.k);
            }
            false
        }
    } {}
}
