use ada::{shape, Canvas};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use verlet_rs::{behaviors, constraints, Spring2D, VerletObject2D, VerletPhysics2D};

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;

pub fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Verlet Demo - Simple Rope")
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
    // add behavior for gravity
    let gravity = behaviors::ConstantForceBehavior2D::new(0.0, 1.0);
    verlet_engine.add_behavior(gravity);
    // add rope object
    init_rope(&mut verlet_engine);

    // create canvas for rendering
    let mut canvas = Canvas::new(WIDTH as usize, HEIGHT as usize).unwrap();

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            draw_rope(&mut verlet_engine, &mut canvas, pixels.get_frame());
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

fn init_rope(verlet_phy: &mut VerletPhysics2D) {
    let gap = 10.0;
    let num_particles = 20;

    let mut rope = VerletObject2D::new("rope");

    for i in 0..num_particles {
        let p = rope.create_particle(200. + i as f32 * gap, 10.);
        if i == 0 {
            let pin_c = constraints::PinConstraint2D::new(&p);
            rope.add_constraint(pin_c);
        }
    }

    // add spring connections
    let mut xs: Vec<Spring2D> = Vec::with_capacity(num_particles);
    let particles = rope.get_particles();
    for i in 1..particles.len() {
        let s = Spring2D::new(particles[i - 1], particles[i], 25., 0.5);
        xs.push(s);
    }
    rope.add_springs(xs);
    verlet_phy.add_verlet_object(rope);
}

fn draw_rope(verlet_phy: &mut VerletPhysics2D, canvas: &mut Canvas, buffer: &mut [u8]) {
    canvas.clear(&ada::color::BLACK, buffer);
    let white = &ada::color::WHITE;

    for obj in verlet_phy.get_verlet_objects() {
        let particles = obj.get_particles();

        for particle in particles.iter() {
            let pp = particle.get_position();

            shape::draw_ellipse2d_filled(pp.x as i32, pp.y as i32, 6, 6, canvas, white, buffer);
        }
    }
}
