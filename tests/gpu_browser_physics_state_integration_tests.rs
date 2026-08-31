use majorana_wave_simulator::gpu;

use majorana_wave_simulator::gpu::chebyshev_propagator_1d::GpuChebyshevPropagator1d;

use majorana_wave_simulator::physics::chebyshev_propagation_setup::ChebyshevPropagationSetup;

use majorana_wave_simulator::physics::chebyshev_propagator::direct_real_chebyshev_propagate_with_mass_profile_1d;

use majorana_wave_simulator::physics::mass_profile::create_mass_step_profile_1d;

use majorana_wave_simulator::physics::spectral_bound::conservative_dirac_spectral_scale_1d;

use majorana_wave_simulator::spatial_majorana_field::SpatialMajoranaField;

const COMPONENTS_PER_POINT: usize = 4;

const FLOAT_TOLERANCE: f32 = 1.5e-3_f32;

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

fn read_gpu_f32_buffer(

  device: &wgpu::Device,

  queue: &wgpu::Queue,

  source_buffer: &wgpu::Buffer,

  buffer_size: wgpu::BufferAddress,

) -> Vec<f32> {

  let readback_buffer = device.create_buffer(

    &wgpu::BufferDescriptor {

      label: Some(

        "Browser Physics State Integration Test Readback Buffer",

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

          "Could not send browser physics GPU map result",

        );

    },

  );

  device

    .poll(

      wgpu::PollType::wait_indefinitely(),

    )

    .expect(

      "Could not wait for browser physics GPU readback",

    );

  receiver

    .recv()

    .expect(

      "Browser physics GPU mapping callback did not report a result",

    )

    .expect(

      "Browser physics GPU readback buffer could not be mapped",

    );

  let mapped_range = readback_buffer

    .slice(..)

    .get_mapped_range()

    .expect(

      "Could not access browser physics mapped GPU bytes",

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

fn cpu_propagate_batched_x_lines(

  initial_field: &[[f32; COMPONENTS_PER_POINT]],

  line_length: usize,

  physics_step_count: usize,

  lattice_spacing: f32,

  mass_profile: &[f32],

  setup: &ChebyshevPropagationSetup,

) -> Vec<[f32; COMPONENTS_PER_POINT]> {

  let mut current_field =

    initial_field.to_vec();

  for _ in 0..physics_step_count {

    let mut next_field = Vec::with_capacity(

      current_field.len(),

    );

    for x_line in current_field.chunks_exact(

      line_length,

    ) {

      let propagated_line = direct_real_chebyshev_propagate_with_mass_profile_1d(

        x_line,

        lattice_spacing,

        mass_profile,

        setup.spectral_scale() as f32,

        setup.coefficients(),

      );

      next_field.extend(

        propagated_line,

      );

    }

    current_field =

      next_field;

  }

  current_field

}

#[test] // Verifies that the browser render buffer can remain the GPU-resident physics state across several fixed steps recorded into one submission.

#[ignore = "requires a native GPU adapter"]

fn gpu_browser_field_buffer_advances_multiple_fixed_physics_steps() {

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

        side_length

        * side_length

        * side_length;

      let lattice_spacing =

        1.0_f32;

      let maximum_mass_magnitude =

        1.0_f32;

      let physics_dt =

        0.01_f64;

      let physics_step_count =

        3_usize;

      let spectral_scale = conservative_dirac_spectral_scale_1d(

        line_length,

        lattice_spacing,

        maximum_mass_magnitude,

      ) as f64;

      let setup = ChebyshevPropagationSetup::new(

        spectral_scale,

        physics_dt,

      );

      let initial_field = SpatialMajoranaField::new_centered_gaussian(

        side_length,

        2.0,

      );

      assert_eq!(

        initial_field.len(),

        total_point_count,

      );

      let mass_profile = create_mass_step_profile_1d(

        line_length,

        line_length / 2,

        -1.0,

        1.0,

      );

      let expected = cpu_propagate_batched_x_lines(

        initial_field.points(),

        line_length,

        physics_step_count,

        lattice_spacing,

        &mass_profile,

        &setup,

      );

      let initial_bytes = bytemuck::cast_slice(

        initial_field.points(),

      );

      let state_buffer = gpu::spatial_majorana_field_buffer::create_spatial_majorana_field_buffer(

        &device,

        initial_bytes.len() as wgpu::BufferAddress,

      );

      queue.write_buffer(

        &state_buffer,

        0,

        initial_bytes,

      );

      let mut propagator = GpuChebyshevPropagator1d::new_batched_x_lines(

        &device,

        line_length as u32,

        line_count as u32,

        lattice_spacing,

        &setup,

      );

      queue.write_buffer(

        propagator.mass_profile_buffer(),

        0,

        bytemuck::cast_slice(

          &mass_profile,

        ),

      );

      let mut encoder =

        gpu::commands::create_command_encoder(

          &device,

        );

      for _ in 0..physics_step_count {

        propagator.record_step(

          &mut encoder,

          &state_buffer,

        );

      }

      gpu::commands::submit_commands(

        &queue,

        encoder,

      );

      let actual = read_gpu_f32_buffer(

        &device,

        &queue,

        &state_buffer,

        initial_bytes.len() as wgpu::BufferAddress,

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