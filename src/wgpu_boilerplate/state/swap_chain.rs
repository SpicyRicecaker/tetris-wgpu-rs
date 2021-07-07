use super::State;
impl State {
    pub fn create_swap_chain_descriptor(
        surface: &wgpu::Surface,
        size: &winit::dpi::PhysicalSize<u32>,
        adapter: &wgpu::Adapter,
    ) -> wgpu::SwapChainDescriptor {
        wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: adapter.get_swap_chain_preferred_format(surface).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        }
    }
    pub fn create_swap_chain(
        sc_desc: &wgpu::SwapChainDescriptor,
        surface: &wgpu::Surface,
        device: &wgpu::Device,
    ) -> wgpu::SwapChain {
        device.create_swap_chain(surface, sc_desc)
    }
}
