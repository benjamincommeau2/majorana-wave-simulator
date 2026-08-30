use std::num::NonZeroU64;

use wgpu::util::DeviceExt;

const WORKGROUP_SIZE: u32 = 64;

const GENERATOR_PARAMETERS_SIZE: u64 = 16;

pub struct GpuScaledDiracGenerator1d {

  mass_profile_buffer: wgpu::Buffer,

  _derivative_matrix_buffer: wgpu::Buffer,

  _parameter_buffer: wgpu::Buffer,

  bind_group_layout: wgpu::BindGroupLayout,

  pipeline: wgpu::ComputePipeline,

  total_point_count: u32,

}

impl GpuScaledDiracGenerator1d {

  pub fn new(

    device: &wgpu::Device,

    point_count: u32,

    lattice_spacing: f32,

    spectral_scale: f32,

  ) -> Self {

    Self::new_batched_x_lines(

      device,

      point_count,

      1,

      lattice_spacing,

      spectral_scale,

    )

  }

  pub fn new_batched_x_lines(

    device: &wgpu::Device,

    line_length: u32,

    line_count: u32,

    lattice_spacing: f32,

    spectral_scale: f32,

  ) -> Self {

    assert!(

      line_length > 0,

      "GPU scaled Dirac generator requires at least one point per x-line.",

    );

    assert!(

      line_count > 0,

      "GPU scaled Dirac generator requires at least one x-line.",

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

    let total_point_count = line_length

      .checked_mul(

        line_count,

      )

      .expect(

        "GPU scaled Dirac generator point count overflowed.",

      );

    let mass_profile_buffer = device.create_buffer(

      &wgpu::BufferDescriptor {

        label: Some(

          "Scaled Dirac Generator Mass Profile Buffer",

        ),

        size:

          line_length as u64

          * std::mem::size_of::<f32>() as u64,

        usage:

          wgpu::BufferUsages::STORAGE

          | wgpu::BufferUsages::COPY_DST,

        mapped_at_creation: false,

      },

    );

    let derivative_matrix = create_spectral_derivative_matrix(

      line_length,

      lattice_spacing,

    );

    let derivative_matrix_buffer = device.create_buffer_init(

      &wgpu::util::BufferInitDescriptor {

        label: Some(

          "Scaled Dirac Generator Spectral Derivative Matrix Buffer",

        ),

        contents: bytemuck::cast_slice(

          &derivative_matrix,

        ),

        usage:

          wgpu::BufferUsages::STORAGE,

      },

    );

    let parameter_bytes = create_parameter_bytes(

      line_length,

      total_point_count,

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

      _derivative_matrix_buffer:

        derivative_matrix_buffer,

      _parameter_buffer:

        parameter_buffer,

      bind_group_layout,

      pipeline,

      total_point_count,

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

                ._derivative_matrix_buffer

                .as_entire_binding(),

          },

          wgpu::BindGroupEntry {

            binding: 4,

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

      self.total_point_count.div_ceil(

        WORKGROUP_SIZE,

      ),

      1,

      1,

    );

  }

}

fn create_spectral_derivative_matrix(

  line_length: u32,

  lattice_spacing: f32,

) -> Vec<f32> {

  let line_length_f64 =

    line_length as f64;

  let lattice_spacing_f64 =

    lattice_spacing as f64;

  let derivative_scale =

    std::f64::consts::PI

    / (

      line_length_f64

      * lattice_spacing_f64

    );

  let mut matrix = Vec::with_capacity(

    line_length as usize

    * line_length as usize,

  );

  for output_x in 0..line_length {

    for source_x in 0..line_length {

      if output_x == source_x {

        matrix.push(

          0.0,

        );

        continue;

      }

      let signed_difference =

        output_x as i64

        - source_x as i64;

      let absolute_difference =

        signed_difference.abs() as u32;

      let even_grid_opposite_point =

        line_length % 2 == 0

        && absolute_difference * 2 == line_length;

      if even_grid_opposite_point {

        matrix.push(

          0.0,

        );

        continue;

      }

      let angle =

        std::f64::consts::PI

        * signed_difference as f64

        / line_length_f64;

      let alternating_sign =

        if absolute_difference % 2 == 0 {

          1.0_f64

        } else {

          -1.0_f64

        };

      let sine =

        angle.sin();

      let coefficient =

        if line_length % 2 == 0 {

          derivative_scale

          * alternating_sign

          * angle.cos()

          / sine

        } else {

          derivative_scale

          * alternating_sign

          / sine

        };

      matrix.push(

        coefficient as f32,

      );

    }

  }

  matrix

}

fn create_parameter_bytes(

  line_length: u32,

  total_point_count: u32,

  spectral_scale: f32,

) -> Vec<u8> {

  let inverse_spectral_scale =

    1.0_f32

    / spectral_scale;

  let mut bytes = Vec::with_capacity(

    GENERATOR_PARAMETERS_SIZE as usize,

  );

  bytes.extend_from_slice(

    &line_length.to_le_bytes(),

  );

  bytes.extend_from_slice(

    &total_point_count.to_le_bytes(),

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

        storage_buffer_layout_entry(

          3,

          true,

        ),

        wgpu::BindGroupLayoutEntry {

          binding: 4,

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