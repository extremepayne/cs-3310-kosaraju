use petgraph::graph::{Graph, DiGraph, NodeIndex};
use petgraph::Directed;
// use petgraph::visit::EdgeRef;

fn main() {
    let mut gr: DiGraph<usize,bool> = Graph::<usize, bool, Directed>::with_capacity(11, 16);
    let mut nodes: [Option<NodeIndex>; 11] = Default::default();
    for i in 0..nodes.len() {
        nodes[i] = Some(gr.add_node(i));
    }
    gr.extend_with_edges(&[
        (0, 2), (2, 4), (2, 10), (4, 0), (10, 7), (10, 5),
        (5, 9), (9, 7), (7, 5), (4, 6), (4, 8), (6, 8), (8, 3),
        (3, 6), (8, 1), (1, 3), (1, 9), (8, 7)
    ]);
    // expected result from this graph: (4, 3, 3, 1, 0)
    println!("{:?}", gr);
    // for e in gr.edges(nodes[8].unwrap()){
        // println!("we got an edge with source {:?} and dest {:?}", e.source(), e.target());
    // }
}

/// Returns sizes of top five strongly connected components.
fn kosaraju(gr: DiGraph<u32,bool>) -> (u32, u32, u32, u32, u32) {
    (0, 0, 0, 0, 0)
}
