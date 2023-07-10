mod context;
mod texture;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.0, 0.0],
        texture_coords: [0.5, 0.5],
    },
    Vertex {
        position: [1.0, 0.0, 0.0],
        texture_coords: [1.0, 0.5],
    },
    Vertex {
        position: [0.49999997, 0.86602545, 0.0],
        texture_coords: [0.75, 0.9330127],
    },
    Vertex {
        position: [-0.50000006, 0.8660254, 0.0],
        texture_coords: [0.24999997, 0.9330127],
    },
    Vertex {
        position: [-1.0, 0.0, 0.0],
        texture_coords: [0.0, 0.5],
    },
    Vertex {
        position: [-0.4999999, -0.86602545, 0.0],
        texture_coords: [0.25000006, 0.066987276],
    },
    Vertex {
        position: [0.50000036, -0.8660252, 0.0],
        texture_coords: [0.7500002, 0.066987395],
    },
];

const INDICES: &[u16] = &[0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 5, 0, 5, 6, 0, 6, 1];

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    texture_coords: [f32; 2],
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Debug).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(winit::dpi::LogicalSize::new(400, 400))
        .build(&event_loop)
        .unwrap();
    window.set_title("Box Render");

    #[cfg(target_arch = "wasm32")]
    {
        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.
        use winit::dpi::PhysicalSize;
        window.set_inner_size(PhysicalSize::new(600, 600));

        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.body().expect("Couldn't get document body.");
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

    let mut context = context::Context::new(window).await;

    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(window_id) if window_id == context.window.id() => {
            context.update();
            match context.render() {
                Ok(_) => {}
                // Recreate the surface if lost
                Err(wgpu::SurfaceError::Lost) => context.resize(context.size),
                // The system is out of memory, we should probably quit
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            context.window.request_redraw();
        }
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == context.window.id() => {
            if !context.input(event) {
                match event {
                    WindowEvent::MouseWheel { delta, .. } => {
                        let mut color = context.background_color();
                        match delta {
                            MouseScrollDelta::PixelDelta(pos) => {
                                if pos.y > 0.0 {
                                    color.r = (color.r + 0.025).clamp(0.0, 1.0);
                                    color.g = (color.g + 0.025).clamp(0.0, 1.0);
                                    color.b = (color.b + 0.025).clamp(0.0, 1.0);
                                } else {
                                    color.r = (color.r - 0.025).clamp(0.0, 1.0);
                                    color.g = (color.g - 0.025).clamp(0.0, 1.0);
                                    color.b = (color.b - 0.025).clamp(0.0, 1.0);
                                }
                                context.set_background_color(color);
                                context.window.request_redraw();
                            }
                            MouseScrollDelta::LineDelta(_x, y) => {
                                if *y > 0.0 {
                                    color.r = (color.r + 0.025).clamp(0.0, 1.0);
                                    color.g = (color.g + 0.025).clamp(0.0, 1.0);
                                    color.b = (color.b + 0.025).clamp(0.0, 1.0);
                                } else {
                                    color.r = (color.r - 0.025).clamp(0.0, 1.0);
                                    color.g = (color.g - 0.025).clamp(0.0, 1.0);
                                    color.b = (color.b - 0.025).clamp(0.0, 1.0);
                                }
                                context.set_background_color(color);
                                context.window.request_redraw();
                            }
                        }
                    }
                    WindowEvent::Resized(physical_size) => {
                        context.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        context.resize(**new_inner_size);
                    }
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    _ => {}
                }
            }
        }
        _ => {}
    });
}
