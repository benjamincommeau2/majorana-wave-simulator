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
