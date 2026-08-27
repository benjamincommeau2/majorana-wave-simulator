// src/gpu/development_cube/render_spatial_majorana_field_frame.rs

pub(super) fn render_spatial_majorana_field_frame( // Renders one frame of the actual uploaded three-dimensional Majorana field.

  surface: &wgpu::Surface<'static>, // Borrows the configured browser surface that receives the rendered field.

  device: &wgpu::Device, // Borrows the WebGPU device used to create this frame's command encoder.

  queue: &wgpu::Queue, // Borrows the queue used to update rotation and submit rendering commands.

  pipeline: &wgpu::RenderPipeline, // Borrows the spatial Majorana field render pipeline.

  rotation_buffer: &wgpu::Buffer, // Borrows the uniform buffer containing the current yaw and pitch.

  field_bind_group: &wgpu::BindGroup, // Borrows the bind group containing both rotation and the uploaded spatial field.

  rotation_values: &[f32; 4], // Borrows the current mouse-controlled rotation values.

) { // Starts rendering one spatial Majorana field frame.

  queue.write_buffer( // Updates the GPU rotation uniform before rendering this frame.

    rotation_buffer, // Selects the shared rotation uniform buffer.

    0, // Starts writing at the beginning of the uniform.

    bytemuck::cast_slice(rotation_values), // Reinterprets the four f32 rotation values as sixteen bytes.

  ); // Finishes updating the rotation uniform.

  let surface_texture = match surface.get_current_texture() { // Acquires the browser surface texture for this frame.

    wgpu::CurrentSurfaceTexture::Success(texture) => texture, // Uses a normally acquired surface texture.

    wgpu::CurrentSurfaceTexture::Suboptimal(texture) => texture, // Also uses a valid texture when the surface reports a suboptimal configuration.

    surface_error => panic!("Could not acquire spatial Majorana field surface texture: {surface_error:?}"), // Stops clearly if no renderable texture can be acquired.

  }; // Finishes acquiring the surface texture.

  let surface_view = surface_texture.texture.create_view( // Creates the texture view used as the render-pass output.

    &wgpu::TextureViewDescriptor::default(), // Uses the complete browser surface texture.

  ); // Finishes creating the surface view.

  let mut encoder = device.create_command_encoder( // Creates the encoder that records this frame's GPU commands.

    &wgpu::CommandEncoderDescriptor { // Starts the command-encoder description.

      label: Some("Spatial Majorana Field Animation Encoder"), // Gives the encoder an explicit debugging label.

    }, // Finishes the encoder description.

  ); // Finishes creating the encoder.

  { // Starts a scope so the render pass ends before submission.

    let mut render_pass = encoder.begin_render_pass( // Begins recording the field-rendering commands.

      &wgpu::RenderPassDescriptor { // Starts the render-pass description.

        label: Some("Spatial Majorana Field Animation Pass"), // Gives the field render pass a readable debugging name.

        color_attachments: &[ // Starts the browser-surface color attachment list.

          Some(wgpu::RenderPassColorAttachment { // Uses the browser surface as the single color target.

            view: &surface_view, // Sends rendered particles into the current browser surface texture.

            depth_slice: None, // Uses no three-dimensional texture slice.

            resolve_target: None, // Uses no multisample resolve target.

            ops: wgpu::Operations { // Describes how this frame handles existing surface pixels.

              load: wgpu::LoadOp::Clear( // Clears the previous frame before drawing the field.

                wgpu::Color { // Uses the same dark background as the development renderer.

                  r: 0.08,

                  g: 0.12,

                  b: 0.20,

                  a: 1.0,

                }, // Finishes the background color.

              ), // Finishes the clear operation.

              store: wgpu::StoreOp::Store, // Keeps the finished image so the browser can present it.

            }, // Finishes the color operations.

          }), // Finishes the browser-surface color attachment.

        ], // Finishes the color-attachment list.

        depth_stencil_attachment: None, // Keeps depth buffering disabled for this first transparent field visualization.

        timestamp_writes: None, // Uses no GPU timing instrumentation yet.

        occlusion_query_set: None, // Uses no visibility-query system.

        multiview_mask: None, // Uses one ordinary browser view.

      }, // Finishes the render-pass description.

    ); // Finishes beginning the render pass.

    render_pass.set_pipeline(pipeline); // Selects the spatial Majorana field rendering pipeline.

    render_pass.set_bind_group( // Exposes rotation and the complete uploaded spatial field to WGSL.

      0, // Connects the resources to shader bind group zero.

      field_bind_group, // Supplies binding zero and binding one together.

      &[], // Uses no dynamic buffer offsets.

    ); // Finishes binding the field resources.

    render_pass.draw(0..24_576, 0..1); // Generates six vertices for each of the 4096 actual spatial Majorana field samples.

  } // Ends the render pass before submission.

  queue.submit( // Sends the completed rendering commands to WebGPU.

    Some(encoder.finish()), // Finishes the command encoder and submits its command buffer.

  ); // Finishes GPU submission.

  queue.present(surface_texture); // Presents the completed spatial-field frame in the browser canvas.

} // Finishes rendering one spatial Majorana field frame.