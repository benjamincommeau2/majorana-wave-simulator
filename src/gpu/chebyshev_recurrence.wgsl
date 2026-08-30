struct StepParameters {
  coefficient: f32,
  point_count: u32,
  padding: vec2<u32>,
}

@group(0)
@binding(0)
var<storage, read_write> previous_basis: array<vec4<f32>>;

@group(0)
@binding(1)
var<storage, read> generator_scratch: array<vec4<f32>>;

@group(0)
@binding(2)
var<storage, read_write> accumulator: array<vec4<f32>>;

@group(0)
@binding(3)
var<uniform> step_parameters: StepParameters;

@compute
@workgroup_size(64)
fn main(
  @builtin(global_invocation_id) global_id: vec3<u32>,
) {
  let point_index =
    global_id.x;

  if point_index >= step_parameters.point_count {
    return;
  }

  let next_basis =
    2.0
    * generator_scratch[point_index]
    + previous_basis[point_index];

  previous_basis[point_index] =
    next_basis;

  accumulator[point_index] =
    accumulator[point_index]
    + step_parameters.coefficient
    * next_basis;
}