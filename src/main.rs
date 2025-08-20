use vulkan::VulkanApp;

mod vulkan;

fn main() {
    let mut vulkan_app = VulkanApp::new();
    match vulkan_app.run() {
        Err(err) => eprintln!("Err:{err}"),
        _ => println!("scop closed correctely"),
    }
}
