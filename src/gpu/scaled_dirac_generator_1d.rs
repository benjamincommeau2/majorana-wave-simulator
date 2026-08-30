use std::num::NonZeroU64;

use wgpu::util::DeviceExt;

const WORKGROUP_SIZE: u32 = 64;

const GENERATOR_PARAMETERS_SIZE: u64 = 16;

pub struct GpuScaledDiracGenerator1d {

  mass_profile_buffer: wgpu::Buffer,

  _parameter_buffer: wgpu::Buffer,

  bind_group_layout: wgpu::BindGroupLayout,

  pipeline: wgpu::ComputePipeline,

  point_count: u32,

}

impl GpuScaledDiracGenerator1d {

  pub fn new(

    device: &wgpu::Device,

    point_count: u32,

    lattice_spacing: f32,

    spectral_scale: f32,

  ) -> Self {

    assert!(

      point_count > 0,

      "GPU scaled Dirac generator requires at least one field point.",

    );

    assert!(

      lattice_spacing.is_finite()

        && lattice_spacing > 0.0,

      "GPU scaled Dirac generator requires a positive finite lattice spacing.",

    );

    assert!(

      spectral_scale.is_finite()

        && spectral_scale > 0.0,

      "GPU scaled Dirac generator requires a positive finite spectral scale.",

    );

    let mass_profile_buffer = device.create_buffer(

      &wgpu::BufferDescriptor {

        label: Some(

          "Scaled Dirac Generator Mass Profile Buffer",

        ),

        size:

          point_count as u64

          * std::mem::size_of::<f32>() as u64,

        usage:

          wgpu::BufferUsages::STORAGE

          | wgpu::BufferUsages::COPY_DST,

        mapped_at_creation: false,

      },

    );

    let parameter_bytes = create_parameter_bytes(

      point_count,

      lattice_spacing,

      spectral_scale,

    );

    let parameter_buffer = device.create_buffer_init(

      &wgpu::util::BufferInitDescriptor {

        label: Some(

          "Scaled Dirac Generator Parameter Buffer",

        ),

        contents:

          &parameter_bytes,

        usage:

          wgpu::BufferUsages::UNIFORM,

      },

    );

    let bind_group_layout = create_bind_group_layout(

      device,

    );

    let shader = device.create_shader_module(

      wgpu::ShaderModuleDescriptor {

        label: Some(

          "Scaled Dirac Generator 1D Shader",

        ),

        source: wgpu::ShaderSource::Wgsl(

          include_str!(

            "scaled_dirac_generator_1d.wgsl"

          )

          .into(),

        ),

      },

    );

    let pipeline_layout = device.create_pipeline_layout(

      &wgpu::PipelineLayoutDescriptor {

        label: Some(

          "Scaled Dirac Generator 1D Pipeline Layout",

        ),

        bind_group_layouts: &[

          Some(

            &bind_group_layout,

          ),

        ],

        immediate_size: 0,

      },

    );

    let pipeline = device.create_compute_pipeline(

      &wgpu::ComputePipelineDescriptor {

        label: Some(

          "Scaled Dirac Generator 1D Compute Pipeline",

        ),

        layout: Some(

          &pipeline_layout,

        ),

        module:

          &shader,

        entry_point: Some(

          "main",

        ),

        compilation_options:

          wgpu::PipelineCompilationOptions::default(),

        cache: None,

      },

    );

    Self {

      mass_profile_buffer,

      _parameter_buffer:

        parameter_buffer,

      bind_group_layout,

      pipeline,

      point_count,

    }

  }

  pub fn mass_profile_buffer(

    &self,

  ) -> &wgpu::Buffer {

    &self.mass_profile_buffer

  }

  pub fn create_binding(

    &self,

    device: &wgpu::Device,

    input_buffer: &wgpu::Buffer,

    output_buffer: &wgpu::Buffer,

  ) -> wgpu::BindGroup {

    device.create_bind_group(

      &wgpu::BindGroupDescriptor {

        label: Some(

          "Scaled Dirac Generator 1D Bind Group",

        ),

        layout:

          &self.bind_group_layout,

        entries: &[

          wgpu::BindGroupEntry {

            binding: 0,

            resource:

              input_buffer

                .as_entire_binding(),

          },

          wgpu::BindGroupEntry {

            binding: 1,

            resource:

              output_buffer

                .as_entire_binding(),

          },

          wgpu::BindGroupEntry {

            binding: 2,

            resource:

              self

                .mass_profile_buffer

                .as_entire_binding(),

          },

          wgpu::BindGroupEntry {

            binding: 3,

            resource:

              self

                ._parameter_buffer

                .as_entire_binding(),

          },

        ],

      },

    )

  }

  pub fn record_apply(

    &self,

    encoder: &mut wgpu::CommandEncoder,

    binding: &wgpu::BindGroup,

  ) {

    let mut compute_pass = encoder.begin_compute_pass(

      &wgpu::ComputePassDescriptor {

        label: Some(

          "Scaled Dirac Generator 1D Compute Pass",

        ),

        timestamp_writes: None,

      },

    );

    compute_pass.set_pipeline(

      &self.pipeline,

    );

    compute_pass.set_bind_group(

      0,

      binding,

      &[],

    );

    compute_pass.dispatch_workgroups(

      self.point_count.div_ceil(

        WORKGROUP_SIZE,

      ),

      1,

      1,

    );

  }

}

fn create_parameter_bytes(

  point_count: u32,

  lattice_spacing: f32,

  spectral_scale: f32,

) -> Vec<u8> {

  let inverse_spectral_scale =

    1.0_f32

    / spectral_scale;

  let mut bytes = Vec::with_capacity(

    GENERATOR_PARAMETERS_SIZE as usize,

  );

  bytes.extend_from_slice(

    &point_count.to_le_bytes(),

  );

  bytes.extend_from_slice(

    &lattice_spacing.to_le_bytes(),

  );

  bytes.extend_from_slice(

    &inverse_spectral_scale.to_le_bytes(),

  );

  bytes.extend_from_slice(

    &0_u32.to_le_bytes(),

  );

  bytes

}

fn create_bind_group_layout(

  device: &wgpu::Device,

) -> wgpu::BindGroupLayout {

  device.create_bind_group_layout(

    &wgpu::BindGroupLayoutDescriptor {

      label: Some(

        "Scaled Dirac Generator 1D Bind Group Layout",

      ),

      entries: &[

        storage_buffer_layout_entry(

          0,

          true,

        ),

        storage_buffer_layout_entry(

          1,

          false,

        ),

        storage_buffer_layout_entry(

          2,

          true,

        ),

        wgpu::BindGroupLayoutEntry {

          binding: 3,

          visibility:

            wgpu::ShaderStages::COMPUTE,

          ty: wgpu::BindingType::Buffer {

            ty:

              wgpu::BufferBindingType::Uniform,

            has_dynamic_offset: false,

            min_binding_size:

              NonZeroU64::new(

                GENERATOR_PARAMETERS_SIZE,

              ),

          },

          count: None,

        },

      ],

    },

  )

}

fn storage_buffer_layout_entry(

  binding: u32,

  read_only: bool,

) -> wgpu::BindGroupLayoutEntry {

  wgpu::BindGroupLayoutEntry {

    binding,

    visibility:

      wgpu::ShaderStages::COMPUTE,

    ty: wgpu::BindingType::Buffer {

      ty:

        wgpu::BufferBindingType::Storage {

          read_only,

        },

      has_dynamic_offset: false,

      min_binding_size: None,

    },

    count: None,

  }

}