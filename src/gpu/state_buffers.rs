/*
Purpose: create the GPU buffers used to store and read back the Majorana state.

The state buffer is used by the GPU simulation.
The readback buffer is a temporary staging buffer that lets the CPU read GPU data.
*/

pub fn create_majorana_buffers( // Creates both GPU buffers needed for one Majorana state.

  device: &wgpu::Device, // Borrows the WebGPU device that owns and creates GPU resources.

  size: wgpu::BufferAddress // Receives the required buffer size in bytes.

) -> (wgpu::Buffer, wgpu::Buffer) { // Returns the state buffer first and the readback buffer second.

  let state_buffer = device.create_buffer( // Creates the GPU-working buffer that stores the Majorana state.

    &wgpu::BufferDescriptor {

      label: Some("Majorana State Buffer"),

      size,

      usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,

      mapped_at_creation: false

  });

  let readback_buffer = device.create_buffer( // Creates the CPU-readable staging buffer used for GPU readback.

    &wgpu::BufferDescriptor {

      label: Some("Majorana Readback Buffer"),

      size,

      usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,

      mapped_at_creation: false

  });

  (state_buffer, readback_buffer) // Returns both completed buffers to the caller.

} // Closes the buffer-creation helper.