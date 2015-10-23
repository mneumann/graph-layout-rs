use std::io::Write;
use super::P2d;

pub struct SvgCanvas {
    pub width: f32,
    pub height: f32,
    pub border: f32,
    pub radius: f32,
    pub scalex: f32,
    pub scaley: f32,
    pub offsetx: f32,
    pub offsety: f32,
    pub stroke_width: f32,
    pub stroke_color: String,
    pub fill_color: String,
}

impl SvgCanvas {
    /// If all graph coordinates are in [0,1].
    pub fn default_for_unit_layout() -> SvgCanvas {
        SvgCanvas{
            width:1000.0,
            height:1000.0,
            border: 40.0,
            radius: 10.0,
            scalex: 1000.0,
            scaley: 1000.0,
            offsetx: 0.0,
            offsety: 0.0,
            stroke_width: 1.0,
            stroke_color: "black".to_string(),
            fill_color: "red".to_string(),
        }
    }
}

pub struct SvgWriter<'a> {
    canvas: SvgCanvas,
    wr: &'a mut Write,
}

impl<'a> SvgWriter<'a> {
    pub fn new<'b>(canvas: SvgCanvas, wr: &'b mut Write) -> SvgWriter<'b> {
        SvgWriter{canvas: canvas, wr: wr}
    }

    pub fn header(&mut self) {
        writeln!(&mut self.wr, r#"<?xml version="1.0" encoding="UTF-8"?>
                <svg xmlns="http://www.w3.org/2000/svg"
                version="1.1" baseProfile="full"
                width="100%" height="100%"
                viewBox="{} {} {} {}">"#, 0, 0, self.canvas.width+2.0*self.canvas.border, self.canvas.height+2.0*self.canvas.border).unwrap();
    }

    pub fn footer(&mut self) {
        writeln!(&mut self.wr, "</svg>").unwrap();
    }

    pub fn node(&mut self, pos: &P2d) {
        let x = self.canvas.border + (pos.0*self.canvas.scalex) + self.canvas.offsetx;
        let y = self.canvas.border + (pos.1*self.canvas.scaley) + self.canvas.offsety;
        writeln!(&mut self.wr, r#"<circle cx="{}" cy="{}" r="{}" stroke="{}" stroke-width="{}px" fill="{}" />"#,
                 x, y, self.canvas.radius, self.canvas.stroke_color, self.canvas.stroke_width, self.canvas.fill_color).unwrap();
    }

    pub fn edge(&mut self, pos1: &P2d, pos2: &P2d) {
        let x1 = self.canvas.border + (pos1.0*self.canvas.scalex) + self.canvas.offsetx;
        let y1 = self.canvas.border + (pos1.1*self.canvas.scaley) + self.canvas.offsety;
        let x2 = self.canvas.border + (pos2.0*self.canvas.scalex) + self.canvas.offsetx;
        let y2 = self.canvas.border + (pos2.1*self.canvas.scaley) + self.canvas.offsety;

         writeln!(&mut self.wr, r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="{}px" />"#,
                  x1, y1, x2, y2, self.canvas.stroke_color, self.canvas.stroke_width).unwrap();
    }
}
