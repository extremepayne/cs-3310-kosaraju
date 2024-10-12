use petgraph::graph::{Graph, DiGraph};
use petgraph::Directed;

fn main() {
    let gr: DiGraph<u32,bool> = Graph::<u32, bool, Directed>::with_capacity(11, 16);
}

/// Returns sizes of top five strongly connected components.
fn kosaraju(gr: DiGraph<u32,bool>) -> (u32, u32, u32, u32, u32) {
    (0, 0, 0, 0, 0)
}
