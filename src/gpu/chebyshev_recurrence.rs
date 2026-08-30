use std::num::NonZeroU64;

use wgpu::util::DeviceExt;

const COMPONENTS_PER_POINT: u64 = 4;

const BYTES_PER_COMPONENT: u64 =
  std::mem::size_of::<f32>() as u64;

const WORKGROUP_SIZE: u32 = 64;

const STEP_PARAMETERS_SIZE: u64 = 16;

pub struct GpuChebyshevRecurrence {

  basis_a_buffer: wgpu::Buffer,

  basis_b_buffer: wgpu::Buffer,

  generator_scratch_buffer: wgpu::Buffer,

  accumulator_buffer: wgpu::Buffer,

  _step_parameter_buffer: wgpu::Buffer,

  pipeline: wgpu::ComputePipeline,

  basis_a_as_previous_bind_group: wgpu::BindGroup,

  basis_b_as_previous_bind_group: wgpu::BindGroup,

  point_count: u32,

  coefficient_count: usize,

  parameter_stride: u32,

  current_basis_is_a: bool,

}

impl GpuChebyshevRecurrence {

  pub fn new(

    device: &wgpu::Device,

    point_count: u32,

    coefficients: &[f64],

  ) -> Self {

    assert!(

      point_count > 0,

      "GPU Chebyshev recurrence requires at least one field point.",

    );

    assert!(

      !coefficients.is_empty(),

      "GPU Chebyshev recurrence requires at least one coefficient.",

    );

    let field_buffer_size =

      point_count as u64

      * COMPONENTS_PER_POINT

      * BYTES_PER_COMPONENT;

    let basis_a_buffer = create_field_buffer(

      device,

      "Chebyshev Basis A Buffer",

      field_buffer_size,

    );

    let basis_b_buffer = create_field_buffer(

      device,

      "Chebyshev Basis B Buffer",

      field_buffer_size,

    );

    let generator_scratch_buffer = create_field_buffer(

      device,

      "Chebyshev Generator Scratch Buffer",

      field_buffer_size,

    );

    let accumulator_buffer = create_field_buffer(

      device,

      "Chebyshev Accumulator Buffer",

      field_buffer_size,

    );

    let parameter_stride =

      device

        .limits()

        .min_uniform_buffer_offset_alignment

        .max(

          STEP_PARAMETERS_SIZE as u32,

        );

    let step_parameter_bytes = create_step_parameter_bytes(

      point_count,

      coefficients,

      parameter_stride,

    );

    let step_parameter_buffer = device.create_buffer_init(

      &wgpu::util::BufferInitDescriptor {

        label: Some(

          "Chebyshev Step Parameter Buffer",

        ),

        contents: &step_parameter_bytes,

        usage: wgpu::BufferUsages::UNIFORM,

      },

    );

    let bind_group_layout = create_bind_group_layout(

      device,

    );

    let shader = device.create_shader_module(

      wgpu::ShaderModuleDescriptor {

        label: Some(

          "Chebyshev Recurrence Shader",

        ),

        source: wgpu::ShaderSource::Wgsl(

          include_str!(

            "chebyshev_recurrence.wgsl"

          )

          .into(),

        ),

      },

    );

    let pipeline_layout = device.create_pipeline_layout(

      &wgpu::PipelineLayoutDescriptor {

        label: Some(

          "Chebyshev Recurrence Pipeline Layout",

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

          "Chebyshev Recurrence Compute Pipeline",

        ),

        layout: Some(

          &pipeline_layout,

        ),

        module: &shader,

        entry_point: Some(

          "main",

        ),

        compilation_options:

          wgpu::PipelineCompilationOptions::default(),

        cache: None,

      },

    );

    let basis_a_as_previous_bind_group = create_recurrence_bind_group(

      device,

      &bind_group_layout,

      "Chebyshev Basis A Previous Bind Group",

      &basis_a_buffer,

      &generator_scratch_buffer,

      &accumulator_buffer,

      &step_parameter_buffer,

    );

    let basis_b_as_previous_bind_group = create_recurrence_bind_group(

      device,

      &bind_group_layout,

      "Chebyshev Basis B Previous Bind Group",

      &basis_b_buffer,

      &generator_scratch_buffer,

      &accumulator_buffer,

      &step_parameter_buffer,

    );

    Self {

      basis_a_buffer,

      basis_b_buffer,

      generator_scratch_buffer,

      accumulator_buffer,

      _step_parameter_buffer:

        step_parameter_buffer,

      pipeline,

      basis_a_as_previous_bind_group,

      basis_b_as_previous_bind_group,

      point_count,

      coefficient_count:

        coefficients.len(),

      parameter_stride,

      current_basis_is_a: false,

    }

  }

  pub(crate) fn reset_basis_roles(

    &mut self,

  ) {

    self.current_basis_is_a =

      false;

  }

  pub fn previous_basis_buffer(

    &self,

  ) -> &wgpu::Buffer {

    if self.current_basis_is_a {

      &self.basis_b_buffer

    } else {

      &self.basis_a_buffer

    }

  }

  pub fn current_basis_buffer(

    &self,

  ) -> &wgpu::Buffer {

    if self.current_basis_is_a {

      &self.basis_a_buffer

    } else {

      &self.basis_b_buffer

    }

  }

  pub fn generator_scratch_buffer(

    &self,

  ) -> &wgpu::Buffer {

    &self.generator_scratch_buffer

  }

  pub fn accumulator_buffer(

    &self,

  ) -> &wgpu::Buffer {

    &self.accumulator_buffer

  }

  pub fn record_step(

    &mut self,

    encoder: &mut wgpu::CommandEncoder,

    coefficient_order: usize,

  ) {

    assert!(

      coefficient_order < self.coefficient_count,

      "Chebyshev coefficient order is outside the configured coefficient table.",

    );

    let parameter_offset =

      coefficient_order as u64

      * self.parameter_stride as u64;

    let dynamic_parameter_offset = u32::try_from(

      parameter_offset,

    )

    .expect(

      "Chebyshev parameter offset exceeds WebGPU dynamic-offset range.",

    );

    let bind_group =

      if self.current_basis_is_a {

        &self.basis_b_as_previous_bind_group

      } else {

        &self.basis_a_as_previous_bind_group

      };

    {

      let mut compute_pass = encoder.begin_compute_pass(

        &wgpu::ComputePassDescriptor {

          label: Some(

            "Chebyshev Recurrence Compute Pass",

          ),

          timestamp_writes: None,

        },

      );

      compute_pass.set_pipeline(

        &self.pipeline,

      );

      compute_pass.set_bind_group(

        0,

        bind_group,

        &[

          dynamic_parameter_offset,

        ],

      );

      compute_pass.dispatch_workgroups(

        self.point_count.div_ceil(

          WORKGROUP_SIZE,

        ),

        1,

        1,

      );

    }

    self.current_basis_is_a =

      !self.current_basis_is_a;

  }

}

fn create_field_buffer(

  device: &wgpu::Device,

  label: &'static str,

  size: wgpu::BufferAddress,

) -> wgpu::Buffer {

  device.create_buffer(

    &wgpu::BufferDescriptor {

      label: Some(

        label,

      ),

      size,

      usage:

        wgpu::BufferUsages::STORAGE

        | wgpu::BufferUsages::COPY_DST

        | wgpu::BufferUsages::COPY_SRC,

      mapped_at_creation: false,

    },

  )

}

fn create_step_parameter_bytes(

  point_count: u32,

  coefficients: &[f64],

  parameter_stride: u32,

) -> Vec<u8> {

  let parameter_stride =

    parameter_stride as usize;

  let total_size = parameter_stride

    .checked_mul(

      coefficients.len(),

    )

    .expect(

      "Chebyshev parameter buffer size overflowed.",

    );

  let mut bytes = vec![

    0_u8;

    total_size

  ];

  for (

    coefficient_order,

    coefficient,

  ) in coefficients

    .iter()

    .enumerate()

  {

    let parameter_start =

      coefficient_order

      * parameter_stride;

    let coefficient_bytes =

      (

        *coefficient as f32

      )

      .to_ne_bytes();

    let point_count_bytes =

      point_count.to_ne_bytes();

    bytes[

      parameter_start

        ..parameter_start + 4

    ]

    .copy_from_slice(

      &coefficient_bytes,

    );

    bytes[

      parameter_start + 4

        ..parameter_start + 8

    ]

    .copy_from_slice(

      &point_count_bytes,

    );

  }

  bytes

}

fn create_bind_group_layout(

  device: &wgpu::Device,

) -> wgpu::BindGroupLayout {

  device.create_bind_group_layout(

    &wgpu::BindGroupLayoutDescriptor {

      label: Some(

        "Chebyshev Recurrence Bind Group Layout",

      ),

      entries: &[

        storage_buffer_layout_entry(

          0,

          false,

        ),

        storage_buffer_layout_entry(

          1,

          true,

        ),

        storage_buffer_layout_entry(

          2,

          false,

        ),

        wgpu::BindGroupLayoutEntry {

          binding: 3,

          visibility:

            wgpu::ShaderStages::COMPUTE,

          ty: wgpu::BindingType::Buffer {

            ty:

              wgpu::BufferBindingType::Uniform,

            has_dynamic_offset: true,

            min_binding_size:

              NonZeroU64::new(

                STEP_PARAMETERS_SIZE,

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

fn create_recurrence_bind_group(

  device: &wgpu::Device,

  layout: &wgpu::BindGroupLayout,

  label: &'static str,

  previous_basis_buffer: &wgpu::Buffer,

  generator_scratch_buffer: &wgpu::Buffer,

  accumulator_buffer: &wgpu::Buffer,

  step_parameter_buffer: &wgpu::Buffer,

) -> wgpu::BindGroup {

  device.create_bind_group(

    &wgpu::BindGroupDescriptor {

      label: Some(

        label,

      ),

      layout,

      entries: &[

        wgpu::BindGroupEntry {

          binding: 0,

          resource:

            previous_basis_buffer

              .as_entire_binding(),

        },

        wgpu::BindGroupEntry {

          binding: 1,

          resource:

            generator_scratch_buffer

              .as_entire_binding(),

        },

        wgpu::BindGroupEntry {

          binding: 2,

          resource:

            accumulator_buffer

              .as_entire_binding(),

        },

        wgpu::BindGroupEntry {

          binding: 3,

          resource:

            wgpu::BindingResource::Buffer(

              wgpu::BufferBinding {

                buffer:

                  step_parameter_buffer,

                offset: 0,

                size:

                  NonZeroU64::new(

                    STEP_PARAMETERS_SIZE,

                  ),

              },

            ),

        },

      ],

    },

  )

}