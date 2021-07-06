use super::State;

// Challenge for movign cursor
impl State {
    pub fn render_background(
        &mut self,
        r: f64,
        g: f64,
        b: f64,
        a: f64,
    ) -> Result<(), wgpu::SwapChainError> {
        let frame = self.swap_chain.get_current_frame()?.output;
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color { r, g, b, a }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
        }

        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }

    pub fn swap_render_pipeline(&mut self) {
        match self.selected_rd_pipeline_idx {
            1 => {
                self.selected_rd_pipeline_idx = 0;
            }
            0 => {
                self.selected_rd_pipeline_idx = 1;
            }
            _ => (),
        };
    }
}
