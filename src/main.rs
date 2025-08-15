mod vulkan;

fn main() {
    match vulkan::VulkanApp::new() {
        Ok(app) => app.run(),
        Err(err) => println!("VulkanApp::new() => {err}"),
    }
}
