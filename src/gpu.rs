pub mod gpu_context; // Makes `src/gpu/gpu_context.rs` available as the clearly named `gpu::gpu_context` module.

pub mod gpu_j_readback_verification; // Keeps the pure GPU J result-verification logic available wherever browser startup is compiled and allows centralized tests to exercise it.

pub mod start_gpu_j_runtime_verification; // Keeps the complete GPU J dispatch/readback runtime procedure out of browser startup orchestration.

pub mod state_buffers; // Makes the dedicated Majorana state buffer module available as `gpu::state_buffers`.

pub mod spatial_majorana_field_buffer; // Exposes the explicitly named GPU buffer helper for the complete three-dimensional Majorana field.

pub mod shader_modules; // Makes the dedicated shader-module helper available as `gpu::shader_modules`.

pub mod pipelines; // Makes the dedicated compute-pipeline module available as `gpu::pipelines`.

pub mod bind_groups; // Makes the dedicated GPU bind-group helper available as `gpu::bind_groups`.

pub mod commands; // Makes the GPU command-recording module available as `gpu::commands`.

pub mod development_cube; // Makes the dedicated development-cube animation module available as `gpu::development_cube`.

pub mod chebyshev_recurrence; // Provides the reusable constant-memory GPU Chebyshev basis recurrence.