// src/gpu/spatial_majorana_field.wgsl

@group(0) @binding(0) // Places the mouse-controlled field rotation at bind group zero and binding zero.

var<uniform> rotation: vec4<f32>; // Stores yaw in rotation.x and pitch in rotation.y using the same sixteen-byte layout as the development cube.

@group(0) @binding(1) // Places the complete uploaded spatial Majorana field at binding one.

var<storage, read> field_points: array<vec4<f32>>; // Lets the vertex shader read one four-component Majorana value for every spatial grid point.

const GRID_SIDE_LENGTH: u32 = 16u; // Matches the current sixteen-by-sixteen-by-sixteen CPU spatial field.

const VERTICES_PER_PARTICLE: u32 = 6u; // Uses two triangles, or six generated vertices, for each visible field sample.

const PARTICLE_CORNERS = array<vec2<f32>, 6>( // Defines the two triangles forming one small screen-facing square.

  vec2<f32>(-1.0, -1.0), // Defines the lower-left corner of the first triangle.

  vec2<f32>( 1.0, -1.0), // Defines the lower-right corner of the first triangle.

  vec2<f32>( 1.0,  1.0), // Defines the upper-right corner of the first triangle.

  vec2<f32>(-1.0, -1.0), // Reuses the lower-left corner for the second triangle.

  vec2<f32>( 1.0,  1.0), // Reuses the upper-right corner for the second triangle.

  vec2<f32>(-1.0,  1.0), // Defines the upper-left corner of the second triangle.

); // Finishes the generated particle-corner array.

struct VertexOutput { // Defines the information passed from the vertex shader into the fragment shader.

  @builtin(position)
  position: vec4<f32>, // Stores the final clip-space position of this generated particle vertex.

  @location(0)
  amplitude: f32, // Carries the actual Majorana component-zero amplitude into the fragment shader.

  @location(1)
  particle_coordinate: vec2<f32>, // Carries the local square coordinate so the fragment shader can turn the square into a round particle.

} // Finishes the vertex-stage output structure.

@vertex // Declares the following function as the spatial-field vertex shader.

fn vs_main( // Generates one vertex belonging to one spatial Majorana field sample.

  @builtin(vertex_index) vertex_index: u32, // Receives the automatically generated vertex number from the draw command.

) -> VertexOutput { // Returns both the particle position and its actual field amplitude.

  let point_index = vertex_index / VERTICES_PER_PARTICLE; // Determines which of the 4096 spatial field samples this vertex belongs to.

  let particle_corner_index = vertex_index % VERTICES_PER_PARTICLE; // Determines which of the six square vertices is being generated.

  let x_index = point_index % GRID_SIDE_LENGTH; // Recovers the x grid coordinate from the flattened field index.

  let y_index = (point_index / GRID_SIDE_LENGTH) % GRID_SIDE_LENGTH; // Recovers the y grid coordinate from the flattened field index.

  let z_index = point_index / (GRID_SIDE_LENGTH * GRID_SIDE_LENGTH); // Recovers the z grid coordinate from the flattened field index.

  let grid_position = vec3<f32>( // Converts the integer three-dimensional grid coordinate into floating-point shader coordinates.

    f32(x_index), // Converts the x grid coordinate to f32.

    f32(y_index), // Converts the y grid coordinate to f32.

    f32(z_index), // Converts the z grid coordinate to f32.

  ); // Finishes creating the floating-point grid position.

  let normalized_position = ( // Maps grid coordinates from zero-through-fifteen into approximately minus-one-through-plus-one space.

    grid_position / f32(GRID_SIDE_LENGTH - 1u) // Converts each grid axis into the zero-through-one range.

  ) * 2.0 - vec3<f32>(1.0, 1.0, 1.0); // Recenters all three axes around zero.

  let y_angle = rotation.x; // Uses horizontal mouse dragging as rotation around the vertical axis.

  let x_angle = rotation.y; // Uses vertical mouse dragging as rotation around the horizontal axis.

  let rotated_y = vec3<f32>( // Rotates the actual spatial grid point around the vertical axis.

    normalized_position.x * cos(y_angle) + normalized_position.z * sin(y_angle), // Computes the rotated horizontal coordinate.

    normalized_position.y, // Preserves the vertical coordinate during the y-axis rotation.

    -normalized_position.x * sin(y_angle) + normalized_position.z * cos(y_angle), // Computes the rotated depth coordinate.

  ); // Finishes the vertical-axis rotation.

  let rotated = vec3<f32>( // Applies the second rotation around the horizontal axis.

    rotated_y.x, // Preserves the horizontal coordinate during this rotation.

    rotated_y.y * cos(x_angle) - rotated_y.z * sin(x_angle), // Computes the final vertical coordinate.

    rotated_y.y * sin(x_angle) + rotated_y.z * cos(x_angle), // Computes the final depth coordinate.

  ); // Finishes rotating the three-dimensional field point.

  let amplitude = clamp( // Produces the visualization strength from the actual uploaded field value.

    abs(field_points[point_index].x), // Reads component zero of this real four-component Majorana field sample.

    0.0, // Prevents the visualization strength from becoming negative.

    1.0, // Caps this first visualization at full intensity.

  ); // Finishes deriving the visual amplitude.

  let particle_coordinate = PARTICLE_CORNERS[particle_corner_index]; // Selects this vertex's local corner within its tiny screen-facing particle.

  let particle_size = 0.006 + amplitude * 0.020; // Makes high-amplitude Gaussian samples visibly larger while keeping low-amplitude samples small.

  let screen_center = rotated.xy * 0.72; // Projects the rotated three-dimensional field into the current simple orthographic-style screen view.

  let screen_position = screen_center + particle_coordinate * particle_size; // Offsets this generated vertex from the field point center to form its tiny square.

  var output: VertexOutput; // Creates the value returned from this vertex invocation.

  output.position = vec4<f32>( // Places the generated particle vertex into WebGPU clip space.

    screen_position, // Uses the generated two-dimensional location after three-dimensional rotation.

    0.0, // Keeps depth buffering disabled for this first field visualization.

    1.0, // Uses the ordinary homogeneous clip-space coordinate.

  ); // Finishes the clip-space position.

  output.amplitude = amplitude; // Passes the actual Majorana field amplitude to the fragment shader.

  output.particle_coordinate = particle_coordinate; // Passes the local particle coordinate to the fragment shader.

  return output; // Returns this generated field-particle vertex.

} // Finishes the spatial-field vertex shader.

@fragment // Declares the following function as the spatial-field fragment shader.

fn fs_main( // Colors one fragment belonging to one rendered spatial field sample.

  input: VertexOutput, // Receives the actual field amplitude and local particle coordinate from the vertex shader.

) -> @location(0) vec4<f32> { // Returns the final color written into the browser surface.

  if length(input.particle_coordinate) > 1.0 { // Checks whether this fragment lies outside the circular center of the generated square.

    discard; // Removes square corners so every field sample appears as a round particle instead of a box.

  } // Finishes the circular-particle test.

  let brightness = input.amplitude; // Uses the actual Majorana component-zero amplitude as the visible brightness.

  return vec4<f32>( // Produces a blue-white field color whose intensity and transparency come from the Gaussian amplitude.

    0.30 + brightness * 0.55, // Increases the red channel toward white as the field amplitude rises.

    0.55 + brightness * 0.40, // Keeps the visible field strongly blue-white.

    1.0, // Keeps the blue channel at full strength.

    brightness * 0.55, // Makes weak Gaussian samples transparent and strong samples more visible.

  ); // Finishes producing this field fragment's color.

} // Finishes the spatial-field fragment shader.