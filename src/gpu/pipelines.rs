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

pub fn create_development_cube_pipeline( // Creates the render pipeline used for the temporary wireframe cube.

  device: &wgpu::Device, // Borrows the WebGPU device responsible for creating the pipeline.

  shader: &wgpu::ShaderModule, // Borrows the already-compiled cube rendering shader.

  surface_format: wgpu::TextureFormat, // Receives the pixel format selected for the browser rendering surface.

) -> wgpu::RenderPipeline { // Returns the completed wireframe-cube render pipeline.

  device.create_render_pipeline( // Asks WebGPU to create the rendering pipeline.

    &wgpu::RenderPipelineDescriptor { // Starts the render-pipeline configuration.

      label: Some("Development Cube Render Pipeline"), // Gives the pipeline a readable debugging name.

      layout: None, // Lets wgpu infer the empty pipeline layout because this first cube uses no bound resources.

      vertex: wgpu::VertexState { // Configures the vertex stage.

        module: shader, // Uses the development-cube WGSL module.

        entry_point: Some("vs_main"), // Selects the cube vertex function.

        buffers: &[], // Uses no vertex buffers because the shader currently contains the cube geometry itself.

        compilation_options: wgpu::PipelineCompilationOptions::default(), // Uses default vertex-shader compilation settings.

      }, // Finishes the vertex-stage configuration.

      fragment: Some(wgpu::FragmentState { // Configures the fragment stage that colors the cube lines.

        module: shader, // Uses the same WGSL module for fragment processing.

        entry_point: Some("fs_main"), // Selects the cube fragment function.

        targets: &[Some(wgpu::ColorTargetState { // Defines the browser surface as the pipeline's color output.

          format: surface_format, // Matches the pipeline output format to the configured WebGPU surface.

          blend: Some(wgpu::BlendState::REPLACE), // Replaces destination pixels with the cube-line color.

          write_mask: wgpu::ColorWrites::ALL, // Allows the fragment shader to write all color channels.

        })], // Finishes the pipeline's single color target.

        compilation_options: wgpu::PipelineCompilationOptions::default(), // Uses default fragment-shader compilation settings.

      }), // Finishes the fragment-stage configuration.

      primitive: wgpu::PrimitiveState { // Describes how WebGPU interprets the generated vertices.

        topology: wgpu::PrimitiveTopology::LineList, // Treats every pair of vertices as one independent cube edge.

        ..Default::default() // Keeps the remaining primitive settings at their standard values.
      }, // Finishes the primitive configuration.

      depth_stencil: None, // Uses no depth buffer because this first milestone is a wireframe cube.

      multisample: wgpu::MultisampleState::default(), // Uses ordinary single-sample rendering.

      multiview_mask: None, // Uses one normal browser view rather than multiview rendering.

      cache: None, // Creates the pipeline without a pipeline cache.

    }, // Finishes the render-pipeline descriptor.

  ) // Returns the completed development-cube pipeline.

} // Finishes the development-cube pipeline helper.