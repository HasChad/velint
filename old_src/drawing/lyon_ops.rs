use lyon::path::{LineCap, LineJoin, Path};
use lyon::tessellation::{
    BuffersBuilder, FillOptions, FillTessellator, FillVertex, StrokeOptions, StrokeTessellator,
    StrokeVertex, VertexBuffers,
};
use macroquad::prelude::*;

pub struct LyonOpsLine {
    pub geometry: VertexBuffers<[f32; 2], u16>,
    pub vertices: Vec<Vertex>,
}

impl LyonOpsLine {
    pub fn new(path: &Path, color: Color, width: f32) -> Self {
        // Tessellate into triangles
        let mut geometry: VertexBuffers<[f32; 2], u16> = VertexBuffers::new();
        let mut tessellator = StrokeTessellator::new();

        tessellator
            .tessellate_path(
                path,
                &StrokeOptions::default()
                    .with_line_width(width)
                    .with_line_cap(LineCap::Round)
                    .with_line_join(LineJoin::Round),
                &mut BuffersBuilder::new(&mut geometry, |vertex: StrokeVertex| {
                    vertex.position().to_array()
                }),
            )
            .unwrap();

        // Convert into Macroquad Mesh
        let vertices: Vec<Vertex> = geometry
            .vertices
            .iter()
            .map(|[x, y]| Vertex {
                position: Vec3::new(*x, *y, 0.0),
                uv: Vec2::ZERO,
                color: color.into(),
                normal: Vec4::ZERO,
            })
            .collect();

        LyonOpsLine { geometry, vertices }
    }
}

pub struct LyonOpsFill {
    pub geometry: VertexBuffers<[f32; 2], u16>,
    pub vertices: Vec<Vertex>,
}

impl LyonOpsFill {
    pub fn new(path: &Path, color: Color) -> Self {
        // Tessellate into triangles
        let mut geometry: VertexBuffers<[f32; 2], u16> = VertexBuffers::new();
        let mut tessellator = FillTessellator::new();

        tessellator
            .tessellate_path(
                path,
                &FillOptions::default(),
                &mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex| {
                    vertex.position().to_array()
                }),
            )
            .unwrap();

        // Convert into Macroquad Mesh
        let vertices: Vec<Vertex> = geometry
            .vertices
            .iter()
            .map(|[x, y]| Vertex {
                position: Vec3::new(*x, *y, 0.0),
                uv: Vec2::ZERO,
                color: color.into(),
                normal: Vec4::ZERO,
            })
            .collect();

        LyonOpsFill { geometry, vertices }
    }
}
