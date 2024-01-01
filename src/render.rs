use std::borrow::Cow;

use wgpu::{
    Backends, Features, FragmentState, InstanceDescriptor, InstanceFlags, Limits, MultisampleState,
    PrimitiveState, ShaderModuleDescriptor, VertexState,
};
use winit::window::Window;


// fn create_texels(size: usize) -> Vec<u8> {
//     (0..size * size)
//         .map(|id| {
//             // get high five for recognizing this ;)
//             let cx = 3.0 * (id % size) as f32 / (size - 1) as f32 - 2.0;
//             let cy = 2.0 * (id / size) as f32 / (size - 1) as f32 - 1.0;
//             let (mut x, mut y, mut count) = (cx, cy, 0);
//             while count < 0xFF && x * x + y * y < 4.0 {
//                 let old_x = x;
//                 x = x * x - y * y + cx;
//                 y = 2.0 * old_x * y + cy;
//                 count += 1;
//             }
//             count
//         })
//         .collect()
// }

pub async fn init_rendering(window: &Window) -> (
    wgpu::Device,
    wgpu::Queue,
    wgpu::BindGroup,
    wgpu::Surface,
    wgpu::RenderPipeline,
    wgpu::SurfaceConfiguration,
    wgpu::Texture
) {
    let ctx = InstanceDescriptor {
        // TODO: Also browser?
        backends: Backends::VULKAN,
        flags: InstanceFlags::DEBUG,
        // TODO: We don't care about this, right?
        dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
        // TODO: Do we care?
        gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
    };
    let instance = wgpu::Instance::new(ctx);
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            // TODO: What kind of surface we need?
            compatible_surface: None,
        })
        .await
        .expect("We should have adapter");
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: Some("hiekkapeli_dd"),
                features: Features::empty(),
                limits: Limits::downlevel_defaults(),
            },
            None,
        )
        .await
        .expect("We should have device");

    let size = 256u32;
    // let texels = create_texels(size as usize);
    let texture_extent = wgpu::Extent3d {
        width: size,
        height: size,
        depth_or_array_layers: 1,
    };
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("hiekkapeli_t"),
        size: texture_extent,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::R8Uint,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    // queue.write_texture(
    //     texture.as_image_copy(),
    //     &texels,
    //     wgpu::ImageDataLayout {
    //         offset: 0,
    //         bytes_per_row: Some(size),
    //         rows_per_image: None,
    //     },
    //     texture_extent,
    // );

    let module = device.create_shader_module(ShaderModuleDescriptor {
        label: Some("hiekkapeli_shader"),
        // TODO: Make this more dynamic?
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("pixel.wgsl"))),
    });
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("hiekkapeli_bgl"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                multisampled: false,
                sample_type: wgpu::TextureSampleType::Uint,
                view_dimension: wgpu::TextureViewDimension::D2,
            },
            count: None,
        }],
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::TextureView(&texture_view),
        }],
        label: None,
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("hiekkapeli_pl"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let size = window.inner_size();
    let surface =
        unsafe { instance.create_surface(window) }.expect("We should be able to create surface");
    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swapchain_format = swapchain_capabilities.formats[0];
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("hiekkapeli_p"),
        layout: Some(&pipeline_layout),
        vertex: VertexState {
            module: &module,
            entry_point: "vs_main",
            buffers: &[],
        },
        fragment: Some(FragmentState {
            module: &module,
            entry_point: "fs_main",
            targets: &[Some(swapchain_format.into())],
        }),
        primitive: PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleStrip,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Cw,
            cull_mode: None,
            unclipped_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        },
        depth_stencil: None,
        multisample: MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    });

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: swapchain_capabilities.alpha_modes[0],
        view_formats: vec![],
    };

    surface.configure(&device, &config);
    (
        device,
        queue,
        bind_group,
        surface,
        render_pipeline,
        config,
        texture,
    )
}

pub fn render(
    surface: &wgpu::Surface,
    device: &wgpu::Device,
    render_pipeline: &wgpu::RenderPipeline,
    bind_group: &wgpu::BindGroup,
    queue: &wgpu::Queue,
) {
    let frame = surface
        .get_current_texture()
        .expect("Failed to acquire next swap chain texture");
    let view = frame
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("hiekkapeli_render"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        rpass.set_pipeline(&render_pipeline);
        rpass.set_bind_group(0, &bind_group, &[]);
        rpass.draw(0..6, 0..1);
    }

    queue.submit(Some(encoder.finish()));
    frame.present();
}
