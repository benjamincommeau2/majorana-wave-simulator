pub fn create_mass_step_profile_1d( // Creates a one-dimensional piecewise-constant mass profile with one discrete boundary.

  point_count: usize,

  boundary_index: usize,

  left_mass: f32,

  right_mass: f32,

) -> Vec<f32> {

  assert!( // Allows the boundary to sit at the end of the lattice while rejecting indices beyond the available sites.

    boundary_index <= point_count,

    "Mass-step boundary index must not exceed the lattice point count.",

  );

  let mut mass_profile = Vec::with_capacity( // Allocates exactly one mass value for every lattice site.

    point_count,

  );

  for spatial_index in 0..point_count { // Visits each lattice site in ordinary spatial order.

    let mass = if spatial_index < boundary_index { // Assigns sites before the boundary to the left-hand mass region.

      left_mass

    } else { // Assigns the boundary site itself and every later site to the right-hand mass region.

      right_mass

    };

    mass_profile.push(

      mass,

    );

  }

  mass_profile // Returns the complete piecewise-constant spatial mass field.

}