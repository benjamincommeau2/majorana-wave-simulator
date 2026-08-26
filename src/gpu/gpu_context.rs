/*
Purpose: initialize the low-level WebGPU context needed by the simulator.

This module keeps GPU-discovery boilerplate out of `lib.rs` so the main
application file can focus on the simulator's high-level execution flow.
*/

pub fn create_instance() -> wgpu::Instance { // Creates and returns wgpu's main entry point so later GPU setup steps can share the same instance.

  wgpu::Instance::default() // Creates the WebGPU instance that will later create the rendering surface and request the adapter.

} // Closes the WebGPU instance-creation helper.

pub async fn request_adapter( // Finds and returns a WebGPU adapter using an already-created instance.

  instance: &wgpu::Instance, // Borrows the shared WebGPU instance instead of creating a hidden instance inside this helper.

) -> wgpu::Adapter { // Returns the compatible WebGPU adapter after the asynchronous browser request succeeds.

  instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.expect("Could not find a compatible GPU adapter") // Preserves the current adapter-request behavior while allowing the caller to retain the instance for future surface creation.

} // Closes the `request_adapter` helper.

pub async fn request_device_and_queue(adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) { // Requests the logical WebGPU device and queue associated with an existing adapter.

  adapter.request_device(&wgpu::DeviceDescriptor::default()).await.expect("Could not create WebGPU device") // Returns the device and queue when WebGPU initialization succeeds.

} // Closes the `request_device_and_queue` helper.