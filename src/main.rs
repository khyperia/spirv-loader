pub async fn create_device() -> (wgpu::Device, wgpu::Queue) {
    let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
    let adapter = instance.request_adapter(&Default::default()).await.unwrap();
    adapter
        .request_device(&Default::default(), None)
        .await
        .unwrap()
}
unsafe fn u8_as_u32_slice(p: &[u8]) -> &[u32] {
    std::slice::from_raw_parts(p.as_ptr() as *const u32, p.len() / 4)
}
fn main() {
    env_logger::init();
    let (device, _queue) = futures::executor::block_on(create_device());
    for path in std::env::args().skip(1) {
        println!("{}", path);
        let spirv = std::fs::read(path).unwrap();
        let _module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::SpirV(unsafe { u8_as_u32_slice(&spirv) }.into()),
            flags: wgpu::ShaderFlags::VALIDATION,
        });
    }
}
