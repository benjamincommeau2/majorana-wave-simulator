use std::num::NonZeroU64;

use wgpu::util::DeviceExt;

use crate::gpu::chebyshev_recurrence::GpuChebyshevRecurrence;

use crate::gpu::scaled_dirac_generator_1d::GpuScaledDiracGenerator1d;

use crate::physics::chebyshev_propagation_setup::ChebyshevPropagationSetup;

const COMPONENTS_PER_POINT: u64 = 4;

const BYTES_PER_COMPONENT: u64 =
  std::mem::size_of::<f32>() as u64;

const WORKGROUP_SIZE: u32 = 64;

const INITIALIZATION_PARAMETERS_SIZE: u64 = 16;

pub struct GpuChebyshevPropagator1d {

  generator: GpuScaledDiracGenerator1d,

  recurrence: GpuChebyshevRecurrence,

  generator_basis_a_binding: wgpu::BindGroup,

  generator_basis_b_binding: wgpu::BindGroup,

  initialization_pipeline: wgpu::ComputePipeline,

  initialization_bind_group: wgpu::BindGroup,

  _initialization_parameter_buffer: wgpu::Buffer,

  point_count: u32,

  field_buffer_size: wgpu::BufferAddress,

  max_order: usize,

}

impl GpuChebyshevPropagator1d {

  pub fn new(

    device: &wgpu::Device,

    point_count: u32,

    lattice_spacing: f32,

    setup: &ChebyshevPropagationSetup,

  ) -> Self {

    Self::new_batched_x_lines(

      device,

      point_count,

      1,

      lattice_spacing,

      setup,

    )

  }

  pub fn new_batched_x_lines(

    device: &wgpu::Device,

    line_length: u32,

    line_count: u32,

    lattice_spacing: f32,

    setup: &ChebyshevPropagationSetup,

  ) -> Self {

    assert!(

      line_length > 0,

      "GPU Chebyshev propagator requires at least one point per x-line.",

    );

    assert!(

      line_count > 0,

      "GPU Chebyshev propagator requires at least one x-line.",

    );

    assert!(

      !setup.coefficients().is_empty(),

      "GPU Chebyshev propagator requires at least one coefficient.",

    );

    let total_point_count = line_length

      .checked_mul(

        line_count,

      )

      .expect(

        "GPU Chebyshev propagator point count overflowed.",

      );

    let field_buffer_size =

      total_point_count as u64

      * COMPONENTS_PER_POINT

      * BYTES_PER_COMPONENT;

    let mut recurrence = GpuChebyshevRecurrence::new(

      device,

      total_point_count,

      setup.coefficients(),

    );

    recurrence.reset_basis_roles();

    let generator = GpuScaledDiracGenerator1d::new_batched_x_lines(

      device,

      line_length,

      line_count,

      lattice_spacing,

      setup.spectral_scale() as f32,

    );

    let generator_basis_a_binding = generator.create_binding(

      device,

      recurrence.previous_basis_buffer(),

      recurrence.generator_scratch_buffer(),

    );

    let generator_basis_b_binding = generator.create_binding(

      device,

      recurrence.current_basis_buffer(),

      recurrence.generator_scratch_buffer(),

    );

    let initialization_parameter_bytes = create_initialization_parameter_bytes(

      total_point_count,

      setup.coefficients(),

    );

    let initialization_parameter_buffer = device.create_buffer_init(

      &wgpu::util::BufferInitDescriptor {

        label: Some(

          "Chebyshev Propagator Initialization Parameter Buffer",

        ),

        contents:

          &initialization_parameter_bytes,

        usage:

          wgpu::BufferUsages::UNIFORM,

      },

    );

    let initialization_bind_group_layout = create_initialization_bind_group_layout(

      device,

    );

    let initialization_shader = device.create_shader_module(

      wgpu::ShaderModuleDescriptor {

        label: Some(

          "Chebyshev Propagator Initialization Shader",

        ),

        source: wgpu::ShaderSource::Wgsl(

          include_str!(

            "chebyshev_propagator_1d.wgsl"

          )

          .into(),

        ),

      },

    );

    let initialization_pipeline_layout = device.create_pipeline_layout(

      &wgpu::PipelineLayoutDescriptor {

        label: Some(

          "Chebyshev Propagator Initialization Pipeline Layout",

        ),

        bind_group_layouts: &[

          Some(

            &initialization_bind_group_layout,

          ),

        ],

        immediate_size: 0,

      },

    );

    let initialization_pipeline = device.create_compute_pipeline(

      &wgpu::ComputePipelineDescriptor {

        label: Some(

          "Chebyshev Propagator Initialization Compute Pipeline",

        ),

        layout: Some(

          &initialization_pipeline_layout,

        ),

        module:

          &initialization_shader,

        entry_point: Some(

          "main",

        ),

        compilation_options:

          wgpu::PipelineCompilationOptions::default(),

        cache: None,

      },

    );

    let initialization_bind_group = device.create_bind_group(

      &wgpu::BindGroupDescriptor {

        label: Some(

          "Chebyshev Propagator Initialization Bind Group",

        ),

        layout:

          &initialization_bind_group_layout,

        entries: &[

          wgpu::BindGroupEntry {

            binding: 0,

            resource:

              recurrence

                .previous_basis_buffer()

                .as_entire_binding(),

          },

          wgpu::BindGroupEntry {

            binding: 1,

            resource:

              recurrence

                .current_basis_buffer()

                .as_entire_binding(),

          },

          wgpu::BindGroupEntry {

            binding: 2,

            resource:

              recurrence

                .accumulator_buffer()

                .as_entire_binding(),

          },

          wgpu::BindGroupEntry {

            binding: 3,

            resource:

              initialization_parameter_buffer

                .as_entire_binding(),

          },

        ],

      },

    );

    Self {

      generator,

      recurrence,

      generator_basis_a_binding,

      generator_basis_b_binding,

      initialization_pipeline,

      initialization_bind_group,

      _initialization_parameter_buffer:

        initialization_parameter_buffer,

      point_count:

        total_point_count,

      field_buffer_size,

      max_order:

        setup.max_order(),

    }

  }

  pub fn mass_profile_buffer(

    &self,

  ) -> &wgpu::Buffer {

    self.generator

      .mass_profile_buffer()

  }

  pub fn record_step(

    &mut self,

    encoder: &mut wgpu::CommandEncoder,

    state_buffer: &wgpu::Buffer,

  ) {

    self.recurrence

      .reset_basis_roles();

    encoder.copy_buffer_to_buffer(

      state_buffer,

      0,

      self.recurrence

        .previous_basis_buffer(),

      0,

      self.field_buffer_size,

    );

    self.generator.record_apply(

      encoder,

      &self.generator_basis_a_binding,

    );

    encoder.copy_buffer_to_buffer(

      self.recurrence

        .generator_scratch_buffer(),

      0,

      self.recurrence

        .current_basis_buffer(),

      0,

      self.field_buffer_size,

    );

    self.record_accumulator_initialization(

      encoder,

    );

    for coefficient_order in 2..=self.max_order {

      let generator_binding =

        if coefficient_order % 2 == 0 {

          &self.generator_basis_b_binding

        } else {

          &self.generator_basis_a_binding

        };

      self.generator.record_apply(

        encoder,

        generator_binding,

      );

      self.recurrence.record_step(

        encoder,

        coefficient_order,

      );

    }

    encoder.copy_buffer_to_buffer(

      self.recurrence

        .accumulator_buffer(),

      0,

      state_buffer,

      0,

      self.field_buffer_size,

    );

  }

  fn record_accumulator_initialization(

    &self,

    encoder: &mut wgpu::CommandEncoder,

  ) {

    let mut compute_pass = encoder.begin_compute_pass(

      &wgpu::ComputePassDescriptor {

        label: Some(

          "Chebyshev Propagator Initialization Compute Pass",

        ),

        timestamp_writes: None,

      },

    );

    compute_pass.set_pipeline(

      &self.initialization_pipeline,

    );

    compute_pass.set_bind_group(

      0,

      &self.initialization_bind_group,

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

fn create_initialization_parameter_bytes(

  point_count: u32,

  coefficients: &[f64],

) -> Vec<u8> {

  let coefficient_zero =

    coefficients[0] as f32;

  let coefficient_one =

    coefficients

      .get(

        1,

      )

      .copied()

      .unwrap_or(

        0.0,

      ) as f32;

  let mut bytes = Vec::with_capacity(

    INITIALIZATION_PARAMETERS_SIZE as usize,

  );

  bytes.extend_from_slice(

    &point_count.to_le_bytes(),

  );

  bytes.extend_from_slice(

    &coefficient_zero.to_le_bytes(),

  );

  bytes.extend_from_slice(

    &coefficient_one.to_le_bytes(),

  );

  bytes.extend_from_slice(

    &0_u32.to_le_bytes(),

  );

  bytes

}

fn create_initialization_bind_group_layout(

  device: &wgpu::Device,

) -> wgpu::BindGroupLayout {

  device.create_bind_group_layout(

    &wgpu::BindGroupLayoutDescriptor {

      label: Some(

        "Chebyshev Propagator Initialization Bind Group Layout",

      ),

      entries: &[

        storage_buffer_layout_entry(

          0,

          true,

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

            has_dynamic_offset:

              false,

            min_binding_size:

              NonZeroU64::new(

                INITIALIZATION_PARAMETERS_SIZE,

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

      has_dynamic_offset:

        false,

      min_binding_size:

        None,

    },

    count: None,

  }

}