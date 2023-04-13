use clap::Parser;
use graph::*;
use std::io::{stdout, Write};

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
    let (mut s, mut up) = (String::new(), true);

    while {
        show_progress(&mut s, &mut up);
        let edges = generate_random_graph_edges(args.nodes, args.edges);
        if args.edges <= args.k {
            println!(
                "All graph of {} edges is not {}-edge-connected (not defined).",
                args.edges, args.k
            );
            return;
        }
        !Graph::new(args.nodes, edges.clone()).is_k_edge_connected(args.k) || {
            println!("\n{edges:?} is {}-edge-connected.", args.k);
            if args.subgraphs {
                enumerate_k_edge_connected_induced_subgraphs(args.nodes, edges, args.k);
            }
            false
        }
    } {}
}

fn show_progress(s: &mut String, up: &mut bool) {
    let clear = (0..=100).map(|_| ' ').collect::<String>();
    std::thread::sleep(std::time::Duration::from_millis(10));
    if *up {
        print!("\r{clear}");
        stdout().flush().unwrap();
        print!("\r{s}");
        stdout().flush().unwrap();
        if s.len() == 100 {
            *up = false;
        } else {
            s.push('|')
        }
    } else {
        print!("\r{clear}");
        stdout().flush().unwrap();
        print!("\r{s}");
        stdout().flush().unwrap();
        if s.len() == 0 {
            *up = true
        } else {
            s.pop();
        }
    }
}
