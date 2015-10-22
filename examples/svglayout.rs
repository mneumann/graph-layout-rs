extern crate graph_layout;
extern crate graph_generators;
extern crate rand;

use rand::{random, Closed01};
use graph_layout::P2d;
use std::io::Write;

struct SvgCanvas {
    width: f32,
    height: f32,
    border: f32,
    radius: f32,
    scalex: f32,
    scaley: f32,
    offsetx: f32,
    offsety: f32,
}

struct SvgWriter {
    canvas: SvgCanvas,
    wr: Box<Write>,
}

impl SvgWriter {
    fn new(canvas: SvgCanvas, wr: Box<Write>) -> SvgWriter {
        SvgWriter{canvas: canvas, wr: wr}
    }

    fn header(&mut self) {
        writeln!(&mut self.wr, r#"<?xml version="1.0" encoding="UTF-8"?>
                <svg xmlns="http://www.w3.org/2000/svg"
                version="1.1" baseProfile="full"
                width="100%" height="100%"
                viewBox="{} {} {} {}">"#, 0, 0, self.canvas.width+2.0*self.canvas.border, self.canvas.height+2.0*self.canvas.border).unwrap();
    }

    fn footer(&mut self) {
        writeln!(&mut self.wr, "</svg>").unwrap();
    }

    fn node(&mut self, pos: &P2d) {
        let x = self.canvas.border + (pos.0*self.canvas.scalex) + self.canvas.offsetx;
        let y = self.canvas.border + (pos.1*self.canvas.scaley) + self.canvas.offsety;
        writeln!(&mut self.wr, r#"<circle cx="{}" cy="{}" r="{}" stroke="black" stroke-width="1px" fill="red" />"#,
                 x, y, self.canvas.radius).unwrap();
    }

    fn edge(&mut self, pos1: &P2d, pos2: &P2d) {
        let x1 = self.canvas.border + (pos1.0*self.canvas.scalex) + self.canvas.offsetx;
        let y1 = self.canvas.border + (pos1.1*self.canvas.scaley) + self.canvas.offsety;
        let x2 = self.canvas.border + (pos2.0*self.canvas.scalex) + self.canvas.offsetx;
        let y2 = self.canvas.border + (pos2.1*self.canvas.scaley) + self.canvas.offsety;

         writeln!(&mut self.wr, r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="black" stroke-width="1px" />"#,
                  x1, y1, x2, y2).unwrap();
    }
}


fn draw_graph(g: graph_generators::Graph, filename: &str, l: Option<f32>) {
    use std::fs::File;
    let mut node_positions: Vec<P2d> = g.nodes.iter().map(|_| P2d(random::<Closed01<f32>>().0, random::<Closed01<f32>>().0)).collect();
    let mut node_neighbors: Vec<Vec<usize>> = g.nodes.iter().map(|_| Vec::new()).collect();
    for &(src, dst) in g.edges.iter() {
        node_neighbors[src].push(dst);
    }

    graph_layout::typical_fruchterman_reingold_2d(l,
                                            &mut node_positions[..],
                                            &node_neighbors);

    let canvas = SvgCanvas{
        width:1000.0,
        height:1000.0,
        border:40.0,
        radius:10.0,
        scalex: 1000.0,
        scaley: 1000.0,
        offsetx: 0.0,
        offsety: 0.0};
    let mut svg_wr = SvgWriter::new(canvas, Box::new(File::create(filename).unwrap()));

    svg_wr.header();

    for (i, pos1) in node_positions.iter().enumerate() {
        for &n in node_neighbors[i].iter() {
            let pos2 = &node_positions[n];

             svg_wr.edge(&pos1, &pos2);
        }
    }

    for pos1 in node_positions.iter() {
        svg_wr.node(&pos1);
    }

    svg_wr.footer();
}

fn main() {
    let mut rng = rand::thread_rng();
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
        g.add_edge((i, (i+1)%n));
    }
    draw_graph(g, "circle_50.svg", Some(0.01));
}
