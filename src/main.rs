use vulkan::VulkanApp;

mod vulkan;

fn main() {
    if let Ok(mut vulkan_app) = VulkanApp::new() {
        match vulkan_app.run() {
            Err(err) => println!("Err:{err}"),
            _ => println!("scop closed correctely"),
        }
    }
}
