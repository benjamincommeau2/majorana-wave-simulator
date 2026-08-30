struct InitializationParameters {
  point_count: u32,
  coefficient_zero: f32,
  coefficient_one: f32,
  padding: u32,
}

@group(0)
@binding(0)
var<storage, read> phi_zero:
  array<vec4<f32>>;

@group(0)
@binding(1)
var<storage, read> phi_one:
  array<vec4<f32>>;

@group(0)
@binding(2)
var<storage, read_write> accumulator:
  array<vec4<f32>>;

@group(0)
@binding(3)
var<uniform> parameters:
  InitializationParameters;

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

  accumulator[point_index] =
    parameters.coefficient_zero
    * phi_zero[point_index]
    + parameters.coefficient_one
    * phi_one[point_index];
}