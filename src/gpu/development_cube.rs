#[cfg(any(target_arch = "wasm32", test))]

mod mouse_drag_rotation;

#[cfg(target_arch = "wasm32")]

mod render_spatial_majorana_field_frame;

#[cfg(target_arch = "wasm32")]

use std::cell::RefCell;

#[cfg(target_arch = "wasm32")]

use std::rc::Rc;

#[cfg(target_arch = "wasm32")]

use wasm_bindgen::closure::Closure;

#[cfg(target_arch = "wasm32")]

use wasm_bindgen::JsCast;

#[cfg(target_arch = "wasm32")]

pub fn start_mouse_rotation(

  canvas: web_sys::HtmlCanvasElement,

  surface: wgpu::Surface<'static>,

  device: wgpu::Device,

  queue: wgpu::Queue,

  pipeline: wgpu::RenderPipeline,

  spatial_majorana_field_buffer: wgpu::Buffer,

) {

  const SIDE_LENGTH: usize = 16;

  const LATTICE_SPACING: f32 = 1.0;

  const MAXIMUM_MASS_MAGNITUDE: f32 = 1.0;

  const LEFT_MASS: f32 = -1.0;

  const RIGHT_MASS: f32 = 1.0;

  const INITIAL_MASS_BOUNDARY_INDEX: usize =
    SIDE_LENGTH
    / 2;

  const PHYSICS_DT: f64 = 0.01;

  const PLAYBACK_RATE: f64 = 1.0;

  const MAX_PHYSICS_STEPS_PER_FRAME: usize = 4;

  let rotation_buffer = device.create_buffer(

    &wgpu::BufferDescriptor {

      label: Some(

        "Development Cube Rotation Buffer",

      ),

      size: 16,

      usage:

        wgpu::BufferUsages::UNIFORM

        | wgpu::BufferUsages::COPY_DST,

      mapped_at_creation: false,

    },

  );

  let field_bind_group = crate::gpu::bind_groups::create_spatial_majorana_field_render_bind_group(

    &device,

    &pipeline,

    &rotation_buffer,

    &spatial_majorana_field_buffer,

  );

  let drag_state = mouse_drag_rotation::attach_mouse_drag_rotation(

    &canvas,

  );

  let spectral_scale = crate::physics::spectral_bound::conservative_dirac_spectral_scale_1d(

    SIDE_LENGTH,

    LATTICE_SPACING,

    MAXIMUM_MASS_MAGNITUDE,

  );

  let propagation_setup = crate::physics::chebyshev_propagation_setup::ChebyshevPropagationSetup::new(

    spectral_scale as f64,

    PHYSICS_DT,

  );

  let x_line_count =

    SIDE_LENGTH

    * SIDE_LENGTH;

  let mut physics_propagator = crate::gpu::chebyshev_propagator_1d::GpuChebyshevPropagator1d::new_batched_x_lines(

    &device,

    SIDE_LENGTH as u32,

    x_line_count as u32,

    LATTICE_SPACING,

    &propagation_setup,

  );

  let mass_profile = crate::physics::mass_profile::create_mass_step_profile_1d(

    SIDE_LENGTH,

    INITIAL_MASS_BOUNDARY_INDEX,

    LEFT_MASS,

    RIGHT_MASS,

  );

  queue.write_buffer(

    physics_propagator.mass_profile_buffer(),

    0,

    bytemuck::cast_slice(

      &mass_profile,

    ),

  );

  let mut simulation_clock = crate::simulation_clock::SimulationClock::new(

    PHYSICS_DT,

    PLAYBACK_RATE,

    MAX_PHYSICS_STEPS_PER_FRAME,

  );

  let animation_callback: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(

    RefCell::new(

      None,

    ),

  );

  let animation_callback_for_start =

    animation_callback.clone();

  *animation_callback_for_start.borrow_mut() = Some(

    Closure::<dyn FnMut(f64)>::new(

      move |timestamp_ms: f64| {

        let scheduled_physics_steps = simulation_clock.steps_for_frame(

          timestamp_ms,

        );

        if scheduled_physics_steps > 0 {

          let mut physics_encoder = crate::gpu::commands::create_command_encoder(

            &device,

          );

          for _ in 0..scheduled_physics_steps {

            physics_propagator.record_step(

              &mut physics_encoder,

              &spatial_majorana_field_buffer,

            );

          }

          crate::gpu::commands::submit_commands(

            &queue,

            physics_encoder,

          );

        }

        let rotation_values = {

          let state =

            drag_state.borrow();

          let [

            yaw,

            pitch,

          ] = state.angles();

          [

            yaw,

            pitch,

            0.0,

            0.0,

          ]

        };

        render_spatial_majorana_field_frame::render_spatial_majorana_field_frame(

          &surface,

          &device,

          &queue,

          &pipeline,

          &rotation_buffer,

          &field_bind_group,

          &rotation_values,

        );

        let window = web_sys::window()

          .expect(

            "Could not get browser window for cube animation",

          );

        window.request_animation_frame(

          animation_callback

            .borrow()

            .as_ref()

            .expect(

              "Cube animation callback disappeared",

            )

            .as_ref()

            .unchecked_ref(),

        )

        .expect(

          "Could not request the next cube animation frame",

        );

      },

    ),

  );

  let window = web_sys::window()

    .expect(

      "Could not get browser window to start cube animation",

    );

  window.request_animation_frame(

    animation_callback_for_start

      .borrow()

      .as_ref()

      .expect(

        "Cube animation callback was not created",

      )

      .as_ref()

      .unchecked_ref(),

  )

  .expect(

    "Could not start cube animation",

  );

}