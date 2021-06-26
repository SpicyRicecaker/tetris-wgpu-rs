use wgpu::{
    BackendBit, DeviceDescriptor, Features, Instance, Limits, PowerPreference,
    RequestAdapterOptionsBase,
};
use winit::window::Window;

use futures::executor::block_on;

pub fn init(raw_window_handle: &Window) {
    // First create the wgpu instance, choosing the primary backend
    let instance = Instance::new(BackendBit::PRIMARY);

    // Create the surface to draw on (from window, which we get from winit)
    let surface;
    unsafe {
        surface = instance.create_surface(raw_window_handle);
    };

    // Create the adapter
    let request_adapter_options = RequestAdapterOptionsBase {
        power_preference: PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
    };
    let adapter = block_on(instance.request_adapter(&request_adapter_options)).unwrap();

    // Create the device from adapter
    let desc = DeviceDescriptor {
        label: Some("gpu"),
        features: Features::default(),
        limits: Limits::default(),
    };
    let trace_path = None;
    let (device, queue) = block_on(adapter.request_device(&desc, trace_path)).unwrap();
}
