use wgpu::util::DeviceExt;

use super::State;
impl State {
    pub fn update(&mut self) {
        self.uniforms.update_view_proj(&self.camera);
        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        );
    }
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let frame = self.surface.get_current_texture()?;
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            {
                if self.background.should_clear {
                    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(self.background.color),
                                store: true,
                            },
                        }],
                        depth_stencil_attachment: None,
                    });
                    self.background.reset();
                }

                {
                    // Not sure which one is better
                    self.vertex_buffer =
                        self.device
                            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                                label: None,
                                contents: bytemuck::cast_slice(&self.vertices),
                                usage: wgpu::BufferUsages::VERTEX,
                            });
                }
                {
                    // Not sure which one is better
                    self.index_buffer =
                        self.device
                            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                                label: None,
                                contents: bytemuck::cast_slice(&self.indices),
                                usage: wgpu::BufferUsages::INDEX,
                            });
                }

                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: true,
                        },
                    }],
                    depth_stencil_attachment: None,
                });

                render_pass.set_pipeline(&self.render_pipeline);
                // render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
                // Index is 1 since it's the second
                render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);

                render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
                render_pass
                    .set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..self.indices.len() as u32, 0, 0..1);
                // Clear buffer
                self.indices.clear();
                self.vertices.clear();
            }

            self.font_interface
                .draw(&self.device, &mut encoder, self.size, &view);
            self.font_interface.finish();
        }
        self.queue.submit(Some(encoder.finish()));
        frame.present();
        Ok(())
    }

    /// Get a reference to the state's size.
    pub fn size(&self) -> &winit::dpi::PhysicalSize<u32> {
        &self.size
    }
}

impl State {
    fn update_config(&mut self) {
        self.config.width = self.size.width;
        self.config.height = self.size.height;

        self.surface.configure(&self.device, &self.config)
    }
    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.size = size;
        // Update swap chain description based off new size
        self.update_config();
        // update swap chain based of new swap description
        // self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
        // Update other
        self.camera.aspect_ratio = (size.width as f32 / size.height as f32) / 2.0;
        // println!("{}x{}, AR: {}", self.size.width, self.size.height, self.size.width as f32 / self.size.height as f32);
        // println!("CAMERA AR: {}", self.camera.aspect_ratio);
    }
}
