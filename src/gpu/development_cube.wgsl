@group(0) @binding(0) // Places the cube rotation data at bind group zero and binding zero.

var<uniform> rotation: vec4<f32>; // Stores the changing rotation angle in the first component of one sixteen-byte GPU uniform value.

const CUBE_POINTS = array<vec3<f32>, 8>( // Stores the eight corners of a cube directly inside this first rendering shader.
  vec3<f32>(-0.7, -0.7, -0.7), // Defines the lower-left-back cube corner.

  vec3<f32>( 0.7, -0.7, -0.7), // Defines the lower-right-back cube corner.

  vec3<f32>( 0.7,  0.7, -0.7), // Defines the upper-right-back cube corner.

  vec3<f32>(-0.7,  0.7, -0.7), // Defines the upper-left-back cube corner.

  vec3<f32>(-0.7, -0.7,  0.7), // Defines the lower-left-front cube corner.

  vec3<f32>( 0.7, -0.7,  0.7), // Defines the lower-right-front cube corner.

  vec3<f32>( 0.7,  0.7,  0.7), // Defines the upper-right-front cube corner.

  vec3<f32>(-0.7,  0.7,  0.7), // Defines the upper-left-front cube corner.
); // Finishes the cube-corner array.

const CUBE_EDGES = array<u32, 24>( // Stores two corner indices for each of the cube's twelve edges.
  0u, 1u, // Draws the first back-face edge.

  1u, 2u, // Draws the second back-face edge.

  2u, 3u, // Draws the third back-face edge.

  3u, 0u, // Draws the fourth back-face edge.

  4u, 5u, // Draws the first front-face edge.

  5u, 6u, // Draws the second front-face edge.

  6u, 7u, // Draws the third front-face edge.

  7u, 4u, // Draws the fourth front-face edge.

  0u, 4u, // Connects the first back corner to the front face.

  1u, 5u, // Connects the second back corner to the front face.

  2u, 6u, // Connects the third back corner to the front face.

  3u, 7u, // Connects the fourth back corner to the front face.
); // Finishes the cube-edge array.

@vertex // Declares the following function as the vertex-shader entry point.

fn vs_main( // Begins the vertex shader used to position each cube-edge endpoint.

  @builtin(vertex_index) vertex_index: u32, // Receives the automatically generated vertex number from the draw command.

) -> @builtin(position) vec4<f32> { // Returns one final clip-space position for the current cube endpoint.

  let point = CUBE_POINTS[CUBE_EDGES[vertex_index]]; // Selects the cube corner required for this line endpoint.

  let y_angle = rotation.x; // Uses the horizontal mouse-drag angle to rotate the cube around its vertical axis.

  let x_angle = rotation.y; // Uses the vertical mouse-drag angle to rotate the cube around its horizontal axis.

  let rotated_y = vec3<f32>( // Applies the fixed vertical-axis rotation to the selected cube point.
    point.x * cos(y_angle) + point.z * sin(y_angle), // Computes the rotated horizontal coordinate.

    point.y, // Preserves the vertical coordinate during the y-axis rotation.

    -point.x * sin(y_angle) + point.z * cos(y_angle), // Computes the rotated depth coordinate.
  ); // Finishes the y-axis rotation.

  let rotated = vec3<f32>( // Applies the second rotation around the horizontal axis.
    rotated_y.x, // Preserves the horizontal coordinate during the x-axis rotation.

    rotated_y.y * cos(x_angle) - rotated_y.z * sin(x_angle), // Computes the final vertical coordinate.

    rotated_y.y * sin(x_angle) + rotated_y.z * cos(x_angle), // Computes the final depth coordinate.
  ); // Finishes the second cube rotation.

  return vec4<f32>(rotated.xy * 0.7, 0.0, 1.0); // Places the rotated cube inside WebGPU clip space for display.
} // Finishes the cube vertex shader.

@fragment // Declares the following function as the fragment-shader entry point.

fn fs_main() -> @location(0) vec4<f32> { // Produces the visible color for every cube-line fragment.

  return vec4<f32>(0.45, 0.90, 1.0, 1.0); // Draws the development cube with a bright blue-white line color.
} // Finishes the cube fragment shader.