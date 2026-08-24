/*
This file contains integration tests for the CPU-side Majorana state module.

Its responsibilities include:
- Testing the public behavior exposed by `src/state.rs`.
- Verifying expected Majorana state values and dimensions.
- Checking edge cases and numerical behavior as the simulator grows.
- Catching regressions when production code changes.
- Supporting a Test-Driven Development (TDD) workflow.

For new testable behavior, the intended TDD cycle is:
1. Write a test that describes the expected behavior.
2. Run `cargo test` and confirm that the new test fails for the expected reason.
3. Implement the smallest production-code change needed to make it pass.
4. Run `cargo test` again and confirm that the test passes.
5. Refactor while keeping the full test suite passing.

These tests exercise the simulator through its public Rust API rather than
directly accessing private implementation details.
*/

use majorana_wave_simulator::state; // Imports the public state module from our simulator library so this integration test file can call its production code.

#[test] // Tells Rust that the next function is an automated test that should run when we execute `cargo test`.

fn majorana_state_has_four_components() { // Defines a test function whose name states the behavior we expect from a Majorana state.

  let majorana_state = state::MajoranaState::new(); // Asks the production state module to create a new Majorana state that this test can inspect.

  assert_eq!(majorana_state.len(), 4); // Passes only if the new Majorana state reports exactly four components.

  } // Closes the `majorana_state_has_four_components` test function.

#[test] // Tells Rust that the next function will be our second automated test case.

fn new_majorana_state_has_expected_components() { // Defines a test for the exact four-component values of a newly created Majorana state.

  let majorana_state = state::MajoranaState::new(); // Creates a fresh Majorana state so this test can verify its initial component values.

  assert_eq!(majorana_state.components(), &[1.0, 0.0, 0.0, 0.0]); // Passes only if the new Majorana state exposes exactly the expected four initial component values.

  } // Closes the `new_majorana_state_has_expected_components` test function.