pub mod context; // Makes `src/gpu/context.rs` available as the `gpu::context` module.

pub mod buffers; // Makes the dedicated GPU buffer module available as `gpu::buffers`.

pub mod shaders; // Makes the dedicated shader-module helper available as `gpu::shaders`.

pub mod pipelines; // Makes the dedicated compute-pipeline module available as `gpu::pipelines`.

pub mod bind_groups; // Makes the dedicated GPU bind-group helper available as `gpu::bind_groups`.

pub mod commands; // Makes the GPU command-recording module available as `gpu::commands`.