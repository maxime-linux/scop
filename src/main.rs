use winit::event_loop::EventLoop;

use crate::scop::Scop;

mod scop;

fn main() {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    let mut scop = Scop::new();

    event_loop.run_app(&mut scop);
}
