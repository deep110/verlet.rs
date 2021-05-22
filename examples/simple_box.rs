extern crate verlet_rs;

use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use verlet_rs::VerletPhysics2D;

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;

pub fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Verlet Demo - Simple Box")
            .with_inner_size(size)
            .with_resizable(false)
            .with_always_on_top(true)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
    };

    // initialize verlet engine
    let mut verlet_engine = VerletPhysics2D::new();

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            if pixels
                .render()
                .map_err(|e| println!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            // Update internal state and request a redraw
            verlet_engine.update();
            window.request_redraw();
        }
    });
}
