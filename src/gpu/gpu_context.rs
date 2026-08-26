/*
Purpose: initialize the low-level WebGPU context needed by the simulator.

This module keeps GPU-discovery boilerplate out of `lib.rs` so the main
application file can focus on the simulator's high-level execution flow.
*/

pub async fn request_adapter() -> wgpu::Adapter { // Finds and returns a WebGPU adapter that the simulator can use.

  let instance = wgpu::Instance::default(); // Creates wgpu's main entry point for discovering available GPU hardware.

  instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.expect("Could not find a compatible GPU adapter") // Requests a compatible adapter and returns it when successful.

} // Closes the `request_adapter` helper.

pub async fn request_device_and_queue(adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) { // Requests the logical WebGPU device and queue associated with an existing adapter.

  adapter.request_device(&wgpu::DeviceDescriptor::default()).await.expect("Could not create WebGPU device") // Returns the device and queue when WebGPU initialization succeeds.

} // Closes the `request_device_and_queue` helper.