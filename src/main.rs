#![recursion_limit = "5000"]

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
        // let mut nodes: [Option<NodeIndex>; gr.node_count()] = Default::default();
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
        edges.push((a,b));
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
    // DFS subroutine
    fn dfs_scc(
        gr: &Graph<usize, bool>,
        nodes: &Vec<Option<NodeIndex>>,
        explored: &mut Vec<bool>,
        scc_id: usize,
        scc_ids: &mut Vec<Option<usize>>,
        start_node: Option<NodeIndex>,
    ) {
        let start_node_index = nodes
            .iter()
            .position(|&x| x == start_node)
            .expect("NodeIndex doesn't exist in provided nodes array!");
        explored[start_node_index] = true;
        scc_ids[start_node_index] = Some(scc_id);
        for edge in gr.edges(start_node.unwrap()) {
            let v = edge.target();
            let v_index = nodes.iter().position(|&x| x == Some(v)).unwrap();
            if !explored[v_index] {
                dfs_scc(gr, nodes, explored, scc_id, scc_ids, Some(v));
            }
        }
    }
    // main loop
    for node in magic_order {
        let node_index = nodes.iter().position(|&x| x == Some(node)).unwrap();
        if !explored[node_index] {
            scc_id += 1;
            dfs_scc(gr, nodes, &mut explored, scc_id, &mut scc_ids, Some(node));
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
fn graph_reverse(gr: &DiGraph<usize, bool>, nodes: &Vec<Option<NodeIndex>>) -> DiGraph<usize, bool> {
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

/// Returns graph with node weights in a topological order.
/// Uses recursive DFS. NB ensure recursion_limit is set sufficiently high
/// for the problems you want to work with.
fn topo_sort(gr: &DiGraph<usize, bool>, nodes: &Vec<Option<NodeIndex>>) -> Vec<NodeIndex> {
    // list for keeping track of nodes in their final order
    let mut order: Vec<NodeIndex> = Vec::with_capacity(gr.node_count());
    // list, synced with indicies of nodes, for keeping track of which nodes
    // have been visited
    let mut explored = vec![false; gr.node_count()];
    // DFS subroutine
    fn dfs_topo(
        gr: &Graph<usize, bool>,
        nodes: &Vec<Option<NodeIndex>>,
        order: &mut Vec<NodeIndex>,
        explored: &mut Vec<bool>,
        start_node: Option<NodeIndex>,
    ) {
        let start_node_index = nodes
            .iter()
            .position(|&x| x == start_node)
            .expect("NodeIndex doesn't exist in provided nodes array!");
        explored[start_node_index] = true;
        for edge in gr.edges(start_node.unwrap()) {
            let v = edge.target();
            let v_index = nodes.iter().position(|&x| x == Some(v)).unwrap();
            if !explored[v_index] {
                dfs_topo(gr, nodes, order, explored, Some(v));
            }
        }
        // give this node the current lowest avaliable indexing
        // it should have highest at the end, so we will reverse
        // the vector
        order.push(start_node.unwrap());
    }
    // main loop
    for node in nodes {
        let node_index = nodes.iter().position(|&x| x == *node).unwrap();
        if !explored[node_index] {
            dfs_topo(gr, nodes, &mut order, &mut explored, *node);
        }
    }
    // reverse the vector as noted above
    order.reverse();
    order
}
