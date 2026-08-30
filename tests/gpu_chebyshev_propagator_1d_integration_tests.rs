use majorana_wave_simulator::gpu;

use majorana_wave_simulator::gpu::chebyshev_propagator_1d::GpuChebyshevPropagator1d;

use majorana_wave_simulator::physics::chebyshev_propagation_setup::ChebyshevPropagationSetup;

use majorana_wave_simulator::physics::chebyshev_propagator::direct_real_chebyshev_propagate_with_mass_profile_1d;

use majorana_wave_simulator::physics::mass_profile::create_mass_step_profile_1d;

use wgpu::util::DeviceExt;

const COMPONENTS_PER_POINT: usize = 4;

const FLOAT_TOLERANCE: f32 = 2.0e-4_f32;

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

        "GPU Chebyshev Propagator Integration Test State Buffer",

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

        "GPU Chebyshev Propagator Integration Test Readback Buffer",

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

          "Could not send GPU Chebyshev propagator map result",

        );

    },

  );

  device

    .poll(

      wgpu::PollType::wait_indefinitely(),

    )

    .expect(

      "Could not wait for GPU Chebyshev propagator readback",

    );

  receiver

    .recv()

    .expect(

      "GPU Chebyshev propagator mapping callback did not report a result",

    )

    .expect(

      "GPU Chebyshev propagator readback buffer could not be mapped",

    );

  let mapped_range = readback_buffer

    .slice(..)

    .get_mapped_range()

    .expect(

      "Could not access GPU Chebyshev propagator mapped bytes",

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

#[test] // Verifies one complete GPU Chebyshev physics step against the already-tested CPU mass-profile oracle.

#[ignore = "requires a native GPU adapter"]

fn gpu_chebyshev_propagator_matches_cpu_for_one_complete_step() {

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

          -0.25,

          0.5,

          0.75,

        ],

        [

          -0.6_f32,

          0.9,

          1.1,

          -0.2,

        ],

        [

          0.4_f32,

          -1.2,

          0.3,

          1.5,

        ],

        [

          1.25_f32,

          0.15,

          -0.7,

          0.2,

        ],

        [

          -0.35_f32,

          0.6,

          0.85,

          -1.0,

        ],

      ];

      let lattice_spacing =

        0.5_f32;

      let spectral_scale =

        6.0_f64;

      let physics_dt =

        0.05_f64;

      let mass_profile =

        create_mass_step_profile_1d(

          field.len(),

          2,

          -0.75,

          0.5,

        );

      let setup =

        ChebyshevPropagationSetup::new(

          spectral_scale,

          physics_dt,

        );

      let expected = direct_real_chebyshev_propagate_with_mass_profile_1d(

        &field,

        lattice_spacing,

        &mass_profile,

        setup.spectral_scale() as f32,

        setup.coefficients(),

      );

      let mut propagator = GpuChebyshevPropagator1d::new(

        &device,

        field.len() as u32,

        lattice_spacing,

        &setup,

      );

      let state_buffer = create_state_buffer(

        &device,

        &field,

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

          field.len(),

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

#[test] // Verifies two chronological GPU steps where a moved mass wall acts on the previously propagated state rather than reinitializing it.

#[ignore = "requires a native GPU adapter"]

fn gpu_chebyshev_propagator_preserves_state_when_mass_boundary_moves() {

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

      let initial_field = [

        [

          0.75_f32,

          -0.5,

          0.25,

          1.0,

        ],

        [

          -0.25_f32,

          0.8,

          1.2,

          -0.4,

        ],

        [

          1.0_f32,

          -0.75,

          0.5,

          0.2,

        ],

        [

          0.3_f32,

          1.1,

          -0.9,

          0.6,

        ],

        [

          -0.8_f32,

          0.4,

          0.7,

          -1.1,

        ],

        [

          1.25_f32,

          -0.1,

          0.35,

          0.9,

        ],

      ];

      let lattice_spacing =

        0.35_f32;

      let spectral_scale =

        8.0_f64;

      let physics_dt =

        0.05_f64;

      let first_mass_profile =

        create_mass_step_profile_1d(

          initial_field.len(),

          2,

          -1.25,

          0.5,

        );

      let moved_mass_profile =

        create_mass_step_profile_1d(

          initial_field.len(),

          4,

          -1.25,

          0.5,

        );

      let setup =

        ChebyshevPropagationSetup::new(

          spectral_scale,

          physics_dt,

        );

      assert_eq!(

        setup.max_order(),

        6,

        "This test deliberately uses an odd number of recurrence role swaps so the second physics step exercises recurrence reset.",

      );

      let cpu_after_first_step = direct_real_chebyshev_propagate_with_mass_profile_1d(

        &initial_field,

        lattice_spacing,

        &first_mass_profile,

        setup.spectral_scale() as f32,

        setup.coefficients(),

      );

      let cpu_after_second_step = direct_real_chebyshev_propagate_with_mass_profile_1d(

        &cpu_after_first_step,

        lattice_spacing,

        &moved_mass_profile,

        setup.spectral_scale() as f32,

        setup.coefficients(),

      );

      let mut propagator = GpuChebyshevPropagator1d::new(

        &device,

        initial_field.len() as u32,

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

          &first_mass_profile,

        ),

      );

      record_and_submit_step(

        &device,

        &queue,

        &mut propagator,

        &state_buffer,

      );

      queue.write_buffer(

        propagator.mass_profile_buffer(),

        0,

        bytemuck::cast_slice(

          &moved_mass_profile,

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

      let expected = flatten_field(

        &cpu_after_second_step,

      );

      assert_f32_slices_approximately_equal(

        &actual,

        &expected,

      );

    },

  );

}