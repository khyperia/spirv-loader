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
const VERT_SHADER: &[u32] = &[
    0x02030723, 0x00000001, 0x00000007, 0x00050000, 0x00000000, 0x00110002, 0x00010000, 0x000e0003,
    0x00000000, 0x00000000, 0x000f0005, 0x00000000, 0x00010000, 0x616d6e69, 0x00000000, 0x00130002,
    0x00020000, 0x00210003, 0x00030000, 0x00020000, 0x00360005, 0x00020000, 0x00010000, 0x00000000,
    0x00030000, 0x00f80002, 0x00040000, 0x00fd0001, 0x00380001,
];
fn main() {
    env_logger::init();
    let (device, _queue) = futures::executor::block_on(create_device());
    for path in std::env::args().skip(1) {
        println!("{}", path);
        let spirv = std::fs::read(path).unwrap();
        let bin = rspirv::dr::load_bytes(&spirv).unwrap();
        let entry = bin
            .entry_points
            .iter()
            .find(|i| i.class.opcode == rspirv::spirv::Op::EntryPoint)
            .unwrap();
        let model = entry.operands[0].unwrap_execution_model();
        let name = entry.operands[2].unwrap_literal_string();
        let module = unsafe {
            device.create_shader_module_spirv(&wgpu::ShaderModuleDescriptorSpirV {
                label: None,
                source: wgpu::util::make_spirv_raw(&spirv),
            })
        };
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[/*wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::StorageTexture {
                    access: wgpu::StorageTextureAccess::ReadWrite,
                    format: wgpu::TextureFormat::Rgba32Float,
                    view_dimension: wgpu::TextureViewDimension::D2,
                },
                count: None,
            }*/],
        });
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });
        match model {
            rspirv::spirv::ExecutionModel::Vertex => {
                let _pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: None,
                    layout: Some(&render_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &module,
                        entry_point: name,
                        buffers: &[],
                    },
                    primitive: Default::default(),
                    depth_stencil: None,
                    multisample: Default::default(),
                    fragment: None,
                });
            }
            rspirv::spirv::ExecutionModel::Fragment => {
                let vert = unsafe {
                    device.create_shader_module_spirv(&wgpu::ShaderModuleDescriptorSpirV {
                        label: None,
                        source: VERT_SHADER.into(),
                    })
                };
                let _pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: None,
                    layout: Some(&render_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &vert,
                        entry_point: "main",
                        buffers: &[],
                    },
                    primitive: Default::default(),
                    depth_stencil: None,
                    multisample: Default::default(),
                    fragment: Some(wgpu::FragmentState {
                        module: &module,
                        entry_point: name,
                        targets: &[wgpu::ColorTargetState {
                            format: wgpu::TextureFormat::Rgba8Unorm,
                            blend: None,
                            write_mask: wgpu::ColorWrites::ALL,
                        }],
                    }),
                });
            }
            _ => println!("Skipping pipeline creation, not vert or frag"),
        }
    }
}
