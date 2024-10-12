use petgraph::graph::{Graph, DiGraph};
use petgraph::Directed;

fn main() {
    let gr: DiGraph<u32,bool> = Graph::<u32, bool, Directed>::with_capacity(11, 16);
}
