pub struct Vertex {
  position: [f32; 3],
}

pub struct Model<'a> {
  vertex_buffer: wgpu::Buffer,
  render_pipeline: &'a wgpu::RenderPipeline,
}
