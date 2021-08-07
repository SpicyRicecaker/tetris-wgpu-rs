use super::State;
use crate::{wgpu_boilerplate::buffers::Vertex, World};
impl State {
    pub fn input(&mut self, event: &winit::event::WindowEvent, world: &mut World) -> bool {
        let mut bool = false;
        // what the fk is this code?? who the fk wrote this sht?
        bool = bool || world.controller.process_events(event);
        bool = bool || self.camera_controller.process_events(event);
        bool
    }

    pub fn update(&mut self) {
        self.camera_controller.update_camera(&mut self.camera);
        self.uniforms.update_view_proj(&self.camera);
        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        );
    }
    pub fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        let frame = self.swap_chain.get_current_frame()?.output;
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
                            view: &frame.view,
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

                while let Some(shape) = self.buffer_queue.pop_front() {
                    // Create staging buffer for vertex
                    // slot = what buffer slot to use for buffer (can have mult buffers)
                    // 2nd = slice of buffer to use
                    {
                        let vertex_buffer = Self::create_buffer(
                            &self.device,
                            None,
                            bytemuck::cast_slice(shape.vertices.as_slice()),
                            wgpu::BufferUsage::COPY_SRC,
                        );
                        self.num_vertices = shape.vertices.len() as u32;
                        // Copy stuff over
                        encoder.copy_buffer_to_buffer(
                            &vertex_buffer,
                            0,
                            &self.vertex_buffer,
                            0,
                            std::mem::size_of::<Vertex>() as u64
                                * self.num_vertices as wgpu::BufferAddress, // std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress
                        );
                    }
                    {
                        let index_buffer = Self::create_buffer(
                            &self.device,
                            None,
                            bytemuck::cast_slice(shape.indices.as_slice()),
                            wgpu::BufferUsage::COPY_SRC,
                        );
                        self.num_indices = shape.indices.len() as u32;
                        // Copy stuff over
                        encoder.copy_buffer_to_buffer(
                            &index_buffer,
                            0,
                            &self.index_buffer,
                            0,
                            std::mem::size_of::<u16>() as u64
                                * self.num_indices as wgpu::BufferAddress, // std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress
                        );
                    }

                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Render Pass"),
                        color_attachments: &[wgpu::RenderPassColorAttachment {
                            view: &frame.view,
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
                    render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
                }
            }

            self.font_interface
                .draw(&self.device, &mut encoder, self.size, &frame);
            self.font_interface.finish();
        }
        self.queue.submit(Some(encoder.finish()));
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
