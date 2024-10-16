use petgraph::graph::{DiGraph, Graph, NodeIndex};
use petgraph::visit::EdgeRef;
use petgraph::Directed;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("No arguments passed. Defaulting to sanity checks");
        sanity_checks();
    } else {
        let gr = read_file(&args[1]);
        println!("Reading file finished");
        let mut nodes: Vec<Option<NodeIndex>> = Vec::with_capacity(gr.node_count());
        for node in gr.node_indices() {
            nodes.push(Some(node));
        }
        // println!("nodes {:?}", nodes);
        // topo_sort(&gr, &nodes);
        // println!("order {:?}", order);
        let sccs = kosaraju(&gr, &nodes);
        println!("Top five strongly connected component sizes:");
        println!("{:?}", sccs);
    }
}

/// Reads in a test case file and constructs a graph
fn read_file(filename: &str) -> DiGraph<usize, bool> {
    println!("reading data file: {filename}");
    let data = File::open(filename).unwrap();
    let data_reader = io::BufReader::new(data).lines();
    let mut edges: Vec<(u32, u32)> = Vec::new();
    for line in data_reader.flatten() {
        let mut iter = line.split_whitespace();
        let a: u32 = iter.next().unwrap().parse().unwrap();
        let b: u32 = iter.next().unwrap().parse().unwrap();
        // we are subtracting one to avoid petgraph creating an extraneous node at index 0
        edges.push((a - 1, b - 1));
    }
    let mut gr: DiGraph<usize, bool> = Graph::<usize, bool, Directed>::new();
    gr.extend_with_edges(edges);
    // println!("{:?}", gr);
    gr
}

fn sanity_checks() {
    let mut gr: DiGraph<usize, bool> = Graph::<usize, bool, Directed>::with_capacity(11, 16);
    let mut nodes: Vec<Option<NodeIndex>> = Vec::with_capacity(11);
    for i in 0..11 {
        nodes.push(Some(gr.add_node(i)));
    }
    gr.extend_with_edges([
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
    println!("graph {:?}", gr);
    println!("nodes {:?}", nodes);
    let rev_gr = graph_reverse(&gr, &nodes);
    println!("reversed graph {:?}", rev_gr);
    let sorted = topo_sort(&gr, &nodes);
    println!("topological sort of verticies {:?}", sorted);
    let sccs = kosaraju(&gr, &nodes);
    println!("top 5 sccs {:?}", sccs);
    println!("should be [4, 3, 3, 1, 0]")
    // for e in gr.edges(nodes[8].unwrap()){
    // println!("we got an edge with source {:?} and dest {:?}", e.source(), e.target());
    // }
}

/// Returns sizes of top five strongly connected components.
fn kosaraju(gr: &DiGraph<usize, bool>, nodes: &Vec<Option<NodeIndex>>) -> [u32; 5] {
    // reverse graph
    let gr_rev = graph_reverse(gr, nodes);
    // topologically sort reversed graph to produce magic ordering
    let magic_order = topo_sort(&gr_rev, nodes);
    // second pass of dfs
    //
    // list, synced with indicies of nodes, for keeping track of which nodes
    // have been visited
    let mut explored = vec![false; gr.node_count()];
    // variable to keep track of which scc we're working on
    let mut scc_id: usize = 0;
    // vec, indicies synced, to keep track of which scc a node belongs to
    let mut scc_ids: Vec<Option<usize>> = vec![None; gr.node_count()];
    // stack for keeping track of nodes to visit next
    let mut stack: Vec<NodeIndex> = Vec::with_capacity(gr.node_count());
    // list, synced indicies, for keeping track of which nodes are in stack?
    let mut in_stack = vec![false; gr.node_count()];
    // counter for keeping track of how many nodes so far
    let mut nodes_processed: u32 = 0;
    // main loop
    for node in magic_order {
        let node_index = nodes.iter().position(|&x| x == Some(node)).unwrap();
        if !explored[node_index] {
            scc_id += 1;
            stack.push(node);
            in_stack[node_index] = true;
            while !stack.is_empty() {
                let current = stack.pop();
                let current_index = nodes.iter().position(|&x| x == current).unwrap();
                in_stack[current_index] = false;
                if !explored[current_index] {
                    explored[current_index] = true;
                    scc_ids[current_index] = Some(scc_id);
                    for edge in gr.edges(current.unwrap()) {
                        let v = edge.target();
                        let v_index = nodes.iter().position(|&x| x == Some(v)).unwrap();
                        if !explored[v_index] && !in_stack[v_index] {
                            stack.push(v);
                            in_stack[v_index] = true;
                        }
                    }
                }
            }
            nodes_processed += 1;
            if nodes_processed % 1000 == 0 {
                println!("Finished scc computation for {nodes_processed} nodes");
            }
        }
    }
    // get sizes out of scc_ids
    let mut scc_sizes: Vec<u32> = vec![0; scc_id + 1];
    for id in scc_ids {
        scc_sizes[id.unwrap()] += 1;
    }
    // reverse sort to get biggest first
    scc_sizes.sort();
    scc_sizes.reverse();
    // pad out to five if necessary
    while scc_sizes.len() < 5 {
        scc_sizes.push(0)
    }
    // convert to exactly sized array for returning
    const NUM: usize = 5;
    let top_scc_sizes: [u32; 5] = scc_sizes[..NUM].try_into().unwrap();
    top_scc_sizes
}

/// Reverses directionality of all edges in input graph
fn graph_reverse(
    gr: &DiGraph<usize, bool>,
    nodes: &Vec<Option<NodeIndex>>,
) -> DiGraph<usize, bool> {
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

/// Returns list with node references in a topological order.
/// Using iterative DFS (hopefully).
fn topo_sort(gr: &DiGraph<usize, bool>, nodes: &Vec<Option<NodeIndex>>) -> Vec<NodeIndex> {
    // list for keeping track of nodes in their final order
    let mut order: Vec<NodeIndex> = Vec::with_capacity(gr.node_count());
    // list, synced with indicies of nodes, for keeping track of which nodes
    // have been visited
    let mut explored = vec![false; gr.node_count()];
    // stack for keeping track of nodes to visit next
    let mut stack: Vec<NodeIndex> = Vec::with_capacity(gr.node_count());
    // list, synced indicies, for keeping track of which nodes are in stack?
    let mut in_stack = vec![false; gr.node_count()];
    // counter for keeping track of how many nodes so far
    let mut nodes_processed: u32 = 0;
    // main loop
    for node in nodes {
        // println!("order {:?}", order);
        // println!("stack {:?}", stack);
        let node_index = nodes.iter().position(|&x| x == *node).unwrap();
        if !explored[node_index] {
            stack.push(node.unwrap());
            in_stack[node_index] = true;
            while !stack.is_empty() {
                let current = stack.last();
                // this is so ugly i hate it but this is due in an hour
                let current_index = nodes
                    .iter()
                    .position(|&x| x == Some(*current.unwrap()))
                    .unwrap();
                if !explored[current_index] {
                    explored[current_index] = true;
                    for edge in gr.edges(*current.unwrap()) {
                        let v = edge.target();
                        let v_index = nodes.iter().position(|&x| x == Some(v)).unwrap();
                        if !explored[v_index] && !in_stack[v_index] {
                            stack.push(v);
                            in_stack[v_index] = true;
                        }
                    }
                } else {
                    // give this node the current lowest avaliable indexing
                    // it should have highest at the end, so we will reverse
                    // the vector
                    order.push(*current.unwrap());
                    stack.pop();
                }
            }
        }
        nodes_processed += 1;
        if nodes_processed % 1000 == 0 {
            println!("Finished topological order for {nodes_processed} nodes");
        }
    }
    // reverse the vector as noted above
    order.reverse();
    order
}
