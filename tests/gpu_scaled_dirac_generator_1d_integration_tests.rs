use majorana_wave_simulator::gpu;

use majorana_wave_simulator::gpu::scaled_dirac_generator_1d::GpuScaledDiracGenerator1d;

use majorana_wave_simulator::physics::dirac_generator::direct_dirac_generator_with_mass_profile_1d;

use majorana_wave_simulator::physics::mass_profile::create_mass_step_profile_1d;

use wgpu::util::DeviceExt;

const COMPONENTS_PER_POINT: usize = 4;

const FLOAT_TOLERANCE: f32 = 1.0e-4_f32;

fn assert_f32_slices_approximately_equal(

  actual: &[f32],

  expected: &[f32],

) {

  assert_eq!(

    actual.len(),

    expected.len(),

  );

  for index in 0..actual.len() {

    let difference =

      (

        actual[index]

        - expected[index]

      )

      .abs();

    assert!(

      difference <= FLOAT_TOLERANCE,

      "index = {index}, actual = {}, expected = {}, difference = {difference}",

      actual[index],

      expected[index],

    );

  }

}

fn field_buffer_size(

  point_count: usize,

) -> wgpu::BufferAddress {

  point_count as wgpu::BufferAddress

    * COMPONENTS_PER_POINT as wgpu::BufferAddress

    * std::mem::size_of::<f32>() as wgpu::BufferAddress

}

fn create_input_buffer(

  device: &wgpu::Device,

  field: &[[f32; COMPONENTS_PER_POINT]],

) -> wgpu::Buffer {

  device.create_buffer_init(

    &wgpu::util::BufferInitDescriptor {

      label: Some(

        "Scaled Dirac Generator Integration Test Input Buffer",

      ),

      contents: bytemuck::cast_slice(

        field,

      ),

      usage:

        wgpu::BufferUsages::STORAGE,

    },

  )

}

fn create_output_buffer(

  device: &wgpu::Device,

  point_count: usize,

) -> wgpu::Buffer {

  device.create_buffer(

    &wgpu::BufferDescriptor {

      label: Some(

        "Scaled Dirac Generator Integration Test Output Buffer",

      ),

      size: field_buffer_size(

        point_count,

      ),

      usage:

        wgpu::BufferUsages::STORAGE

        | wgpu::BufferUsages::COPY_SRC,

      mapped_at_creation: false,

    },

  )

}

fn read_gpu_f32_buffer(

  device: &wgpu::Device,

  queue: &wgpu::Queue,

  source_buffer: &wgpu::Buffer,

  buffer_size: wgpu::BufferAddress,

) -> Vec<f32> {

  let readback_buffer = device.create_buffer(

    &wgpu::BufferDescriptor {

      label: Some(

        "Scaled Dirac Generator Integration Test Readback Buffer",

      ),

      size: buffer_size,

      usage:

        wgpu::BufferUsages::COPY_DST

        | wgpu::BufferUsages::MAP_READ,

      mapped_at_creation: false,

    },

  );

  let mut encoder =

    gpu::commands::create_command_encoder(

      device,

    );

  encoder.copy_buffer_to_buffer(

    source_buffer,

    0,

    &readback_buffer,

    0,

    buffer_size,

  );

  gpu::commands::submit_commands(

    queue,

    encoder,

  );

  let readback_slice =

    readback_buffer.slice(..);

  let (

    sender,

    receiver,

  ) = std::sync::mpsc::channel();

  readback_slice.map_async(

    wgpu::MapMode::Read,

    move |map_result| {

      sender

        .send(

          map_result,

        )

        .expect(

          "Could not send scaled Dirac generator GPU map result",

        );

    },

  );

  device

    .poll(

      wgpu::PollType::wait_indefinitely(),

    )

    .expect(

      "Could not wait for scaled Dirac generator GPU readback",

    );

  receiver

    .recv()

    .expect(

      "Scaled Dirac generator GPU mapping callback did not report a result",

    )

    .expect(

      "Scaled Dirac generator GPU readback buffer could not be mapped",

    );

  let mapped_range = readback_buffer

    .slice(..)

    .get_mapped_range()

    .expect(

      "Could not access scaled Dirac generator mapped GPU bytes",

    );

  let result =

    bytemuck::cast_slice::<u8, f32>(

      &mapped_range,

    )

    .to_vec();

  drop(

    mapped_range,

  );

  readback_buffer.unmap();

  result

}

fn scaled_cpu_reference(

  field: &[[f32; COMPONENTS_PER_POINT]],

  lattice_spacing: f32,

  mass_profile: &[f32],

  spectral_scale: f32,

) -> Vec<f32> {

  direct_dirac_generator_with_mass_profile_1d(

    field,

    lattice_spacing,

    mass_profile,

  )

  .iter()

  .flat_map(

    |state| {

      state

        .iter()

        .map(

          |component| {

            *component

            / spectral_scale

          },

        )

    },

  )

  .collect()

}

fn dispatch_generator(

  device: &wgpu::Device,

  queue: &wgpu::Queue,

  generator: &GpuScaledDiracGenerator1d,

  binding: &wgpu::BindGroup,

  output_buffer: &wgpu::Buffer,

  mass_profile: &[f32],

) -> Vec<f32> {

  queue.write_buffer(

    generator.mass_profile_buffer(),

    0,

    bytemuck::cast_slice(

      mass_profile,

    ),

  );

  let mut encoder =

    gpu::commands::create_command_encoder(

      device,

    );

  generator.record_apply(

    &mut encoder,

    binding,

  );

  gpu::commands::submit_commands(

    queue,

    encoder,

  );

  read_gpu_f32_buffer(

    device,

    queue,

    output_buffer,

    field_buffer_size(

      mass_profile.len(),

    ),

  )

}

#[test] // Verifies the complete scaled Dirac generator against the CPU oracle on an odd spectral grid.

#[ignore = "requires a native GPU adapter"]

fn gpu_scaled_dirac_generator_matches_cpu_on_odd_grid() {

  pollster::block_on(

    async {

      let instance =

        gpu::gpu_context::create_instance();

      let adapter = gpu::gpu_context::request_adapter(

        &instance,

      )

      .await;

      let (

        device,

        queue,

      ) = gpu::gpu_context::request_device_and_queue(

        &adapter,

      )

      .await;

      let field = [

        [

          1.0_f32,

          -0.5,

          0.25,

          2.0,

        ],

        [

          -0.75_f32,

          0.1,

          1.5,

          -0.2,

        ],

        [

          0.3_f32,

          1.25,

          -0.8,

          0.6,

        ],

        [

          2.0_f32,

          -1.0,

          0.4,

          -0.75,

        ],

        [

          -0.2_f32,

          0.9,

          1.1,

          0.35,

        ],

      ];

      let lattice_spacing =

        0.4_f32;

      let spectral_scale =

        7.0_f32;

      let mass_profile =

        vec![

          0.75_f32;

          field.len()

        ];

      let generator = GpuScaledDiracGenerator1d::new(

        &device,

        field.len() as u32,

        lattice_spacing,

        spectral_scale,

      );

      let input_buffer = create_input_buffer(

        &device,

        &field,

      );

      let output_buffer = create_output_buffer(

        &device,

        field.len(),

      );

      let binding = generator.create_binding(

        &device,

        &input_buffer,

        &output_buffer,

      );

      let actual = dispatch_generator(

        &device,

        &queue,

        &generator,

        &binding,

        &output_buffer,

        &mass_profile,

      );

      let expected = scaled_cpu_reference(

        &field,

        lattice_spacing,

        &mass_profile,

        spectral_scale,

      );

      assert_f32_slices_approximately_equal(

        &actual,

        &expected,

      );

    },

  );

}

#[test] // Verifies that an interactive mass-wall move changes only the uploaded profile while the same GPU generator and binding remain reusable.

#[ignore = "requires a native GPU adapter"]

fn gpu_scaled_dirac_generator_reuses_setup_when_mass_boundary_moves() {

  pollster::block_on(

    async {

      let instance =

        gpu::gpu_context::create_instance();

      let adapter = gpu::gpu_context::request_adapter(

        &instance,

      )

      .await;

      let (

        device,

        queue,

      ) = gpu::gpu_context::request_device_and_queue(

        &adapter,

      )

      .await;

      let field = [

        [

          0.5_f32,

          1.0,

          -0.5,

          0.25,

        ],

        [

          -1.0_f32,

          0.75,

          0.3,

          1.25,

        ],

        [

          1.5_f32,

          -0.2,

          0.8,

          -0.6,

        ],

        [

          0.1_f32,

          -1.5,

          1.0,

          0.4,

        ],

        [

          -0.8_f32,

          0.35,

          -1.2,

          0.9,

        ],

        [

          1.25_f32,

          0.6,

          0.2,

          -1.0,

        ],

      ];

      let lattice_spacing =

        0.35_f32;

      let spectral_scale =

        8.0_f32;

      let first_mass_profile = create_mass_step_profile_1d(

        field.len(),

        2,

        -1.25,

        0.5,

      );

      let moved_mass_profile = create_mass_step_profile_1d(

        field.len(),

        4,

        -1.25,

        0.5,

      );

      let generator = GpuScaledDiracGenerator1d::new(

        &device,

        field.len() as u32,

        lattice_spacing,

        spectral_scale,

      );

      let input_buffer = create_input_buffer(

        &device,

        &field,

      );

      let output_buffer = create_output_buffer(

        &device,

        field.len(),

      );

      let binding = generator.create_binding(

        &device,

        &input_buffer,

        &output_buffer,

      );

      let first_actual = dispatch_generator(

        &device,

        &queue,

        &generator,

        &binding,

        &output_buffer,

        &first_mass_profile,

      );

      let first_expected = scaled_cpu_reference(

        &field,

        lattice_spacing,

        &first_mass_profile,

        spectral_scale,

      );

      assert_f32_slices_approximately_equal(

        &first_actual,

        &first_expected,

      );

      let moved_actual = dispatch_generator(

        &device,

        &queue,

        &generator,

        &binding,

        &output_buffer,

        &moved_mass_profile,

      );

      let moved_expected = scaled_cpu_reference(

        &field,

        lattice_spacing,

        &moved_mass_profile,

        spectral_scale,

      );

      assert_f32_slices_approximately_equal(

        &moved_actual,

        &moved_expected,

      );

    },

  );

}

#[test] // Verifies the locked even-grid convention that the ambiguous Nyquist first derivative is exactly suppressed.

#[ignore = "requires a native GPU adapter"]

fn gpu_scaled_dirac_generator_zeroes_even_grid_nyquist_kinetic_mode() {

  pollster::block_on(

    async {

      let instance =

        gpu::gpu_context::create_instance();

      let adapter = gpu::gpu_context::request_adapter(

        &instance,

      )

      .await;

      let (

        device,

        queue,

      ) = gpu::gpu_context::request_device_and_queue(

        &adapter,

      )

      .await;

      let positive_state = [

        1.0_f32,

        -2.0,

        0.5,

        3.0,

      ];

      let negative_state = [

        -1.0_f32,

        2.0,

        -0.5,

        -3.0,

      ];

      let field = [

        positive_state,

        negative_state,

        positive_state,

        negative_state,

      ];

      let lattice_spacing =

        0.5_f32;

      let spectral_scale =

        3.0_f32;

      let mass_profile =

        vec![

          0.0_f32;

          field.len()

        ];

      let generator = GpuScaledDiracGenerator1d::new(

        &device,

        field.len() as u32,

        lattice_spacing,

        spectral_scale,

      );

      let input_buffer = create_input_buffer(

        &device,

        &field,

      );

      let output_buffer = create_output_buffer(

        &device,

        field.len(),

      );

      let binding = generator.create_binding(

        &device,

        &input_buffer,

        &output_buffer,

      );

      let actual = dispatch_generator(

        &device,

        &queue,

        &generator,

        &binding,

        &output_buffer,

        &mass_profile,

      );

      for (

        index,

        component,

      ) in actual

        .iter()

        .enumerate()

      {

        assert!(

          component.abs()

            <= FLOAT_TOLERANCE,

          "Nyquist kinetic output should be zero at flattened component {index}, but got {component}",

        );

      }

    },

  );

}

#[test] // Verifies generator dispatch beyond one 64-thread workgroup while retaining the correct local mass and scaled output.

#[ignore = "requires a native GPU adapter"]

fn gpu_scaled_dirac_generator_updates_points_beyond_one_workgroup() {

  pollster::block_on(

    async {

      let instance =

        gpu::gpu_context::create_instance();

      let adapter = gpu::gpu_context::request_adapter(

        &instance,

      )

      .await;

      let (

        device,

        queue,

      ) = gpu::gpu_context::request_device_and_queue(

        &adapter,

      )

      .await;

      let point_count =

        65_usize;

      let lattice_spacing =

        0.25_f32;

      let spectral_scale =

        6.0_f32;

      let field = vec![

        [

          0.5_f32,

          -1.0,

          2.0,

          -0.25,

        ];

        point_count

      ];

      let mass_profile = create_mass_step_profile_1d(

        point_count,

        32,

        -0.75,

        1.25,

      );

      let generator = GpuScaledDiracGenerator1d::new(

        &device,

        point_count as u32,

        lattice_spacing,

        spectral_scale,

      );

      let input_buffer = create_input_buffer(

        &device,

        &field,

      );

      let output_buffer = create_output_buffer(

        &device,

        point_count,

      );

      let binding = generator.create_binding(

        &device,

        &input_buffer,

        &output_buffer,

      );

      let actual = dispatch_generator(

        &device,

        &queue,

        &generator,

        &binding,

        &output_buffer,

        &mass_profile,

      );

      let expected = scaled_cpu_reference(

        &field,

        lattice_spacing,

        &mass_profile,

        spectral_scale,

      );

      assert_f32_slices_approximately_equal(

        &actual,

        &expected,

      );

    },

  );

}