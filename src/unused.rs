use petgraph::graph::{DiGraph, Graph, NodeIndex};
use petgraph::visit::EdgeRef;
use petgraph::Directed;
use petgraph::Incoming;

/// Returns graph with node weights in a topological order.
/// Using Kahn's algorithm
fn topo_sort(gr: &DiGraph<usize, bool>, nodes: &[Option<NodeIndex>]) -> Vec<NodeIndex> {
    // stack for keeping track of nodes with no incoming edge
    let mut stack: Vec<NodeIndex> = Vec::with_capacity(gr.node_count());
    // list for keeping track of nodes in their final order
    let mut order: Vec<NodeIndex> = Vec::with_capacity(gr.node_count());
    // populate stack with existing no-incoming-edge nodes
    for node in nodes {
        if gr.edges_directed(node.unwrap(), Incoming).nth(0) == None {
            stack.push(node.unwrap());
        }
    }
    // make mutable copy of gr for removing edges from
    let mut gr_cp = gr.clone();
    // main body of algorithm
    while !stack.is_empty() {
        let n = stack.pop();
        order.push(n.unwrap());
        for e in gr.edges(n.unwrap()) {
            let m = e.target();
            gr_cp.remove_edge(e.id());
            if gr.edges_directed(m, Incoming).nth(0) == None {
                stack.push(m);
            }
        }
    }
    order
}

// I'm keeping this around to demonstrate I understand DFS, however,
// I ended up using Khan's algorithm instead.
/// Performs a depth first search on a graph. Does nothing.
fn DFS_iter(gr: &DiGraph<usize, bool>, nodes: &[Option<NodeIndex>]) {
    // stack for keeping track of nodes to visit next
    let mut stack: Vec<NodeIndex> = Vec::with_capacity(gr.node_count());
    // list, synced with indicies of nodes, for keeping track of which nodes
    // have been visited
    let mut explored = vec![false; gr.node_count()];
    // start searching from nodes[0] for now to avoid needing another parameter
    stack.push(nodes[0].expect("nodes array doesn't have a first element!"));
    while !stack.is_empty() {
        let v = stack.pop();
        // necessary to sync explored with nodes
        let v_index = nodes
            .iter()
            .position(|&x| x == v)
            .expect("NodeIndex doesn't exist in provided nodes array!");
        if !explored[v_index] {
            explored[v_index] = true;
            for edge in gr.edges(v.unwrap()) {
                stack.push(edge.target());
            }
        }
    }
}
