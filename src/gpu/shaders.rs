pub fn create_apply_j_shader(device: &wgpu::Device) -> wgpu::ShaderModule { // Defines a helper that asks wgpu to create the shader module for the GPU J operation.

  device.create_shader_module( // Asks the existing WebGPU device to create a shader module from our WGSL source.

    wgpu::ShaderModuleDescriptor { // Starts the descriptor that tells wgpu how to create this shader module.

      label: Some("Apply J Shader"), // Gives the shader module a readable debugging label.

      source: wgpu::ShaderSource::Wgsl( // Tells wgpu that the shader source is written in WGSL.

        include_str!("apply_j.wgsl").into(), // Embeds the WGSL file into the Rust binary at compile time and converts it into the source type wgpu expects.

      ), // Finishes the WGSL shader-source value.

    }, // Finishes the shader-module descriptor.

  ) // Returns the completed wgpu shader-module handle to the caller.

} // Closes the shader-module creation helper.