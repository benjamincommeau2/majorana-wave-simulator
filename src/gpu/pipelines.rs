pub fn create_apply_j_pipeline( // Defines a helper responsible for creating the compute pipeline that will eventually execute the J shader.

  device: &wgpu::Device, // Borrows the existing WebGPU device that creates GPU pipeline resources.

  shader: &wgpu::ShaderModule, // Borrows the already-created WGSL shader module that contains our J operation.

) -> wgpu::ComputePipeline { // Returns the completed compute pipeline to the caller.

  device.create_compute_pipeline( // Asks WebGPU to create a compute pipeline from the supplied shader module.

    &wgpu::ComputePipelineDescriptor { // Starts the configuration describing this compute pipeline.

      label: Some("Apply J Compute Pipeline"), // Gives the pipeline a readable debugging label.

      layout: None, // Lets wgpu infer the bind-group layout directly from the shader for this first minimal pipeline.

      module: shader, // Selects our existing Apply J shader module as the compute program.

      entry_point: Some("main"), // Selects the WGSL function named `main` as the compute-shader entry point.

      compilation_options: wgpu::PipelineCompilationOptions::default(), // Uses wgpu's default shader-compilation settings for this first correctness checkpoint.

      cache: None, // Creates the pipeline without introducing a pipeline cache at this stage.

    }, // Finishes the compute-pipeline descriptor.

  ) // Returns the compute pipeline created by the WebGPU device.

} // Closes the compute-pipeline helper.