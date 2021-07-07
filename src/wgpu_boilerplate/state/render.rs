use super::State;
use crate::World;
impl State {
    pub fn input(&mut self, event: &winit::event::WindowEvent, world: &mut World) -> bool {
        // return false;
        match event {
            winit::event::WindowEvent::CursorMoved { position, .. } => {
                *world.cursor_pos_mut() = (position.x, position.y);
                true
            }
            _ => false,
        }
    }

    pub fn update(&mut self) {}
    pub fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        let frame = self.swap_chain.get_current_frame()?.output;
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipelines[self.selected_rd_pipeline_idx]);
            // slot = what buffer slot to use for buffer (can have mult buffers)
            // 2nd = slice of buffer to use
            render_pass
                .set_vertex_buffer(0, self.vertex_buffers[self.selected_buffer_idx].slice(..));
            render_pass.set_index_buffer(
                self.index_buffers[self.selected_buffer_idx].slice(..),
                wgpu::IndexFormat::Uint16,
            );
            render_pass.draw_indexed(0..self.num_indices[self.selected_buffer_idx], 0, 0..1);
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        Ok(())
    }

    /// Get a reference to the state's size.
    pub fn size(&self) -> &winit::dpi::PhysicalSize<u32> {
        &self.size
    }
}

impl State {
    fn update_sc_size(&mut self) {
        self.sc_desc.width = self.size.width;
        self.sc_desc.height = self.size.height;
    }
    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.size = size;
        // Update swap chain description based off new size
        self.update_sc_size();
        // update swap chain based of new swap description
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }
}