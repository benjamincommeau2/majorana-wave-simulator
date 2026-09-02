/*
Purpose: initialize the low-level WebGPU context needed by the simulator.

This module keeps GPU-discovery boilerplate out of browser startup.

The fallible helpers let browser startup handle unsupported environments
without panicking.

The convenience helpers preserve the existing behavior used by native
GPU integration tests.
*/


pub fn create_instance() -> wgpu::Instance {

  wgpu::Instance::default()

}


pub async fn try_request_adapter(

  instance: &wgpu::Instance,

) -> Result<

  wgpu::Adapter,

  wgpu::RequestAdapterError,

> {

  instance

    .request_adapter(

      &wgpu::RequestAdapterOptions::default(),

    )

    .await

}


pub async fn request_adapter(

  instance: &wgpu::Instance,

) -> wgpu::Adapter {

  try_request_adapter(

    instance,

  )

  .await

  .expect(

    "Could not find a compatible GPU adapter",

  )

}


pub async fn try_request_device_and_queue(

  adapter: &wgpu::Adapter,

) -> Result<

  (

    wgpu::Device,

    wgpu::Queue,

  ),

  wgpu::RequestDeviceError,

> {

  adapter

    .request_device(

      &wgpu::DeviceDescriptor::default(),

    )

    .await

}


pub async fn request_device_and_queue(

  adapter: &wgpu::Adapter,

) -> (

  wgpu::Device,

  wgpu::Queue,

) {

  try_request_device_and_queue(

    adapter,

  )

  .await

  .expect(

    "Could not create WebGPU device",

  )

}