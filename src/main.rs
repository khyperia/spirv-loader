pub async fn create_device() -> (wgpu::Device, wgpu::Queue) {
    let instance = wgpu::Instance::new(wgpu::Backends::VULKAN);
    let adapter = instance.request_adapter(&Default::default()).await.unwrap();
    let dd = wgpu::DeviceDescriptor {
        label: None,
        features: wgpu::Features::SPIRV_SHADER_PASSTHROUGH,
        limits: Default::default(),
    };
    adapter.request_device(&dd, None).await.unwrap()
}
fn main() {
    env_logger::init();
    let (device, _queue) = futures::executor::block_on(create_device());
    for path in std::env::args().skip(1) {
        println!("{}", path);
        let spirv = std::fs::read(path).unwrap();
        let _module = unsafe {
            device.create_shader_module_spirv(&wgpu::ShaderModuleDescriptorSpirV {
                label: None,
                source: wgpu::util::make_spirv_raw(&spirv),
            })
        };
    }
}
