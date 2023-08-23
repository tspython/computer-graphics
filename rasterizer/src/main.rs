use winit::{
    event_loop::EventLoop,
    window::WindowBuilder,
};

mod rendering;
mod geometry;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let context = rendering::Context::new(&window);

    event_loop.run(|event, _, control_flow| {
    });
}
