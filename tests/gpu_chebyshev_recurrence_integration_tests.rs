use majorana_wave_simulator::gpu;

use majorana_wave_simulator::gpu::chebyshev_recurrence::GpuChebyshevRecurrence;

const COMPONENTS_PER_POINT: usize = 4;

const FLOAT_TOLERANCE: f32 = 1.0e-6_f32;

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

fn flatten_states(

  states: &[[f32; COMPONENTS_PER_POINT]],

) -> Vec<f32> {

  states

    .iter()

    .flat_map(

      |state| state.iter().copied(),

    )

    .collect()

}

fn recurrence_basis(

  previous: &[[f32; COMPONENTS_PER_POINT]],

  scaled_generator_current: &[[f32; COMPONENTS_PER_POINT]],

) -> Vec<[f32; COMPONENTS_PER_POINT]> {

  previous

    .iter()

    .zip(

      scaled_generator_current.iter(),

    )

    .map(

      |(

        previous_state,

        generator_state,

      )| {

        let mut next_state =

          [0.0_f32; COMPONENTS_PER_POINT];

        for component in 0..COMPONENTS_PER_POINT {

          next_state[component] =

            2.0

            * generator_state[component]

            + previous_state[component];

        }

        next_state

      },

    )

    .collect()

}

fn accumulate_basis(

  accumulator: &mut [[f32; COMPONENTS_PER_POINT]],

  coefficient: f32,

  basis: &[[f32; COMPONENTS_PER_POINT]],

) {

  for point in 0..accumulator.len() {

    for component in 0..COMPONENTS_PER_POINT {

      accumulator[point][component] +=

        coefficient

        * basis[point][component];

    }

  }

}

fn buffer_size_for_point_count(

  point_count: u32,

) -> wgpu::BufferAddress {

  point_count as wgpu::BufferAddress

    * COMPONENTS_PER_POINT as wgpu::BufferAddress

    * std::mem::size_of::<f32>() as wgpu::BufferAddress

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

        "Chebyshev Recurrence Integration Test Readback Buffer",

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

          "Could not send Chebyshev recurrence GPU map result",

        );

    },

  );

  device

    .poll(

      wgpu::PollType::wait_indefinitely(),

    )

    .expect(

      "Could not wait for Chebyshev recurrence GPU readback",

    );

  receiver

    .recv()

    .expect(

      "Chebyshev recurrence GPU mapping callback did not report a result",

    )

    .expect(

      "Chebyshev recurrence GPU readback buffer could not be mapped",

    );

  let mapped_range = readback_buffer

    .slice(..)

    .get_mapped_range()

    .expect(

      "Could not access Chebyshev recurrence mapped GPU bytes",

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

#[test] // Verifies two chronological recurrence steps, basis-buffer role rotation, and coefficient accumulation on the real GPU.

#[ignore = "requires a native GPU adapter"]

fn gpu_chebyshev_recurrence_reuses_basis_buffers_across_orders() {

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

      let coefficients = [

        1.0_f64,

        0.5_f64,

        0.25_f64,

        -0.5_f64,

      ];

      let point_count =

        2_u32;

      let mut recurrence = GpuChebyshevRecurrence::new(

        &device,

        point_count,

        &coefficients,

      );

      let phi_zero = [

        [

          1.0_f32,

          2.0,

          3.0,

          4.0,

        ],

        [

          -1.0_f32,

          0.5,

          2.0,

          -3.0,

        ],

      ];

      let phi_one = [

        [

          0.25_f32,

          -1.0,

          1.5,

          2.0,

        ],

        [

          3.0_f32,

          -2.0,

          0.75,

          1.0,

        ],

      ];

      let scaled_generator_phi_one = [

        [

          0.5_f32,

          1.0,

          -0.5,

          2.0,

        ],

        [

          -1.0_f32,

          0.25,

          1.5,

          -0.75,

        ],

      ];

      let scaled_generator_phi_two = [

        [

          -0.25_f32,

          0.5,

          1.0,

          -1.5,

        ],

        [

          0.75_f32,

          -1.25,

          0.5,

          2.0,

        ],

      ];

      let mut expected_accumulator = [

        [

          0.1_f32,

          0.2,

          0.3,

          0.4,

        ],

        [

          -0.1_f32,

          -0.2,

          -0.3,

          -0.4,

        ],

      ];

      queue.write_buffer(

        recurrence.previous_basis_buffer(),

        0,

        bytemuck::cast_slice(

          &phi_zero,

        ),

      );

      queue.write_buffer(

        recurrence.current_basis_buffer(),

        0,

        bytemuck::cast_slice(

          &phi_one,

        ),

      );

      queue.write_buffer(

        recurrence.generator_scratch_buffer(),

        0,

        bytemuck::cast_slice(

          &scaled_generator_phi_one,

        ),

      );

      queue.write_buffer(

        recurrence.accumulator_buffer(),

        0,

        bytemuck::cast_slice(

          &expected_accumulator,

        ),

      );

      let expected_phi_two = recurrence_basis(

        &phi_zero,

        &scaled_generator_phi_one,

      );

      accumulate_basis(

        &mut expected_accumulator,

        coefficients[2] as f32,

        &expected_phi_two,

      );

      let mut first_encoder =

        gpu::commands::create_command_encoder(

          &device,

        );

      recurrence.record_step(

        &mut first_encoder,

        2,

      );

      gpu::commands::submit_commands(

        &queue,

        first_encoder,

      );

      queue.write_buffer(

        recurrence.generator_scratch_buffer(),

        0,

        bytemuck::cast_slice(

          &scaled_generator_phi_two,

        ),

      );

      let expected_phi_three = recurrence_basis(

        &phi_one,

        &scaled_generator_phi_two,

      );

      accumulate_basis(

        &mut expected_accumulator,

        coefficients[3] as f32,

        &expected_phi_three,

      );

      let mut second_encoder =

        gpu::commands::create_command_encoder(

          &device,

        );

      recurrence.record_step(

        &mut second_encoder,

        3,

      );

      gpu::commands::submit_commands(

        &queue,

        second_encoder,

      );

      let buffer_size = buffer_size_for_point_count(

        point_count,

      );

      let gpu_current_basis = read_gpu_f32_buffer(

        &device,

        &queue,

        recurrence.current_basis_buffer(),

        buffer_size,

      );

      let gpu_accumulator = read_gpu_f32_buffer(

        &device,

        &queue,

        recurrence.accumulator_buffer(),

        buffer_size,

      );

      let expected_current_basis = flatten_states(

        &expected_phi_three,

      );

      let expected_accumulator = flatten_states(

        &expected_accumulator,

      );

      assert_f32_slices_approximately_equal(

        &gpu_current_basis,

        &expected_current_basis,

      );

      assert_f32_slices_approximately_equal(

        &gpu_accumulator,

        &expected_accumulator,

      );

    },

  );

}

#[test] // Verifies dispatch sizing across a workgroup boundary so every field point is updated, including point 65.

#[ignore = "requires a native GPU adapter"]

fn gpu_chebyshev_recurrence_updates_points_beyond_one_workgroup() {

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

      let coefficients = [

        0.0_f64,

        0.0_f64,

        1.0_f64,

      ];

      let point_count =

        65_u32;

      let mut recurrence = GpuChebyshevRecurrence::new(

        &device,

        point_count,

        &coefficients,

      );

      let mut previous_basis =

        Vec::with_capacity(

          point_count as usize,

        );

      let mut generator_scratch =

        Vec::with_capacity(

          point_count as usize,

        );

      let zero_accumulator = vec![

        [

          0.0_f32;

          COMPONENTS_PER_POINT

        ];

        point_count as usize

      ];

      for point in 0..point_count {

        let value =

          point as f32

          + 1.0;

        previous_basis.push(

          [

            value,

            -value,

            value * 0.5,

            value * 2.0,

          ],

        );

        generator_scratch.push(

          [

            0.25 * value,

            0.5 * value,

            -0.75 * value,

            value,

          ],

        );

      }

      queue.write_buffer(

        recurrence.previous_basis_buffer(),

        0,

        bytemuck::cast_slice(

          &previous_basis,

        ),

      );

      queue.write_buffer(

        recurrence.generator_scratch_buffer(),

        0,

        bytemuck::cast_slice(

          &generator_scratch,

        ),

      );

      queue.write_buffer(

        recurrence.accumulator_buffer(),

        0,

        bytemuck::cast_slice(

          &zero_accumulator,

        ),

      );

      let expected_next_basis = recurrence_basis(

        &previous_basis,

        &generator_scratch,

      );

      let mut encoder =

        gpu::commands::create_command_encoder(

          &device,

        );

      recurrence.record_step(

        &mut encoder,

        2,

      );

      gpu::commands::submit_commands(

        &queue,

        encoder,

      );

      let gpu_current_basis = read_gpu_f32_buffer(

        &device,

        &queue,

        recurrence.current_basis_buffer(),

        buffer_size_for_point_count(

          point_count,

        ),

      );

      let expected_current_basis = flatten_states(

        &expected_next_basis,

      );

      assert_f32_slices_approximately_equal(

        &gpu_current_basis,

        &expected_current_basis,

      );

    },

  );

}