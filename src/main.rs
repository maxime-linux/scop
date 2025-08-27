use std::error::Error;

use winit::{event_loop::EventLoop, raw_window_handle::HasDisplayHandle};

use std::ffi::c_char;

mod vulkan;
mod window;

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new()?;

    let extension_platform =
        ash_window::enumerate_required_extensions(event_loop.display_handle().unwrap().as_raw())?;

    let mut extensions: Vec<*const c_char> = extension_platform.to_vec();
    extensions.push(ash::ext::debug_utils::NAME.as_ptr());

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    // let mut scop: Scop = Scop::default();

    event_loop.run_app(&mut scop);

    Ok(())
}
