use std::error::Error;

struct VulkanApp;

impl VulkanApp {
    fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self)
    }
    fn run(&mut self) {
        println!("Running application")
    }
}
fn main() {
    match VulkanApp::new() {
        Ok(mut app) => app.run(),
        Err(err) => eprintln!("Failed to create application: {err}"),
    }
}
