use std::error::Error;
use winit;

pub struct VulkanApp {}

impl VulkanApp {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {})
    }
    pub fn run(&self) {
        println!("scop run!");
    }
}
