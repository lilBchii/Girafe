#![allow(clippy::single_match)]

/*use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};*/

use std::path::Path;
use whisper_rs::WhisperContext;

mod record;
mod transcribe;

fn main() {
    record::start_recording(Path::new("assets/rec.wav"));
    let ctx = WhisperContext::new("models/ggml-small.bin").expect("failed to load model");
    let text = transcribe::transcribe(Path::new("assets/rec.wav"), &ctx);
    println!("{:?}", text);
}

/*fn main() -> Result<(), impl std::error::Error> {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Wait);

    let window = WindowBuilder::new()
        .with_title("Test")
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, elwt| {
        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::MouseWheel { delta, .. } => match delta {
                    winit::event::MouseScrollDelta::LineDelta(x, y) => {
                        println!("mouse wheel Line Delta: ({x},{y})");
                        let pixels_per_line = 120.0;
                        let mut pos = window.outer_position().unwrap();
                        pos.x += (x * pixels_per_line) as i32;
                        pos.y += (y * pixels_per_line) as i32;
                        window.set_outer_position(pos)
                    }
                    winit::event::MouseScrollDelta::PixelDelta(p) => {
                        println!("mouse wheel Pixel Delta: ({},{})", p.x, p.y);
                        let mut pos = window.outer_position().unwrap();
                        pos.x += p.x as i32;
                        pos.y += p.y as i32;
                        window.set_outer_position(pos)
                    }
                },
                WindowEvent::RedrawRequested => {
                    //fill_window(&window);
                }
                _ => (),
            }
        }
    })
}
*/
