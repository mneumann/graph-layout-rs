extern crate graph_layout;
extern crate graph_generators;
extern crate rand;

use rand::{Closed01, random};
use graph_layout::P2d;
use graph_layout::svg_writer::{SvgCanvas, SvgWriter};
use std::fs::File;

fn draw_graph(g: graph_generators::Graph, filename: &str, l: Option<f32>) {
    let mut node_positions: Vec<P2d> = g.nodes
                                        .iter()
                                        .map(|_| {
                                            P2d(random::<Closed01<f32>>().0,
                                                random::<Closed01<f32>>().0)
                                        })
                                        .collect();
    let mut node_neighbors: Vec<Vec<usize>> = g.nodes.iter().map(|_| Vec::new()).collect();
    for &(src, dst) in g.edges.iter() {
        node_neighbors[src].push(dst);
    }

    graph_layout::fruchterman_reingold::layout_typical_2d(l, &mut node_positions, &node_neighbors);

    let mut file = File::create(filename).unwrap();
    let svg_wr = SvgWriter::new(SvgCanvas::default_for_unit_layout(), &mut file);
    svg_wr.draw_graph(&node_positions, &node_neighbors, false);
}

fn main() {
    let mut rng = rand::thread_rng();

    let g = graph_generators::barabasi_albert_graph(&mut rng, 50, 1);
    draw_graph(g, "barabasi_albert_50_1.svg", Some(0.03));

    let g = graph_generators::barabasi_albert_graph(&mut rng, 20, 3);
    draw_graph(g, "barabasi_albert_20_3.svg", None);

    let mut g = graph_generators::Graph::new();
    let n1 = g.add_node();
    let n2 = g.add_node();
    let n3 = g.add_node();
    let n4 = g.add_node();
    g.add_edge((n1, n2));
    g.add_edge((n2, n3));
    g.add_edge((n3, n4));
    draw_graph(g, "line.svg", None);

    let mut g = graph_generators::Graph::new();
    let n1 = g.add_node();
    let n2 = g.add_node();
    let n3 = g.add_node();
    g.add_edge((n1, n2));
    g.add_edge((n2, n3));
    g.add_edge((n3, n1));
    draw_graph(g, "triad.svg", None);

    let mut g = graph_generators::Graph::new();
    let n1 = g.add_node();
    let n2 = g.add_node();
    let n3 = g.add_node();
    let n4 = g.add_node();
    g.add_edge((n1, n2));
    g.add_edge((n2, n3));
    g.add_edge((n3, n4));
    g.add_edge((n4, n1));
    draw_graph(g, "square.svg", None);

    let mut g = graph_generators::Graph::new();
    let n1 = g.add_node();
    let n2 = g.add_node();
    let n3 = g.add_node();
    let n4 = g.add_node();
    g.add_edge((n1, n2));
    g.add_edge((n2, n3));
    g.add_edge((n3, n4));
    g.add_edge((n4, n1));
    g.add_edge((n1, n3));
    g.add_edge((n2, n4));
    draw_graph(g, "connected_square.svg", None);

    let mut g = graph_generators::Graph::new();
    let n = 100;
    for _ in 0..n {
        let _ = g.add_node();
    }
    for i in 0..n {
        g.add_edge((i, (i + 1) % n));
    }
    draw_graph(g, "circle_50.svg", Some(0.01));
}
