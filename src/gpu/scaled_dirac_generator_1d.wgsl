struct GeneratorParameters {
  line_length: u32,
  total_point_count: u32,
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
var<storage, read> derivative_matrix:
  array<f32>;

@group(0)
@binding(4)
var<uniform> parameters:
  GeneratorParameters;

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

  if point_index >= parameters.total_point_count {
    return;
  }

  let output_x =
    point_index
    % parameters.line_length;

  let line_start =
    point_index
    - output_x;

  let derivative_row_start =
    output_x
    * parameters.line_length;

  var spatial_derivative =
    vec4<f32>(
      0.0,
      0.0,
      0.0,
      0.0,
    );

  for (
    var source_x = 0u;
    source_x < parameters.line_length;
    source_x = source_x + 1u
  ) {
    let source_point_index =
      line_start
      + source_x;

    let matrix_index =
      derivative_row_start
      + source_x;

    spatial_derivative =
      spatial_derivative
      + derivative_matrix[matrix_index]
      * input_field[source_point_index];
  }

  let state =
    input_field[point_index];

  let local_mass =
    mass_profile[output_x];

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