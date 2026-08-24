/*
This module contains CPU-side Majorana state logic for the simulator.

Development in this module follows Test-Driven Development (TDD):
1. Write a test that describes the expected behavior.
2. Run the test and confirm that it fails.
3. Implement the smallest amount of production code needed.
4. Run the test again and confirm that it passes.
5. Refactor only after the test is passing.

Automated tests for this module live in the project's `tests/` directory and are run with `cargo test`.
*/

pub struct MajoranaState { // Defines a public Rust structure that will store the four real components of a Majorana state.

  components: [f32; 4], // Stores exactly four 32-bit floating-point values representing the real components of the Majorana state.

  } // Closes the `MajoranaState` structure definition.

impl MajoranaState { // Starts an implementation block where we define functions and methods that belong to `MajoranaState`.

  pub fn new() -> Self { // Defines a public constructor named `new` that will create and return a new `MajoranaState`.

    Self { components: [1.0, 0.0, 0.0, 0.0] } // Creates a new Majorana state with the initial four-component real spinor [1, 0, 0, 0].

    } // Closes the `new` constructor function.

  pub fn len(&self) -> usize { // Defines a public method that borrows this Majorana state and returns the number of stored components.

    self.components.len() // Returns the length of the internal four-element components array.

    } // Closes the `len` method.

    pub fn components(&self) -> &[f32; 4] { // Defines a public method that borrows the Majorana state and returns a read-only reference to its four stored components.

      &self.components // Returns a borrowed reference to the internal four-component array without copying or transferring ownership.

      } // Closes the `components` method.

  pub fn from_bytes(bytes: &[u8]) -> Self { // Defines a public constructor that rebuilds a Majorana state from raw CPU-readable GPU bytes.

    Self { components: bytemuck::pod_read_unaligned(bytes) } // Reads exactly sixteen bytes into four f32 components without requiring the byte slice itself to have f32 alignment.

  } // Closes the `from_bytes` constructor.

  // `from_bytes` is now ready to be checked by the integration test that was written before this implementation.

} // Closes the `impl MajoranaState` block that contains methods belonging to `MajoranaState`.

