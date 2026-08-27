pub fn create_apply_j_bind_group( // Defines a helper that connects the existing Majorana GPU buffer to the J compute pipeline.

  device: &wgpu::Device, // Borrows the WebGPU device that creates bind-group resources.

  pipeline: &wgpu::ComputePipeline, // Borrows the J compute pipeline so we can obtain its inferred binding layout.

  state_buffer: &wgpu::Buffer, // Borrows the existing GPU buffer that stores the four-component Majorana state.

) -> wgpu::BindGroup { // Returns the completed bind group that connects the state buffer to the shader.

  let layout = pipeline.get_bind_group_layout(0); // Gets bind-group layout zero, which corresponds to `@group(0)` in our WGSL shader.

  device.create_bind_group( // Asks the WebGPU device to create the bind group using that layout.

    &wgpu::BindGroupDescriptor { // Starts the descriptor that defines which GPU resource occupies each shader binding.

      label: Some("Apply J Bind Group"), // Gives the bind group a readable debugging label.

      layout: &layout, // Uses the binding layout inferred from the J compute shader.

      entries: &[ // Starts the list of GPU resources supplied to the shader.

        wgpu::BindGroupEntry { // Starts the resource entry for WGSL binding zero.

          binding: 0, // Connects this resource to `@binding(0)` in `apply_j.wgsl`.

          resource: state_buffer.as_entire_binding(), // Exposes the complete existing Majorana state buffer to the shader.

        }, // Finishes the binding-zero resource entry.

      ], // Finishes the list of bind-group entries.

    }, // Finishes the bind-group descriptor.

  ) // Returns the completed WebGPU bind group.

} // Closes the bind-group creation helper.

pub fn create_spatial_majorana_field_render_bind_group( // Connects the rotation uniform and complete spatial Majorana field to the field rendering pipeline.

  device: &wgpu::Device, // Borrows the WebGPU device that creates bind-group resources.

  pipeline: &wgpu::RenderPipeline, // Borrows the spatial-field render pipeline so its inferred binding layout can be retrieved.

  rotation_buffer: &wgpu::Buffer, // Borrows the sixteen-byte uniform buffer containing yaw and pitch.

  spatial_majorana_field_buffer: &wgpu::Buffer, // Borrows the GPU storage buffer containing all 4096 four-component Majorana field points.

) -> wgpu::BindGroup { // Returns the completed bind group used by the spatial-field render shader.

  let layout = pipeline.get_bind_group_layout(0); // Retrieves the layout inferred from group zero of spatial_majorana_field.wgsl.

  device.create_bind_group( // Creates the bind group connecting Rust GPU resources to WGSL bindings.

    &wgpu::BindGroupDescriptor { // Starts the spatial-field bind-group description.

      label: Some("Spatial Majorana Field Render Bind Group"), // Gives the bind group a readable debugging name.

      layout: &layout, // Uses the layout inferred from the field rendering shader.

      entries: &[ // Starts the two resources exposed to the field shader.

        wgpu::BindGroupEntry { // Starts the rotation-uniform resource.

          binding: 0, // Matches @group(0) @binding(0) in spatial_majorana_field.wgsl.

          resource: rotation_buffer.as_entire_binding(), // Exposes the complete sixteen-byte rotation uniform.

        }, // Finishes binding zero.

        wgpu::BindGroupEntry { // Starts the complete spatial-field storage resource.

          binding: 1, // Matches @group(0) @binding(1) in spatial_majorana_field.wgsl.

          resource: spatial_majorana_field_buffer.as_entire_binding(), // Exposes all 4096 four-component field points to the vertex shader.

        }, // Finishes binding one.

      ], // Finishes the bind-group resource list.

    }, // Finishes the bind-group description.

  ) // Returns the completed spatial-field bind group.

} // Finishes creating the spatial-field render bind group.