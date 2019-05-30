use wgpu::winit::{
        ElementState,
        Event,
        EventsLoop,
        KeyboardInput,
        VirtualKeyCode,
        Window,
        WindowEvent,
    };

fn start() {
    let instance = wgpu::Instance::new();
    let adapter = instance.get_adapter(&wgpu::AdapterDescriptor {
        power_preference: wgpu::PowerPreference::LowPower,
    });
    let mut device = adapter.create_device(&wgpu::DeviceDescriptor {
        extensions: wgpu::Extensions {
            anisotropic_filtering: false,
        },
        limits: wgpu::Limits::default(),
    });
    let mut events_loop = EventsLoop::new();
    let window = Window::new(&events_loop).unwrap();
    window.set_title(title);
    let size = window
        .get_inner_size()
        .unwrap()
        .to_physical(window.get_hidpi_factor);
    let surface = instance.create_surface(&window);
    let mut sc_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::resource::
    }
}