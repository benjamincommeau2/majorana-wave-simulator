/*
This file is the Rust crate root for the Majorana Wave Simulator.

Its responsibility is to declare the major modules that make up the application.

Browser startup and orchestration live in `browser_startup.rs`.
*/

pub mod state; // Makes the CPU-side Majorana state module available through the crate's public API.

pub mod gpu; // Makes the WebGPU modules available through the crate's public API.

pub mod physics; // Makes the CPU-side physics modules available through the crate's public API.

pub mod spatial_majorana_field;

pub mod runtime_diagnostics;

pub mod webgpu_compatibility;

#[cfg(target_arch = "wasm32")]

mod browser_canvas_recorder;

#[cfg(target_arch = "wasm32")]

mod runtime_diagnostics_overlay;

mod browser_startup;

pub mod simulation_clock;

#[cfg(target_arch = "wasm32")]

mod mass_boundary_control;

