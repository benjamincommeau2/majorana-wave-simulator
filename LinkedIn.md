# 2026/08/29

https://www.linkedin.com/posts/benjamincommeau_rust-webgpu-webassembly-ugcPost-7500324935261868032-hA7W/?utm_source=share&utm_medium=member_desktop&rcm=ACoAACtxJGYB1ue63Kge-Z8YwDkr7dUOCr3VdCs

Majorana WebGPU Simulator: The Propagator Is Now Running Live on the GPU

Repo:
https://github.com/benjamincommeau2/majorana-wave-simulator

Live demo:
https://benjamincommeau2.github.io/majorana-wave-simulator/

My last update ended with a CPU reference implementation of the Majorana/Dirac propagator. That numerical path is now running on WebGPU and driving the browser visualization in real time.

The current path is:

4-real-component Majorana field
→ spatial mass profile m(x)
→ scaled Dirac generator K/a
→ rolling Bessel-Chebyshev recurrence
→ next Majorana field
→ WebGPU rendering

For the 1D physics currently implemented,

K = -αx ∂x + m(x)(-iβ),

with ∂tΨ = KΨ.

The GPU recurrence uses a constant number of field-sized working buffers rather than storing every Chebyshev basis state.

Connecting the verified 1D physics to the existing 16×16×16 visualization exposed an important architecture issue: flattening all 4096 points into one spectral line would incorrectly couple the end of one x-row to the start of the next.

Instead, the GPU treats the volume as 256 independent x-lines × 16 points and applies the same 1D spectral evolution to each fixed (y,z) line in parallel.

I added integration tests verifying that:

→ the full 16³ GPU result matches the CPU line-by-line oracle
→ neighboring x-lines do not couple
→ complete GPU timesteps match the CPU propagator
→ changing the mass profile preserves the already-evolved state

While testing the 16-point even grid, I also isolated a GPU NaN to the spectral derivative kernel. I replaced repeated shader-side trigonometric coefficient calculations with a derivative matrix precomputed once during setup. For the current grid that matrix is only 16×16, leaving the GPU hot path as coefficient loads and multiply-adds.

The browser now uses the same 4096-point GPU buffer for physics and rendering:

requestAnimationFrame
→ fixed-step SimulationClock
→ record N GPU physics steps
→ submit
→ render the evolved field

No CPU readback is required in the simulation loop.

The live demo now visibly evolves continuously while remaining interactively rotatable.

The current model is intentionally still 1D physics embedded in a 3D visualization: each (y,z) x-line evolves independently. I am not calling it a full 3D Dirac solver yet.

Next: runtime performance diagnostics, then an interactive movable mass boundary.

#Rust #WebGPU #WebAssembly #GPUComputing #ScientificComputing #ComputationalPhysics #NumericalMethods #OpenSource

# 2026/08/29

https://www.linkedin.com/posts/benjamincommeau_rust-webgpu-webassembly-activity-7499611838024818688-KE34/?utm_source=share&utm_medium=member_desktop&rcm=ACoAACtxJGYB1ue63Kge-Z8YwDkr7dUOCr3VdCs

Majorana WebGPU Simulator: From GPU Infrastructure to a Tested Propagator

Repo:
 https://lnkd.in/e-wDysjg

Live demo:
 https://lnkd.in/eJpqeYKB

My last update ended with several major physics pieces still missing. Since then, I’ve built the CPU reference path for much of the simulator’s numerical evolution before moving it to WebGPU.

The current path is:

real 4-component Majorana field
 → J-DFT
 → Fourier-pseudospectral derivative
 → real Dirac generator
 → spatial mass profile m(x)
 → real Bessel-Chebyshev propagation
 → next Majorana state

In 1D, the real generator is

K = -αx ∂x + m(x)(-iβ),

with ∂tΨ = KΨ.

I first implemented a direct O(N²) J-DFT as a deliberately slow CPU oracle. It now verifies forward/inverse reconstruction, positive and negative spectral derivatives, the Nyquist convention, the Majorana/Dirac algebra, and the relativistic identity

K²Ψ = -(k² + m²)Ψ

for uniform-mass Fourier modes.

For time evolution, I derived the real Chebyshev recurrence

Φ₀ = Ψ
 Φ₁ = (K/a)Ψ
 Φₙ₊₁ = 2(K/a)Φₙ + Φₙ₋₁

with Bessel coefficients precomputed for a fixed spectral scale, timestep, and truncation order.

The CPU propagator has been checked against the exact single-mode evolution

e^(KΔt)Ψ = cos(EΔt)Ψ + [sin(EΔt)/E]KΨ.

I also implemented the first version of the interaction model I want: movable piecewise mass boundaries.

Ψ₀
 → propagate with mass profile A
 → Ψ₁
 → move boundary
 → propagate the SAME Ψ₁ with mass profile B
 → Ψ₂

The wavefunction is not reinitialized when the Hamiltonian changes. Numerically, this is piecewise-static evolution:

Ψₙ₊₁ = e^(KₙΔt)Ψₙ.

I also derived a conservative 1D spectral scale

a = k_max,active + m_max,

so moving a boundary within the allowed mass range does not require recomputing the Bessel coefficients.

One correction from earlier posts: I fixed an ambiguous tensor-order convention and locked the project to

J = I ⊗ (iY),

so J(a,b,c,d) = (b,-a,d,-c).

That correction is now protected by explicit component and algebra tests.

Current verification:

66 portable Rust tests passing
 native WebGPU integration test passing
 Rust → WebAssembly / Trunk build passing

The new propagation code is still a CPU reference implementation, not yet GPU-accelerated. The next step is to choose Chebyshev order from the actual spectral scale, timestep, and f32 precision target, then use the CPU path as the oracle for the WebGPU implementation.

hashtag#Rust hashtag#WebGPU hashtag#WebAssembly hashtag#ScientificComputing hashtag#ComputationalPhysics hashtag#NumericalMethods hashtag#QuantumPhysics hashtag#TDD hashtag#OpenSource

# 2026/08/27

https://www.linkedin.com/posts/benjamincommeau_rust-webgpu-webassembly-share-7498797625903661058-PaZ7/?utm_source=share&utm_medium=member_desktop&rcm=ACoAACtxJGYB1ue63Kge-Z8YwDkr7dUOCr3VdCs

Majorana WebGPU Simulator: Runtime-Verified GPU Physics + Interactive 3D Rendering

Repo: https://github.com/benjamincommeau2/majorana-wave-simulator

Live demo: https://benjamincommeau2.github.io/majorana-wave-simulator/

Since my last update, I closed an important correctness checkpoint and added the first interactive 3D rendering path.

The GPU implementation of

J = iY ⊗ I

with

(a, b, c, d) → (c, d, -a, -b)

is now verified at runtime in the browser.

The full path is:

MajoranaState
→ upload to GPU
→ WGSL compute shader applies J
→ GPU readback
→ reconstruct Rust state
→ compare against CPU reference

The browser now reports:

GPU J operation verified against CPU reference.

That distinction matters: a successful build is not the same as observing that the GPU computation actually produces the expected result.

Current verification:

→ 6 Rust tests passed
→ Trunk/WebAssembly build passed
→ GPU J path verified in the browser

I also built the first rendering infrastructure:

Rust/WASM
→ wgpu
→ browser canvas
→ WebGPU surface
→ render pipeline
→ WGSL vertex + fragment shaders
→ interactive 3D wireframe cube

The cube can be rotated by clicking and dragging.

Importantly, the cube is only a rendering-development test. It is not being presented as a Majorana wavefunction or physics visualization before that functionality exists.

The actual simulator still needs the spatial grid, spectral derivative machinery, mass profile, generator application, propagation, numerical validation, and eventually a visualization backed by real simulation data.

This stage also reinforced why I am treating comprehensibility as an engineering requirement. Browser startup, GPU compute, input handling, rendering, and physics are being separated into clearly named modules rather than allowed to accumulate in one large file.

The development rule remains:

GREEN → small change → tests/build/runtime verification → GREEN

Next: refactor the new rendering code into smaller responsibilities, then continue toward the spatial Majorana simulation.

Long-term direction:

real 4-component Majorana state
→ J-based spectral formulation
→ SLAC/Fourier-pseudospectral derivatives
→ spatially varying mass barriers
→ norm-preserving propagation
→ CPU/GPU validation
→ physics-backed visualization

If your team works in Rust, GPU computing, WebAssembly, numerical methods, scientific software, simulation, or computational physics, I’d appreciate a referral or introduction to a suitable role.

AI-assisted drafting. I reviewed the technical content and wording before publishing.

#Rust #WebGPU #WebAssembly #GPUComputing #ScientificComputing #ComputationalPhysics #SoftwareEngineering #OpenSource


# 2026/08/26

https://www.linkedin.com/posts/benjamincommeau_github-benjamincommeau2majorana-wave-simulator-activity-7498416508033835008-abZz?utm_source=share&utm_medium=member_desktop&rcm=ACoAACtxJGYB1ue63Kge-Z8YwDkr7dUOCr3VdCs

Majorana WebGPU Simulator: First GPU Physics Operation + Refactoring for Comprehension

Repo: https://lnkd.in/e-wDysjg

Since my last update, the project has moved from proving the GPU memory path to implementing the first actual physics operation on the GPU.

The operation is the real Majorana structure

J = iY ⊗ I

acting on a four-component real state as

(a, b, c, d) → (c, d, -a, -b)

I first implemented a CPU reference version and added tests confirming both the expected component mapping and the important identity

J²ψ = -ψ

I then implemented the same transformation in WGSL and wired it into the existing Rust → WebAssembly → wgpu → WebGPU pipeline.

The current compute path is now:

MajoranaState
→ upload to GPU storage buffer
→ WGSL compute shader applies J
→ copy to readback buffer
→ reconstruct the Rust state
→ compare against the CPU reference

The automated suite is now:

→ 6 tests passed
→ WebAssembly/Trunk build passed

I also made a significant architectural pivot in how I structure the code.

The low-level GPU setup was becoming difficult to reason about as one large browser-startup file, so I started treating comprehensibility as an engineering requirement, not just a style preference.

Recent refactors include:

→ moving browser/WASM orchestration out of lib.rs into browser_startup.rs
→ reducing lib.rs to a small crate/module map
→ replacing vague mod.rs files with gpu.rs and physics.rs
→ renaming buffers.rs → state_buffers.rs
→ renaming context.rs → gpu_context.rs
→ renaming shaders.rs → shader_modules.rs
→ separating GPU commands, pipelines, bind groups, shader creation, buffers, and context by responsibility

Each refactor followed the same rule:

GREEN → refactor → GREEN

The larger lesson for me is that scientific software has two correctness problems: the mathematics has to be right, and the implementation has to remain understandable enough to verify.

Next I’ll continue reducing the remaining readback/orchestration complexity, verify the full GPU J path at runtime, and then build toward the spectral Majorana evolution.

If this kind of work is relevant to your team and you think my approach to Rust, GPU computing, numerical methods, scientific software, or simulation could be useful, I’d genuinely appreciate a referral or introduction to a suitable role.

AI-assisted drafting. I reviewed the technical content and wording before publishing.

#Rust #WebGPU #WebAssembly #GPUComputing #ScientificComputing #ComputationalPhysics #NumericalMethods #SoftwareEngineering #OpenSource

# 2026/08/25

https://www.linkedin.com/posts/benjamincommeau_rust-webgpu-webassembly-share-7498009118801141761-RHNY/?utm_source=share&utm_medium=member_desktop&rcm=ACoAACtxJGYB1ue63Kge-Z8YwDkr7dUOCr3VdCs

Majorana WebGPU Simulator: Verifying the GPU Data Path and Refactoring for Comprehension

Repo: https://lnkd.in/e-wDysjg

Today I reached another important checkpoint in my browser-based Majorana wave simulator.

The project is now running through a Rust → WebAssembly → wgpu → WebGPU stack, and I verified a complete CPU → GPU → CPU round trip for the simulator’s four-component real Majorana state.

The current data path is:

MajoranaState [1.0, 0.0, 0.0, 0.0]
 → upload into a 16-byte GPU buffer
 → copy into a GPU readback buffer
 → asynchronously map the result back to CPU-visible memory
 → reconstruct the Rust state from the returned bytes
 → verify that all four components match exactly

I also continued introducing a more formal test-driven development workflow around the Rust-side state logic.

The current automated test suite verifies:

→ the Majorana state has exactly four real components
→ the initial state is [1.0, 0.0, 0.0, 0.0]
→ the state occupies exactly 16 bytes
→ GPU-style readback bytes can reconstruct the expected Majorana state

Current result:

 → 4 tests passed, 0 failed
 → WebAssembly build passed
 → GPU round-trip verified in the browser

One of the biggest software-engineering lessons from this stage was about making the code easier to reason about.

WebGPU code contains a lot of low-level setup: adapter discovery, device and queue creation, buffer descriptors, command encoders, copies, mapping, asynchronous callbacks, and readback handling.

So I started refactoring the GPU stack by responsibility:
 → gpu/context.rs → adapter, device, and queue initialization
 → gpu/buffers.rs → GPU state and readback-buffer creation
 → lib.rs → higher-level application orchestration

After each extraction, I re-ran the Rust tests and WebAssembly build to make sure the refactor preserved behavior.

I’m learning that software architecture is not only about making code easier for machines to execute. It is also about making complex systems easier for humans to understand, verify, and maintain.

This engineering work sits underneath the physics architecture I’ve been developing for the simulator: a real four-component Majorana representation, spectral/SLAC spatial derivatives, and a proposed J-based spectral formulation using J=iγ5.

Next: continue reducing the remaining GPU boilerplate, then place the first WGSL compute operation between the verified upload and readback paths and compare GPU results against a CPU reference calculation.

The long-term goal remains the same: turn the theory into a publicly accessible, GPU-accelerated browser simulation while building the testing, numerical-validation, and software-engineering discipline needed to make the results trustworthy.

Ai assisted.

hashtag#Rust hashtag#WebGPU hashtag#WebAssembly hashtag#TDD hashtag#GPUComputing hashtag#ScientificComputing hashtag#NumericalMethods hashtag#QuantumPhysics hashtag#SoftwareEngineering

# 2026/08/24

https://lnkd.in/p/e8yR5QFb

Majorana WebGPU Simulator: Moving the Browser Stack From JavaScript to Rust

Since my last update, I’ve started migrating the host side of my Majorana WebGPU simulator from handwritten JavaScript to Rust compiled to WebAssembly.

Repository:

https://lnkd.in/e-wDysjg

The active stack is now:

Rust
↓
WebAssembly
↓
wgpu
↓
browser WebGPU
↓
GPU

This was my first time building a browser application in Rust, so I’ve been approaching the migration the same way I’m approaching the numerical physics: one small, verifiable checkpoint at a time.

So far I have:

• installed and configured the Rust/WebAssembly toolchain
• moved the browser entry point from main.js to src/lib.rs
• set up Cargo, Trunk, wasm-bindgen, web-sys, and wgpu
• successfully compiled Rust to WebAssembly and executed it in the browser
• accessed and updated the DOM directly from Rust/WASM
• created a wgpu::Instance
• asynchronously requested a WebGPU adapter from Rust
• verified adapter acquisition in the browser
• documented a reproducible installation and local-development workflow
• separated generated Cargo/Trunk output from source-controlled files

The current browser checkpoint is:

Rust/WASM
↓
wgpu::Instance
↓
request_adapter(...)
↓
compatible WebGPU adapter acquired

One thing I’m appreciating about Rust already is how much the language forces me to be explicit about concepts that JavaScript can make easy to gloss over: ownership, borrowing, asynchronous execution, result handling, and type boundaries.

For a numerical GPU project, that explicitness is useful rather than incidental.
The next checkpoint is deliberately small again:

wgpu::Adapter
↓
wgpu::Device + wgpu::Queue
↓
four-real-component Majorana state [f32; 4]
↓
16-byte GPU buffer
↓
CPU → GPU upload
↓
GPU → CPU readback verification

Only after that memory path is proven will I move on to the first WGSL compute operation and the Majorana/J-DFT mathematics.

I’m trying to make the repository useful as more than a finished demo. The README now documents the mathematical reasoning, Rust/WebAssembly setup, WebGPU architecture, build process, testing roadmap, and exact development checkpoint so someone else can reproduce the project rather than just look at the final result.

This project has become a useful intersection of several areas I’m actively developing: Rust, GPU programming, WebAssembly, numerical methods, computational physics, and test-driven scientific software.

If your team works in scientific computing, GPU/software infrastructure, simulation, Rust, or numerical engineering, I’d be glad to connect.

AI-assisted drafting was used for this post. I reviewed the technical content and wording before publishing.

hashtag#Rust hashtag#WebGPU hashtag#WebAssembly hashtag#ScientificComputing hashtag#ComputationalPhysics hashtag#GPUComputing hashtag#OpenSource

# 2026/08/23

https://www.linkedin.com/feed/update/urn:li:share:7497306113344839680/

WebGPU Wave Simulator: Why I’m Pivoting From Weyl to Majorana



I’ve decided to pivot the wave-simulator project I’ve been documenting from a two-component Weyl formulation to a Majorana formulation.



Old repo:

https://github.com/benjamincommeau2/weyl-webgpu



New repo:

https://github.com/benjamincommeau2/majorana-wave-simulator



Because working through its numerical implementation exposed the reason for the pivot.



My original plan was attractive for WebGPU: keep only a two-component complex spinor, use an FFT/SLAC-style pseudospectral derivative to avoid lattice fermion doubling, and use a Chebyshev/Jacobi-Anger expansion to fast-forward unitary evolution without Trotterization errors.



I discovered that the vector potential A create many complications, one of them being requiring an infinite value to create perfect reflections. Introducing a mass term allows reflections as long as the energy of the wave packet is less than the mass of the wave in a specific region of space, hence a spatial dependent mass.



The complication appears when a mass is introduced through the two-component Majorana equation. The mass term couples the spinor to its complex conjugate, so the equation is real-linear rather than complex-linear. In order to invoke charge conjugation, one must take arbitrary unitaries and decompose them into charge conjugation operations which becomes a hassel at an algorithm level.



A two-component complex spinor already contains four real numbers. In a Majorana representation, those same four numbers can be treated directly as a real four-component spinor. The time-evolution generator can then be represented as a real anti-Hermitian generator, giving norm-preserving orthogonal evolution on the real state space without adding physical degrees of freedom.



For the spatial transform I’m exploring the real structure



J = iγ⁵,   J² = -1,



where γ⁵ = i γ⁰ γ¹ γ² γ³ is the Dirac chirality matrix, J = i Y ⊗ I can be written as a real anti-symmetric rotation matrix, and with a Fourier kernel



exp(Jθ) = cos(θ) + J sin(θ).



I’m calling this the J-DFT for now. Computationally, it can still map onto cache-friendly i-DFT machinery by pairing the four real components into two complex storage channels. That is only an encoding of the real Majorana state, no charge-conjugate copy needs to be stored.



There is also a useful algebraic split: the kinetic α matrices commute with J, while the mass matrix β anticommutes with J.



That distinction is becoming the architecture of the new simulator.



The larger goal has not changed: build the wave/scattering foundation carefully enough that every numerical shortcut can be tested before I connect it to the larger D-CTC and Smith-chart-style experiments.



This pivot is exactly why I’m developing in small checkpoints: when the mathematics says the data representation should change, change it before optimizing the wrong model.



AI-assisted.



#WebGPU #ScientificComputing #ComputationalPhysics #OpenSource


# 2026/08/19

https://www.linkedin.com/feed/update/urn:li:share:7495863937259966464/

WebGPU Weyl Simulator: Moving From Numerical Design to a Reproducible GPU Project



I’ve made the repository for my WebGPU Weyl simulator public and started turning the numerical design from my last post into a testable implementation.



The repo is here:

https://lnkd.in/e-p2rRnC



The current target remains a two-component Weyl equation,



H(t) = σ · (-i∇ - A(x,t)),



with moving step-function vector potentials.



For the time dependence, I’m starting with a frozen-Hamiltonian approach. During each short interval, A(x,t) is treated as static, the state is propagated under that Hamiltonian, and the resulting wavefunction is handed to the next Hamiltonian after the step moves.



The planned propagation is still Chebyshev/Jacobi-Anger:



exp(-iHΔt)ψ



with repeated applications of H through the Chebyshev recurrence.



For the spatial derivative, I’m continuing with the Fourier-pseudospectral direction from my previous post:



Hψ = F⁻¹[(σ · k)Fψ] - σ · A(x)ψ.



I’m building the implementation in deliberately small checkpoints so I can test each numerical and GPU assumption before stacking more complexity on top.



So far I have:



• set up the public GitHub repository with SSH authentication and an MIT license

• documented the mathematical model, numerical roadmap, performance goals, and exact development handoff point in the README

• initialized WebGPU in Chrome

• confirmed an NVIDIA Lovelace adapter and GPUDevice

• inspected WebGPU storage-buffer limits

• represented one complex two-component Weyl spinor as four Float32 values

• allocated the first GPU storage buffer for that spinor



The next step is intentionally small: copy that 16-byte spinor from CPU memory into the GPU buffer, read it back, and verify the result before writing the first WGSL compute shader.



I’m also adding a test-driven development workflow as the project grows. I want CPU reference tests, GPU integration tests, and physics-level validation such as norm conservation, FFT reconstruction, and Chebyshev convergence.



Once those foundations are trustworthy, I’ll start building toward the FFT Hamiltonian application and then the moving-step propagation.



AI-assisted drafting was used for this post. I reviewed the technical content and wording before publishing.



#WebGPU #ScientificComputing #ComputationalPhysics #OpenSource

# 2026/08/17

https://www.linkedin.com/feed/update/urn:li:activity:7495189884303925249/

WebGPU Dirac/Weyl Simulator — When the Fast Discretization Changes the Physics



I’ve been researching an interactive 3D Dirac/Weyl-family wave simulator in WebGPU.



The larger goal is to combine three mostly unrelated topics in one simulation: 3D wave mechanics, Deutsch closed-timelike-curve (D-CTC) quantum computation, and Smith-chart-style reflection/transmission analysis. Right now I’m building the wave/scattering foundation.



The current two-component model is



H = σ · (p − A(x))



where A(x) acts as an effective vector potential.



For a Weyl state, spinor orientation is tied to kinetic momentum q = p − A. If A changes across an interface, the allowed transmitted momentum and spinor change. Matching can require a reflected component; in other regimes the transmitted normal momentum becomes imaginary and the mode is evanescent. Special matching can still produce Klein-like perfect transmission. [2]



Could A simply be removed by a position-dependent phase transformation? Only if it is pure gauge:



A = ∇χ.



A gauge transformation can add or subtract a gradient, but cannot change ∇ × A. An interface configuration with nonzero curl therefore cannot be transformed away.



Numerically, my first approach looked ideal for WebGPU: centered finite differences plus matrix-free Chebyshev propagation.



After scaling H → H̃ so its spectrum lies in [−1,1],

U(Δt) ≈ exp(−icΔt)[J₀(aΔt) + 2 Σₙ₌₁ᴷ (−i)ⁿ Jₙ(aΔt)Tₙ(H̃)].



The recurrence reduces evolution to repeated Hψ operations instead of constructing or diagonalizing a huge Hamiltonian. [1]



The problem appeared in the spatial derivative. Centered differences replace

k → sin(kΔx)/Δx,



which changes the Weyl dispersion and creates extra low-energy lattice modes: the fermion-doubling problem. [3]



For scattering, this matters because sharp interfaces contain high spatial frequencies that can couple into lattice modes absent from the continuum problem.



I’m now investigating Fourier-pseudospectral differentiation while keeping Chebyshev propagation:



Hψ = F⁻¹[(σ · k)Fψ] − σ · A(x)ψ.



FFT-based pseudospectral methods are established for time-dependent Dirac equations [4], but on a 256³ WebGPU grid this replaces a cheap local stencil with repeated 3D FFTs.



The question I’m testing now is whether improved scattering fidelity justifies the added GPU memory traffic.



Eventually, those reflection/transmission amplitudes are what I want to connect to the Smith-chart part of the larger experiment.



References:



[1] Tal-Ezer & Kosloff, JCP 81, 3967 (1984), DOI 10.1063/1.448136

[2] Erementchouk & Mazumder, PLA 381, 2866 (2017), DOI 10.1016/j.physleta.2017.06.055

[3] Kaplan & Sen, PRL 132, 141604 (2024), DOI 10.1103/PhysRevLett.132.141604

[4] Antoine & Lorin, JCP 395, 583 (2019), DOI 10.1016/j.jcp.2019.06.020

#ScientificComputing #WebGPU #ComputationalPhysics
