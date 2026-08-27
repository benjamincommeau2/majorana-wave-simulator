// src/gpu/development_cube/render_development_cube_frame.rs

pub(super) fn render_development_cube_frame( // Renders one development-cube frame using the current mouse-controlled orientation.

  surface: &wgpu::Surface<'static>, // Borrows the configured browser surface that receives the rendered frame.

  device: &wgpu::Device, // Borrows the WebGPU device used to create this frame's command encoder.

  queue: &wgpu::Queue, // Borrows the queue used to update the rotation uniform and submit rendering commands.

  pipeline: &wgpu::RenderPipeline, // Borrows the existing development-cube render pipeline.

  rotation_buffer: &wgpu::Buffer, // Borrows the uniform buffer that stores the current yaw and pitch values.

  rotation_bind_group: &wgpu::BindGroup, // Borrows the bind group that exposes the rotation uniform to the cube shader.

  rotation_values: &[f32; 4], // Borrows the four-f32 uniform value containing the current cube orientation.

) { // Starts rendering one development-cube frame.

  queue.write_buffer( // Updates the GPU rotation uniform before drawing this frame.

    rotation_buffer, // Selects the development-cube rotation uniform buffer.

    0, // Starts writing at the beginning of the uniform buffer.

    bytemuck::cast_slice(rotation_values), // Reinterprets the four f32 values as the sixteen bytes required by the GPU buffer.

  ); // Finishes updating the rotation uniform.

  let surface_texture = match surface.get_current_texture() { // Acquires the browser surface image that this frame will render into.

    wgpu::CurrentSurfaceTexture::Success(texture) => texture, // Uses the frame when WebGPU reports normal successful acquisition.

    wgpu::CurrentSurfaceTexture::Suboptimal(texture) => texture, // Still uses the valid frame when WebGPU reports that reconfiguration may eventually be useful.

    surface_error => panic!("Could not acquire cube animation surface texture: {surface_error:?}"), // Stops clearly if a renderable surface texture cannot be acquired.

  }; // Finishes acquiring this frame's surface texture.

  let surface_view = surface_texture.texture.create_view( // Creates the texture view used as the render pass color output.

    &wgpu::TextureViewDescriptor::default(), // Uses the complete surface texture with its default view configuration.

  ); // Finishes creating the surface texture view.

  let mut encoder = device.create_command_encoder( // Creates the command encoder used to record this frame's GPU commands.

    &wgpu::CommandEncoderDescriptor { // Starts the per-frame command-encoder description.

      label: Some("Development Cube Animation Encoder"), // Gives the command encoder a readable debugging label.

    }, // Finishes the command-encoder description.

  ); // Finishes creating the command encoder.

  { // Starts a short scope so the render pass ends before command submission.

    let mut render_pass = encoder.begin_render_pass( // Begins recording the commands that draw the development cube.

      &wgpu::RenderPassDescriptor { // Starts the render-pass description.

        label: Some("Development Cube Animation Pass"), // Gives the render pass a readable debugging label.

        color_attachments: &[ // Starts the list of color targets used by this frame.

          Some(wgpu::RenderPassColorAttachment { // Uses the browser surface as the single color output.

            view: &surface_view, // Directs rendered pixels into the current browser surface texture.

            depth_slice: None, // Uses no three-dimensional texture slice because the browser surface is two-dimensional.

            resolve_target: None, // Uses no multisampling resolve texture for the current wireframe preview.

            ops: wgpu::Operations { // Describes how the surface pixels are handled for this frame.

              load: wgpu::LoadOp::Clear( // Clears the entire frame before drawing the cube.

                wgpu::Color { // Uses the existing dark development background color.

                  r: 0.08, // Sets the red background component.

                  g: 0.12, // Sets the green background component.

                  b: 0.20, // Sets the blue background component.

                  a: 1.0, // Keeps the background fully opaque.

                }, // Finishes the background color.

              ), // Finishes the clear operation.

              store: wgpu::StoreOp::Store, // Preserves the completed frame so it can be presented.

            }, // Finishes the color operations.

          }), // Finishes the browser surface color attachment.

        ], // Finishes the color-target list.

        depth_stencil_attachment: None, // Keeps depth buffering disabled for the current wireframe development cube.

        timestamp_writes: None, // Keeps GPU timing instrumentation disabled while establishing renderer correctness.

        occlusion_query_set: None, // Uses no visibility-query system for this simple development renderer.

        multiview_mask: None, // Uses one ordinary browser view.

      }, // Finishes the render-pass description.

    ); // Finishes beginning the render pass.

    render_pass.set_pipeline(pipeline); // Selects the development-cube rendering pipeline.

    render_pass.set_bind_group( // Supplies the current rotation uniform required by the cube shader.

      0, // Connects the resource to shader bind group zero.

      rotation_bind_group, // Supplies the bind group containing the rotation uniform buffer.

      &[], // Uses no dynamic buffer offsets.

    ); // Finishes binding the rotation data.

    render_pass.draw(0..24, 0..1); // Draws the twelve wireframe cube edges.

  } // Ends the render pass before command submission.

  queue.submit( // Sends the completed frame commands to the GPU.

    Some(encoder.finish()), // Finishes the command encoder and supplies its command buffer.

  ); // Finishes submitting this frame.

  queue.present(surface_texture); // Presents the completed frame inside the browser canvas.

} // Finishes rendering one development-cube frame.