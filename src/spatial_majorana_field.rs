// src/spatial_majorana_field.rs

pub struct SpatialMajoranaField { // Stores one four-real-component Majorana state at every point of a cubic spatial grid.

  side_length: usize, // Stores the number of spatial grid points along each x, y, and z axis.

  points: Vec<[f32; 4]>, // Stores the four real Majorana components for every spatial grid point.

} // Finishes the spatial Majorana field structure.

impl SpatialMajoranaField { // Defines construction and read access for the three-dimensional Majorana field.

  pub fn new_centered_gaussian( // Creates a cubic spatial field containing a Gaussian-localized initial Majorana state.

    side_length: usize, // Receives the number of grid points along each spatial axis.

    sigma: f32, // Receives the Gaussian width measured in grid-point units.

  ) -> Self { // Returns the completed three-dimensional Majorana field.

    let center = (side_length as f32 - 1.0) / 2.0; // Places the Gaussian center halfway across the grid, including between the two middle points of an even-sized grid.

    let mut points = Vec::with_capacity( // Allocates enough storage for every spatial point before filling the field.

      side_length * side_length * side_length, // Calculates the total number of points in the cubic grid.

    ); // Finishes allocating field storage.

    for z in 0..side_length { // Visits every grid position along the z axis.

      for y in 0..side_length { // Visits every grid position along the y axis.

        for x in 0..side_length { // Visits every grid position along the x axis.

          let delta_x = x as f32 - center; // Measures the horizontal distance from the Gaussian center.

          let delta_y = y as f32 - center; // Measures the vertical distance from the Gaussian center.

          let delta_z = z as f32 - center; // Measures the depth distance from the Gaussian center.

          let radius_squared = // Calculates the squared three-dimensional distance from the packet center.

            delta_x * delta_x // Adds the squared x displacement.

            + delta_y * delta_y // Adds the squared y displacement.

            + delta_z * delta_z; // Adds the squared z displacement.

          let amplitude = ( // Calculates the scalar Gaussian envelope at this spatial point.

            -radius_squared / (2.0 * sigma * sigma) // Forms the standard centered Gaussian exponent.

          ).exp(); // Converts the exponent into a positive Gaussian amplitude.

          points.push( // Adds one four-component Majorana state to this spatial grid location.

            [ // Starts the four-real-component state stored at this point.

              amplitude, // Places the Gaussian envelope in the first Majorana component.

              0.0, // Starts the second Majorana component empty.

              0.0, // Starts the third Majorana component empty.

              0.0, // Starts the fourth Majorana component empty.

            ], // Finishes this grid point's Majorana state.

          ); // Finishes adding the grid point to field storage.

        } // Finishes visiting the x axis.

      } // Finishes visiting the y axis.

    } // Finishes visiting the z axis.

    Self { // Starts constructing the completed spatial field.

      side_length, // Stores the cubic grid dimension for later three-dimensional indexing.

      points, // Stores all generated four-component Majorana states.

    } // Finishes constructing the spatial field.

  } // Finishes creating the centered Gaussian field.

  pub fn len(&self) -> usize { // Returns the total number of spatial grid points.

    self.points.len() // Reports how many four-component Majorana states are stored.

  } // Finishes returning the field size.

  pub fn components_at( // Returns the four real Majorana components stored at one three-dimensional grid coordinate.

    &self, // Borrows the spatial field without changing it.

    x: usize, // Receives the requested x coordinate.

    y: usize, // Receives the requested y coordinate.

    z: usize, // Receives the requested z coordinate.

  ) -> &[f32; 4] { // Returns a read-only reference to the four components at the requested spatial point.

    assert!(x < self.side_length); // Prevents an invalid x coordinate from silently addressing another grid point.

    assert!(y < self.side_length); // Prevents an invalid y coordinate from silently addressing another grid point.

    assert!(z < self.side_length); // Prevents an invalid z coordinate from reading outside the cubic field.

    let index = x // Starts converting the three-dimensional coordinate into one contiguous storage index.

      + self.side_length * ( // Moves through complete x rows for each y coordinate.

        y // Adds the requested y row.

        + self.side_length * z // Moves through complete x-y planes for the requested z coordinate.

      ); // Finishes calculating the one-dimensional storage index.

    &self.points[index] // Returns the four-component state stored at the requested spatial location.

  } // Finishes reading one spatial grid point.

} // Finishes the spatial Majorana field implementation.