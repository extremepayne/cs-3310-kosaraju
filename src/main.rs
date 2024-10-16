#![recursion_limit = "5000"]

use petgraph::graph::{DiGraph, Graph, NodeIndex};
use petgraph::visit::EdgeRef;
use petgraph::Directed;
use petgraph::Incoming;

fn main() {
    let mut gr: DiGraph<usize, bool> = Graph::<usize, bool, Directed>::with_capacity(11, 16);
    let mut nodes: [Option<NodeIndex>; 11] = Default::default();
    for i in 0..nodes.len() {
        nodes[i] = Some(gr.add_node(i));
    }
    gr.extend_with_edges(&[
        (0, 2),
        (2, 4),
        (2, 10),
        (4, 0),
        (10, 7),
        (10, 5),
        (5, 9),
        (9, 7),
        (7, 5),
        (4, 6),
        (4, 8),
        (6, 8),
        (8, 3),
        (3, 6),
        (8, 1),
        (1, 3),
        (1, 9),
        (8, 7),
    ]);
    // expected result from this graph: (4, 3, 3, 1, 0)
    let res = kosaraju(&gr, &nodes);
    println!("{:?}", gr);
    println!("{:?}", res);
    let rev_gr = graph_reverse(&gr, &nodes);
    println!("{:?}", rev_gr);
    // for e in gr.edges(nodes[8].unwrap()){
    // println!("we got an edge with source {:?} and dest {:?}", e.source(), e.target());
    // }
}

/// Returns sizes of top five strongly connected components.
fn kosaraju(gr: &DiGraph<usize, bool>, nodes: &[Option<NodeIndex>]) -> (u32, u32, u32, u32, u32) {
    // TODO
    (0, 0, 0, 0, 0)
}

/// Reverses directionality of all edges in input graph
fn graph_reverse(gr: &DiGraph<usize, bool>, nodes: &[Option<NodeIndex>]) -> DiGraph<usize, bool> {
    // create empty graph to hold reversed graph
    let mut rev_gr: DiGraph<usize, bool> =
        Graph::<usize, bool>::with_capacity(gr.node_count(), gr.edge_count());
    // add same nodes as original graph
    for node in nodes {
        rev_gr.add_node(*gr.node_weight(node.unwrap()).unwrap());
    }
    // add edges, but with source and target reversed
    for node in nodes {
        for edge in gr.edges(node.unwrap()) {
            rev_gr.add_edge(edge.target(), edge.source(), false);
        }
    }
    rev_gr
}

fn topo_sort(gr: &DiGraph<usize, bool>, nodes: &[Option<NodeIndex>]) -> Vec<NodeIndex> {
    // list for keeping track of nodes in their final order
    let mut order: Vec<NodeIndex> = Vec::with_capacity(gr.node_count());
    order
}
