/*
This file is the Rust crate root for the Majorana Wave Simulator.

Its responsibility is to declare the major modules that make up the application.

Browser startup and orchestration live in `browser_startup.rs`.
*/

pub mod state; // Makes the CPU-side Majorana state module available through the crate's public API.

pub mod gpu; // Makes the WebGPU modules available through the crate's public API.

pub mod physics; // Makes the CPU-side physics modules available through the crate's public API.

pub mod spatial_majorana_field; // Makes the three-dimensional Majorana field representation available to the simulator and its tests.

mod browser_startup; // Includes the browser and WebAssembly startup module in the compiled application.

pub mod simulation_clock; // Exposes the pure simulation clock used to schedule fixed physics steps independently from rendering.