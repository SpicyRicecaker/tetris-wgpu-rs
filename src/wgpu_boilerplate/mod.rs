pub mod challenges;
use winit::{dpi::PhysicalSize, event::WindowEvent, window::Window};

pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    size: PhysicalSize<u32>,
}

impl State {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // First create the wgpu instance, choosing the primary backend
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

        // Create the surface to draw on (from window, which we get from winit)
        let surface = unsafe { instance.create_surface(window) };

        // Create the adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        // Create the device from adapter
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .unwrap();

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: adapter.get_swap_chain_preferred_format(&surface).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
        }
    }
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {}
    // pub fn render(&mut self, r: f64, g: f64, b: f64, a: f64) -> Result<(), wgpu::SwapChainError> {
    //     let frame = self.swap_chain.get_current_frame()?.output;
    //     let mut encoder = self
    //         .device
    //         .create_command_encoder(&wgpu::CommandEncoderDescriptor {
    //             label: Some("Render Encoder"),
    //         });

    //     {
    //         let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    //             label: Some("Render Pass"),
    //             color_attachments: &[wgpu::RenderPassColorAttachment {
    //                 view: &frame.view,
    //                 resolve_target: None,
    //                 ops: wgpu::Operations {
    //                     load: wgpu::LoadOp::Clear(wgpu::Color { r, g, b, a }),
    //                     store: true,
    //                 },
    //             }],
    //             depth_stencil_attachment: None,
    //         });
    //     }

    //     self.queue.submit(std::iter::once(encoder.finish()));

    //     Ok(())
    // }

    /// Get a reference to the state's size.
    pub fn size(&self) -> &PhysicalSize<u32> {
        &self.size
    }
}

impl State {
    fn update_sc_size(&mut self) {
        self.sc_desc.width = self.size.width;
        self.sc_desc.height = self.size.height;
    }
    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size = size;
        // Update swap chain description based off new size
        self.update_sc_size();
        // update swap chain based of new swap description
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }
}
