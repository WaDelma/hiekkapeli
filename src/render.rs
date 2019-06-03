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
        usage: wgpu::TextureUsageFlags::OUTPUT_ATTACHMENT,
        format: wgpu::TextureFormat::Rgba8Unorm,
        width: size.width.round() as u32,
        height: size.height.round() as u32,
    };
    let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);
    let mut running = true;
    while running {
        events_loop.poll_events(|event| match event {
            Event::WindowEvent {
               event: WindowEvent::Resized(size),
            } => {

            },
            Event::WindowEvent {event, ..} => match event {
                WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        state: ElementState::Pressed,
                        ..
                    },
                    ..
                }
                | WindowEvent::CloseRequested => {
                    running = false;
                }
                _ => {
                    //something
                }
            },
            _ => (),
        });
        let frame = swap_chain.get_next_texture();

    }
}