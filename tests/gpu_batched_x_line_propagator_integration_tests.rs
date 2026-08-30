use majorana_wave_simulator::gpu;

use majorana_wave_simulator::gpu::chebyshev_propagator_1d::GpuChebyshevPropagator1d;

use majorana_wave_simulator::physics::chebyshev_propagation_setup::ChebyshevPropagationSetup;

use majorana_wave_simulator::physics::chebyshev_propagator::direct_real_chebyshev_propagate_with_mass_profile_1d;

use majorana_wave_simulator::physics::mass_profile::create_mass_step_profile_1d;

use majorana_wave_simulator::physics::spectral_bound::conservative_dirac_spectral_scale_1d;

use wgpu::util::DeviceExt;

const COMPONENTS_PER_POINT: usize = 4;

const FLOAT_TOLERANCE: f32 = 4.0e-4_f32;

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

fn flatten_field(

  field: &[[f32; COMPONENTS_PER_POINT]],

) -> Vec<f32> {

  field

    .iter()

    .flat_map(

      |state| state.iter().copied(),

    )

    .collect()

}

fn field_buffer_size(

  point_count: usize,

) -> wgpu::BufferAddress {

  point_count as wgpu::BufferAddress

    * COMPONENTS_PER_POINT as wgpu::BufferAddress

    * std::mem::size_of::<f32>() as wgpu::BufferAddress

}

fn create_state_buffer(

  device: &wgpu::Device,

  field: &[[f32; COMPONENTS_PER_POINT]],

) -> wgpu::Buffer {

  device.create_buffer_init(

    &wgpu::util::BufferInitDescriptor {

      label: Some(

        "Batched X-Line Propagator Integration Test State Buffer",

      ),

      contents: bytemuck::cast_slice(

        field,

      ),

      usage:

        wgpu::BufferUsages::STORAGE

        | wgpu::BufferUsages::COPY_SRC

        | wgpu::BufferUsages::COPY_DST,

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

        "Batched X-Line Propagator Integration Test Readback Buffer",

      ),

      size:

        buffer_size,

      usage:

        wgpu::BufferUsages::COPY_DST

        | wgpu::BufferUsages::MAP_READ,

      mapped_at_creation:

        false,

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

          "Could not send batched x-line GPU map result",

        );

    },

  );

  device

    .poll(

      wgpu::PollType::wait_indefinitely(),

    )

    .expect(

      "Could not wait for batched x-line GPU readback",

    );

  receiver

    .recv()

    .expect(

      "Batched x-line GPU mapping callback did not report a result",

    )

    .expect(

      "Batched x-line GPU readback buffer could not be mapped",

    );

  let mapped_range = readback_buffer

    .slice(..)

    .get_mapped_range()

    .expect(

      "Could not access batched x-line mapped GPU bytes",

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

fn create_distinct_x_lines(

  line_length: usize,

  line_count: usize,

) -> Vec<[f32; COMPONENTS_PER_POINT]> {

  let mut field = Vec::with_capacity(

    line_length

    * line_count,

  );

  for line_index in 0..line_count {

    let line_scale =

      0.002_f32

      * (

        line_index as f32

        + 1.0

      );

    for x in 0..line_length {

      let x_value =

        x as f32

        + 1.0;

      field.push(

        [

          line_scale

            * x_value,

          -0.5

            * line_scale

            * x_value,

          line_scale

            * (

              0.25

              + x_value

            ),

          -line_scale

            * (

              0.75

              + 0.5

              * x_value

            ),

        ],

      );

    }

  }

  field

}

fn cpu_propagate_independent_x_lines(

  field: &[[f32; COMPONENTS_PER_POINT]],

  line_length: usize,

  lattice_spacing: f32,

  mass_profile: &[f32],

  setup: &ChebyshevPropagationSetup,

) -> Vec<[f32; COMPONENTS_PER_POINT]> {

  assert_eq!(

    field.len()

      % line_length,

    0,

  );

  let mut propagated = Vec::with_capacity(

    field.len(),

  );

  for x_line in field.chunks_exact(

    line_length,

  ) {

    let propagated_line = direct_real_chebyshev_propagate_with_mass_profile_1d(

      x_line,

      lattice_spacing,

      mass_profile,

      setup.spectral_scale() as f32,

      setup.coefficients(),

    );

    propagated.extend(

      propagated_line,

    );

  }

  propagated

}

fn record_and_submit_step(

  device: &wgpu::Device,

  queue: &wgpu::Queue,

  propagator: &mut GpuChebyshevPropagator1d,

  state_buffer: &wgpu::Buffer,

) {

  let mut encoder =

    gpu::commands::create_command_encoder(

      device,

    );

  propagator.record_step(

    &mut encoder,

    state_buffer,

  );

  gpu::commands::submit_commands(

    queue,

    encoder,

  );

}

#[test] // Verifies the browser-sized 16^3 field as 256 independent contiguous x-lines against the CPU oracle.

#[ignore = "requires a native GPU adapter"]

fn gpu_batched_x_lines_match_cpu_for_full_sixteen_cubed_field() {

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

      let side_length =

        16_usize;

      let line_length =

        side_length;

      let line_count =

        side_length

        * side_length;

      let total_point_count =

        line_length

        * line_count;

      assert_eq!(

        total_point_count,

        4096,

      );

      let lattice_spacing =

        0.5_f32;

      let maximum_mass_magnitude =

        1.0_f32;

      let physics_dt =

        0.02_f64;

      let spectral_scale = conservative_dirac_spectral_scale_1d(

        line_length,

        lattice_spacing,

        maximum_mass_magnitude,

      ) as f64;

      let setup =

        ChebyshevPropagationSetup::new(

          spectral_scale,

          physics_dt,

        );

      let initial_field = create_distinct_x_lines(

        line_length,

        line_count,

      );

      let mass_profile = create_mass_step_profile_1d(

        line_length,

        7,

        -0.75,

        1.0,

      );

      let expected = cpu_propagate_independent_x_lines(

        &initial_field,

        line_length,

        lattice_spacing,

        &mass_profile,

        &setup,

      );

      let mut propagator = GpuChebyshevPropagator1d::new_batched_x_lines(

        &device,

        line_length as u32,

        line_count as u32,

        lattice_spacing,

        &setup,

      );

      let state_buffer = create_state_buffer(

        &device,

        &initial_field,

      );

      queue.write_buffer(

        propagator.mass_profile_buffer(),

        0,

        bytemuck::cast_slice(

          &mass_profile,

        ),

      );

      record_and_submit_step(

        &device,

        &queue,

        &mut propagator,

        &state_buffer,

      );

      let actual = read_gpu_f32_buffer(

        &device,

        &queue,

        &state_buffer,

        field_buffer_size(

          total_point_count,

        ),

      );

      let expected = flatten_field(

        &expected,

      );

      assert_f32_slices_approximately_equal(

        &actual,

        &expected,

      );

    },

  );

}

#[test] // Verifies that spectral differentiation never crosses from one x-line into the next contiguous line.

#[ignore = "requires a native GPU adapter"]

fn gpu_batched_x_lines_do_not_couple_neighboring_lines() {

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

      let line_length =

        5_usize;

      let line_count =

        3_usize;

      let lattice_spacing =

        0.4_f32;

      let physics_dt =

        0.03_f64;

      let maximum_mass_magnitude =

        0.0_f32;

      let spectral_scale = conservative_dirac_spectral_scale_1d(

        line_length,

        lattice_spacing,

        maximum_mass_magnitude,

      ) as f64;

      let setup =

        ChebyshevPropagationSetup::new(

          spectral_scale,

          physics_dt,

        );

      let mut initial_field = vec![

        [

          0.0_f32;

          COMPONENTS_PER_POINT

        ];

        line_length

          * line_count

      ];

      for x in 0..line_length {

        initial_field[x] = [

          x as f32

            + 1.0,

          0.0,

          0.0,

          0.0,

        ];

      }

      let mass_profile = vec![

        0.0_f32;

        line_length

      ];

      let mut propagator = GpuChebyshevPropagator1d::new_batched_x_lines(

        &device,

        line_length as u32,

        line_count as u32,

        lattice_spacing,

        &setup,

      );

      let state_buffer = create_state_buffer(

        &device,

        &initial_field,

      );

      queue.write_buffer(

        propagator.mass_profile_buffer(),

        0,

        bytemuck::cast_slice(

          &mass_profile,

        ),

      );

      record_and_submit_step(

        &device,

        &queue,

        &mut propagator,

        &state_buffer,

      );

      let actual = read_gpu_f32_buffer(

        &device,

        &queue,

        &state_buffer,

        field_buffer_size(

          initial_field.len(),

        ),

      );

      let first_zero_line_start =

        line_length

        * COMPONENTS_PER_POINT;

      for flattened_index in first_zero_line_start..actual.len() {

        assert!(

          actual[flattened_index].abs()

            <= FLOAT_TOLERANCE,

          "inactive x-lines should remain zero, but flattened component {flattened_index} became {}",

          actual[flattened_index],

        );

      }

    },

  );

}