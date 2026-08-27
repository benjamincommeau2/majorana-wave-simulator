// src/gpu/gpu_j_readback_verification.rs

use crate::physics; // Gives the verifier access to the independently tested CPU J reference operation.

use crate::state; // Gives the verifier access to the tested MajoranaState byte-reconstruction logic.

pub(crate) fn gpu_j_readback_matches_cpu_reference( // Checks whether GPU readback bytes contain the same J-transformed state as the CPU reference implementation.

  readback_bytes: &[u8], // Receives the raw bytes mapped back from the GPU readback buffer.

  uploaded_components: &[f32; 4], // Receives the original four real components that were uploaded before the GPU applied J.

) -> bool { // Returns true only when the GPU result exactly matches the CPU J reference.

  let reconstructed_state = state::MajoranaState::from_bytes(readback_bytes); // Reconstructs the GPU-returned bytes through the already-tested MajoranaState conversion.

  let expected_components = physics::j::apply_j(uploaded_components); // Computes the independently tested CPU J result for the original uploaded state.

  reconstructed_state.components() == &expected_components // Accepts the GPU result only when all four reconstructed components match the CPU reference exactly.

} // Finishes GPU J readback verification.

#[cfg(test)] // Includes the centralized GPU J readback-verification tests only while running cargo test.

#[path = "../../tests/unit/gpu_j_readback_verification_tests.rs"] // Keeps this module's test source physically centralized under the repository tests directory.

mod gpu_j_readback_verification_tests; // Attaches the centralized tests as a child module so they can verify this crate-internal function.