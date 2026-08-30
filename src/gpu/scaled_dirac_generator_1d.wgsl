const PI: f32 =
  3.14159265358979323846;

struct GeneratorParameters {
  point_count: u32,
  lattice_spacing: f32,
  inverse_spectral_scale: f32,
  padding: u32,
}

@group(0)
@binding(0)
var<storage, read> input_field:
  array<vec4<f32>>;

@group(0)
@binding(1)
var<storage, read_write> output_field:
  array<vec4<f32>>;

@group(0)
@binding(2)
var<storage, read> mass_profile:
  array<f32>;

@group(0)
@binding(3)
var<uniform> parameters:
  GeneratorParameters;

fn spectral_derivative_coefficient(
  output_index: u32,
  source_index: u32,
) -> f32 {
  if output_index == source_index {
    return 0.0;
  }

  let signed_difference =
    i32(output_index)
    - i32(source_index);

  let angle =
    PI
    * f32(signed_difference)
    / f32(parameters.point_count);

  var alternating_sign =
    -1.0;

  if (
    abs(signed_difference)
    % 2
    == 0
  ) {
    alternating_sign =
      1.0;
  }

  let derivative_scale =
    PI
    / (
      f32(parameters.point_count)
      * parameters.lattice_spacing
    );

  let sine =
    sin(angle);

  let grid_is_even =
    parameters.point_count
    % 2u
    == 0u;

  if grid_is_even {
    return
      derivative_scale
      * alternating_sign
      * cos(angle)
      / sine;
  }

  return
    derivative_scale
    * alternating_sign
    / sine;
}

fn apply_minus_alpha_x(
  derivative_state: vec4<f32>,
) -> vec4<f32> {
  return vec4<f32>(
    -derivative_state.z,
    -derivative_state.w,
    -derivative_state.x,
    -derivative_state.y,
  );
}

fn apply_mass_generator(
  state: vec4<f32>,
  mass: f32,
) -> vec4<f32> {
  return mass
    * vec4<f32>(
      -state.z,
      state.w,
      state.x,
      -state.y,
    );
}

@compute
@workgroup_size(64)
fn main(
  @builtin(global_invocation_id)
  global_id: vec3<u32>,
) {
  let point_index =
    global_id.x;

  if point_index >= parameters.point_count {
    return;
  }

  var spatial_derivative =
    vec4<f32>(
      0.0,
      0.0,
      0.0,
      0.0,
    );

  for (
    var source_index = 0u;
    source_index < parameters.point_count;
    source_index = source_index + 1u
  ) {
    let derivative_coefficient =
      spectral_derivative_coefficient(
        point_index,
        source_index,
      );

    spatial_derivative =
      spatial_derivative
      + derivative_coefficient
      * input_field[source_index];
  }

  let state =
    input_field[point_index];

  let local_mass =
    mass_profile[point_index];

  let kinetic_generator =
    apply_minus_alpha_x(
      spatial_derivative,
    );

  let mass_generator =
    apply_mass_generator(
      state,
      local_mass,
    );

  output_field[point_index] =
    parameters.inverse_spectral_scale
    * (
      kinetic_generator
      + mass_generator
    );
}