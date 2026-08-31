// src/gpu/spatial_majorana_field_buffer.rs

pub fn create_spatial_majorana_field_buffer( // Creates the GPU storage buffer that holds the renderable field and remains the authoritative GPU-resident physics state.

  device: &wgpu::Device, // Borrows the existing WebGPU device responsible for creating GPU resources.

  size: wgpu::BufferAddress, // Receives the total number of bytes required by the complete spatial Majorana field.

) -> wgpu::Buffer { // Returns the completed GPU storage buffer.

  device.create_buffer( // Asks WebGPU to allocate storage for the complete spatial field.

    &wgpu::BufferDescriptor { // Starts the spatial-field buffer description.

      label: Some("Spatial Majorana Field Buffer"), // Gives the GPU resource an explicit debugging name.

      size, // Allocates exactly enough bytes for every four-component point in the field.

      usage:

        wgpu::BufferUsages::STORAGE

        | wgpu::BufferUsages::COPY_DST

        | wgpu::BufferUsages::COPY_SRC, // Lets rendering/compute read the field, Rust upload its initial state, and the propagator copy the current state into its reusable Chebyshev basis buffers.

      mapped_at_creation: false, // Leaves the buffer unmapped because Queue::write_buffer performs initialization.

    },

  )

}