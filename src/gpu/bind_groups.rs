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