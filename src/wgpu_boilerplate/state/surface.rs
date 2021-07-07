use super::State;

impl State {
    pub fn create_surface(instance: &wgpu::Instance, window: &winit::window::Window) -> wgpu::Surface {
        // Create the surface to draw on (from window, which we get from winit)
        unsafe { instance.create_surface(window) }
    }
}
