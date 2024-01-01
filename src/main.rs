use rayon::prelude::*;
use std::collections::VecDeque;
use std::fmt;
use std::mem;
use std::time::Duration;

use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

mod render;

#[derive(Clone, Copy, PartialEq, Hash, Debug)]
enum Tile {
    Air { pressure: i8 },
    Sand { humidity: u8 },
    Water { pressure: i8 },
}

impl fmt::Display for Tile {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::Tile::*;
        match self {
            Air { .. } => fmt.write_str(" "),
            Sand { .. } => fmt.write_str("#"),
            Water { .. } => fmt.write_str("~"),
        }
    }
}

fn create_texels(size: usize) -> Vec<u8> {
    (0..size * size)
        .map(|id| {
            // get high five for recognizing this ;)
            let cx = 3.0 * (id % size) as f32 / (size - 1) as f32 - 2.0;
            let cy = 2.0 * (id / size) as f32 / (size - 1) as f32 - 1.0;
            let (mut x, mut y, mut count) = (cx, cy, 0);
            while count < 0xFF && x * x + y * y < 4.0 {
                let old_x = x;
                x = x * x - y * y + cx;
                y = 2.0 * old_x * y + cy;
                count += 1;
            }
            count
        })
        .collect()
}

pub async fn start() {
    let event_loop = EventLoop::new();
    let builder = winit::window::WindowBuilder::new();
    let window = builder.build(&event_loop).unwrap();
    window.set_title("hiekkapeli");

    let (device, queue, bind_group, surface, render_pipeline, mut config, texture) =
        render::init_rendering(&window).await;

    let mut prev = std::time::Instant::now();

    let mut samples = VecDeque::new();

    let mut n = 0;

    event_loop.run(move |event, _target, flow| {
        *flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                config.width = size.width;
                config.height = size.height;
                surface.configure(&device, &config);
                window.request_redraw();
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                }
                | WindowEvent::CloseRequested => {
                    *flow = ControlFlow::Exit;
                }
                _ => {}
            },
            Event::RedrawRequested(_window) => {
                render::render(&surface, &device, &render_pipeline, &bind_group, &queue);
            }
            Event::MainEventsCleared => {
                if n % 4 == 0 {
                    let texture_extent = texture.size();
                    let texels = create_texels(texture_extent.width as usize);
                    queue.write_texture(
                        texture.as_image_copy(),
                        &texels,
                        wgpu::ImageDataLayout {
                            offset: 0,
                            bytes_per_row: Some(texture_extent.width),
                            rows_per_image: None,
                        },
                        texture_extent,
                    );
                }

                let now = std::time::Instant::now();
                samples.push_back(now - prev);
                if samples.len() > 30 {
                    samples.pop_front();
                }
                if n > 30 {
                    let second = std::time::Duration::from_secs(1).as_nanos();
                    let mean = samples
                        .iter()
                        .cloned()
                        .fold(Duration::from_millis(0), |a, b| a + b)
                        .div_f32(samples.len() as f32);
                    let var = samples
                        .iter()
                        .cloned()
                        .map(|s| {
                            let m = if s > mean { s - mean } else { mean - s };
                            m.mul_f64(m.as_nanos() as f64 / second as f64)
                        })
                        .fold(Duration::from_millis(0), |a, b| a + b)
                        .div_f32(samples.len() as f32);
                    let fps = second as f64 / mean.as_nanos() as f64;
                    println!("{:<6?} ± {:?}", (fps * 100.).round() / 100., var);
                    n = 0;
                }
                n += 1;
                prev = now;
                window.request_redraw();
            }
            _ => {}
        }
    });
}

fn main() {
    pollster::block_on(start());
    let width = 211; //640;
    let height = 57; //480;

    let index = |x: usize, y: usize| x * height + y;

    let mut prev_buffer = vec![Tile::Air { pressure: 0 }; width * height];
    let mut cur_buffer = vec![Tile::Air { pressure: 0 }; width * height];

    for x in 0..width {
        prev_buffer[index(x, 0)] = Tile::Sand { humidity: 0 };
        prev_buffer[index(x, height - 1)] = Tile::Water { pressure: 0 };
    }
    for y in 0..height {
        prev_buffer[index(0, y)] = Tile::Sand { humidity: 0 };
        prev_buffer[index(width - 1, y)] = Tile::Water { pressure: 0 };
    }
    loop {
        // TODO: This allocation should be able to be reused
        let mut bufs = Vec::with_capacity(height);
        let mut buffer_slice = &mut cur_buffer[..];
        for _ in 0..width {
            let (cur, next) = buffer_slice.split_at_mut(height);
            buffer_slice = next;
            bufs.push(cur);
        }
        bufs.par_iter_mut().enumerate().for_each(|(x, b)| {
            for (y, t) in b.iter_mut().enumerate() {
                use self::Tile::*;
                match prev_buffer[index(x, y)] {
                    Air { pressure } => {
                        *t = Air { pressure };
                    }
                    Sand { humidity } => {
                        *t = Sand { humidity };
                    }
                    Water { pressure } => {
                        *t = Water { pressure };
                    }
                }
            }
        });
        mem::swap(&mut cur_buffer, &mut prev_buffer);

        for y in 0..height {
            for x in 0..width {
                print!("{}", prev_buffer[index(x, y)]);
            }
            println!();
        }
    }
}
