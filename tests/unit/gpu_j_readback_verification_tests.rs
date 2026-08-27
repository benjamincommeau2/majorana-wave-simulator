// tests/unit/gpu_j_readback_verification_tests.rs

use super::gpu_j_readback_matches_cpu_reference; // Tests the GPU J readback verifier from inside its production module.

fn components_as_bytes(components: &[f32; 4]) -> Vec<u8> { // Converts a known four-component test result into the same byte representation used by GPU readback.

  bytemuck::cast_slice(components).to_vec() // Copies the sixteen component bytes into owned test data.

} // Finishes creating GPU-style readback bytes.

#[test] // Verifies that the correct GPU J result is accepted.
fn correct_gpu_j_readback_matches_cpu_reference() {

  let uploaded_components = [1.0, 2.0, 3.0, 4.0]; // Uses a state whose four distinct values make ordering and sign mistakes visible.

  let gpu_result = [3.0, 4.0, -1.0, -2.0]; // Represents the correct J-transformed GPU result for the chosen input.

  let readback_bytes = components_as_bytes(&gpu_result); // Converts the expected GPU components into simulated readback bytes.

  assert!( // Requires the verifier to accept the correct GPU result.

    gpu_j_readback_matches_cpu_reference( // Runs the same verification function that browser startup will use.

      &readback_bytes, // Supplies the simulated correct GPU readback bytes.

      &uploaded_components, // Supplies the original state that existed before the GPU applied J.

    ), // Finishes checking the simulated readback.

  ); // Finishes the successful-verification assertion.

}

#[test] // Verifies that an unchanged state is rejected because the GPU was supposed to apply J.

fn unchanged_gpu_readback_does_not_match_cpu_reference() {

  let uploaded_components = [1.0, 2.0, 3.0, 4.0]; // Uses the same distinct input components for a clear comparison.

  let readback_bytes = components_as_bytes(&uploaded_components); // Simulates a GPU path that incorrectly returned the original untransformed state.

  assert!( // Requires the verifier to reject the unchanged state.

    !gpu_j_readback_matches_cpu_reference( // Checks the incorrect simulated GPU result against the CPU reference.

      &readback_bytes, // Supplies bytes containing the original state instead of J applied to that state.

      &uploaded_components, // Supplies the original state used to calculate the CPU reference.

    ), // Finishes checking the unchanged readback.

  ); // Finishes the rejection assertion.

}

#[test] // Verifies that incorrect component ordering is rejected.

fn wrongly_ordered_gpu_j_readback_does_not_match_cpu_reference() {

  let uploaded_components = [1.0, 2.0, 3.0, 4.0]; // Uses distinct values so swapping components cannot accidentally produce the correct result.

  let wrongly_ordered_result = [4.0, 3.0, -1.0, -2.0]; // Simulates a GPU result whose first two J-output components are incorrectly reversed.

  let readback_bytes = components_as_bytes(&wrongly_ordered_result); // Converts the intentionally incorrect result into GPU-style bytes.

  assert!( // Requires the verifier to reject the ordering error.

    !gpu_j_readback_matches_cpu_reference( // Compares the incorrect simulated GPU result against the CPU reference.

      &readback_bytes, // Supplies the wrongly ordered readback bytes.

      &uploaded_components, // Supplies the original state used to calculate the correct CPU J result.

    ), // Finishes checking the wrongly ordered result.

  ); // Finishes the rejection assertion.

}

#[test] // Verifies that incorrect J signs are rejected.

fn wrongly_signed_gpu_j_readback_does_not_match_cpu_reference() {

  let uploaded_components = [1.0, 2.0, 3.0, 4.0]; // Uses nonzero values so incorrect signs remain observable.

  let wrongly_signed_result = [3.0, 4.0, 1.0, 2.0]; // Simulates a GPU result that omitted the required negative signs on the final two components.

  let readback_bytes = components_as_bytes(&wrongly_signed_result); // Converts the intentionally wrong values into GPU-style readback bytes.

  assert!( // Requires the verifier to reject the sign error.

    !gpu_j_readback_matches_cpu_reference( // Compares the incorrect simulated GPU result against the CPU reference.

      &readback_bytes, // Supplies the wrongly signed readback bytes.

      &uploaded_components, // Supplies the original state used for the CPU reference calculation.

    ), // Finishes checking the wrongly signed result.

  ); // Finishes the rejection assertion.

}