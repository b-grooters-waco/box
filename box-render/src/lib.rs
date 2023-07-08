mod context;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const DOC_ELEMENT_ID: &str = "planogram-canvas";

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
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
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [1.0, 0.0, 0.0],
    }, // A
    Vertex {
        position: [0.5, -0.5, 0.0],
        color: [0.0, 1.0, 0.0],
    }, // B
    Vertex {
        position: [0.0, 0.5, 0.0],
        color: [0.0, 0.0, 1.0],
    }, // C
];

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
    let window = WindowBuilder::new().build(&event_loop).unwrap();

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
                            _ => {}
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
