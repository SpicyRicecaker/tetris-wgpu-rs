use wgpu_glyph::{
    ab_glyph::{self, FontArc},
    GlyphBrush, GlyphBrushBuilder, Section,
};
pub struct FontInterface {
    staging_belt: wgpu::util::StagingBelt,
    glyph_brush: GlyphBrush<()>,
}

impl FontInterface {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        // Default font, let's use visitor
        let visitor = ab_glyph::FontArc::try_from_slice(include_bytes!(
            "..\\..\\..\\resources\\visitor2.ttf"
        ))
        .unwrap();
        let glyph_brush = GlyphBrushBuilder::using_font(visitor).build(device, format);
        let staging_belt = wgpu::util::StagingBelt::new(1024);

        Self {
            glyph_brush,
            staging_belt,
        }
    }
    pub fn add_font(&mut self, font: FontArc) {
        self.glyph_brush.add_font(font);
    }
    pub fn finish(&mut self) {
        self.staging_belt.finish()
    }
    #[inline]
    pub fn queue(&mut self, section: Section) {
        self.glyph_brush.queue(section)
    }

    pub fn draw(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        size: winit::dpi::PhysicalSize<u32>,
        frame: &wgpu::TextureView,
    ) {
        self.glyph_brush
            .draw_queued(
                device,
                &mut self.staging_belt,
                encoder,
                frame,
                size.width,
                size.height,
            )
            .expect("Draw queued");
    }
}
