# Majorana WebGPU Wave Simulator

A browser-based, GPU-accelerated numerical wave simulator written in Rust, compiled to WebAssembly, and built around a real four-component Majorana spinor, WebGPU, spectral/SLAC spatial derivatives, and a proposed J-DFT based on a real complex structure.

The browser application is implemented in Rust and compiled to WebAssembly.

The current software stack is:

```text
Rust
    ↓
WebAssembly
    ↓
wgpu
    ↓
browser WebGPU
    ↓
GPU
```

The Rust migration replaces the handwritten JavaScript host-side WebGPU code; it does not replace WebGPU or WGSL.

```math
J=i\gamma^5,
\qquad
J^2=-I.
```

The active repository is:

- **Majorana simulator:** https://github.com/benjamincommeau2/majorana-wave-simulator

The previous Weyl prototype is retained as design history and reference material:

- **Weyl prototype:** https://github.com/benjamincommeau2/weyl-webgpu

The project is intentionally developed in small, testable checkpoints so that each mathematical, numerical, memory-layout, and WebGPU decision can be understood before it is optimized.

---

# Installation and Local Development

This project runs in the browser using Rust compiled to WebAssembly, with `wgpu` providing the Rust-side interface to browser WebGPU.

The active development stack is:

```text
Rust source code
      ↓
Cargo
      ↓
wasm32-unknown-unknown
      ↓
Trunk
      ↓
WebAssembly
      ↓
wgpu
      ↓
browser WebGPU
      ↓
GPU
```

## 1. Install Git

Git is used to download the repository and manage source-code history.

Official download:

https://git-scm.com/downloads

After installation, open PowerShell and verify:

```powershell
git --version
```

A successful installation should print a Git version number.

---

## 2. Install Rust

Install Rust through the official `rustup` installer:

https://www.rust-lang.org/tools/install

On Windows, download and run `rustup-init.exe`.

When prompted, the default Rust installation is appropriate for this project.

After installation, completely restart the terminal or restart Visual Studio Code so the updated system `PATH` is detected.

Verify Rust:

```powershell
rustc --version
```

Verify Cargo:

```powershell
cargo --version
```

Both commands should print installed version numbers.

### Windows Build Tools

Rust on Windows may require the Microsoft Visual C++ Build Tools and Windows SDK.

If the Rust installer requests them, install the Visual Studio Build Tools from:

https://visualstudio.microsoft.com/visual-cpp-build-tools/

The important components are:

```text
MSVC C++ x64/x86 build tools
Windows SDK
```

The Visual Studio workload commonly associated with these components is:

```text
Desktop development with C++
```

---

## 3. Install Visual Studio Code

Visual Studio Code is the editor currently used for development.

Official download:

https://code.visualstudio.com/

VS Code is not required to compile the project, but the development instructions in this repository assume it is being used.

---

## 4. Install a WebGPU-Capable Browser

Use a recent Chromium-based browser with WebGPU support.

Recommended options include:

* Google Chrome: https://www.google.com/chrome/
* Microsoft Edge: https://www.microsoft.com/edge

The simulator depends on browser WebGPU support.

---

## 5. Clone the Repository

Open PowerShell in the directory where you want the project to be stored.

For example:

```powershell
cd C:\Users\YOUR_USERNAME\Documents
```

Clone the repository:

```powershell
git clone https://github.com/benjamincommeau2/majorana-wave-simulator.git
```

Enter the project directory:

```powershell
cd majorana-wave-simulator
```

The repository root should contain files similar to:

```text
majorana-wave-simulator/
├── src/
│   └── lib.rs
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── index.html
├── LICENSE
├── LinkedIn.md
└── README.md
```

---

## 6. Open the Project in Visual Studio Code

From the repository root, run:

```powershell
code .
```

The period means:

```text
open the current directory
```

If the `code` command is unavailable, open Visual Studio Code manually and use:

```text
File
  ↓
Open Folder
  ↓
majorana-wave-simulator
```

---

## 7. Understand the Source Directory

The main Rust source code currently lives in:

```text
src/lib.rs
```

The important project files are:

```text
Cargo.toml
```

Defines the Rust package, build type, and dependencies.

```text
Cargo.lock
```

Records the exact dependency versions selected by Cargo.

```text
src/lib.rs
```

Contains the Rust application code compiled into WebAssembly.

```text
index.html
```

Contains the browser page loaded by Trunk.

```text
.gitignore
```

Prevents generated build output from being committed to Git.

The `src` directory contains source code, but build commands should normally be run from the **repository root**, not from inside `src`.

For example, this is correct:

```text
majorana-wave-simulator/
    ↑
run trunk here
```

This is not the intended working directory:

```text
majorana-wave-simulator/src/
                           ↑
                    do not run Trunk here
```

If you enter the source directory while inspecting files:

```powershell
cd src
```

you can return to the repository root with:

```powershell
cd ..
```

---

## 8. Install the Rust WebAssembly Target

From the repository root, install Rust's browser WebAssembly compilation target:

```powershell
rustup target add wasm32-unknown-unknown
```

If it is already installed, `rustup` will report that the component is up to date.

This target allows Rust source code to be compiled into `.wasm` files that can execute in the browser.

---

## 9. Install Trunk

Trunk builds the Rust application into WebAssembly and serves the resulting browser application.

Project website:

https://trunkrs.dev/

GitHub repository:

https://github.com/trunk-rs/trunk

Install the version currently used by this project:

```powershell
cargo install trunk --locked --version 0.21.14
```

The first installation may take several minutes because Cargo compiles Trunk and its dependencies.

Verify the installation:

```powershell
trunk --version
```

The current project setup expects:

```text
trunk 0.21.14
```

---

## 10. Build the Project

From the repository root:

```powershell
trunk build
```

This performs the basic build pipeline:

```text
Cargo.toml
    ↓
src/lib.rs
    ↓
Rust compiler
    ↓
WebAssembly
    ↓
Trunk browser output
```

A successful build should end with output similar to:

```text
Finished ...
INFO success
```

---

## 11. Run the Simulator

From the repository root, run:

```powershell
trunk serve --open
```

Trunk will:

```text
read Cargo.toml
        ↓
compile src/lib.rs
        ↓
target WebAssembly
        ↓
generate browser assets
        ↓
start a local web server
        ↓
open the simulator in the browser
```

The development page will normally be available at:

```text
http://127.0.0.1:8080/
```

Trunk may also report equivalent localhost addresses.

Keep the terminal running while using the development server.

To stop the server, press:

```text
Ctrl + C
```

---

## 12. Expected Current Checkpoint

At the current development checkpoint, a successful browser run should display:

```text
Majorana WebGPU Wave Simulator

WebGPU adapter acquired successfully.
```

This confirms that the following path is working:

```text
Rust
    ↓
WebAssembly
    ↓
wgpu::Instance
    ↓
browser WebGPU
    ↓
wgpu::Adapter
    ↓
compatible GPU adapter acquired
```

The project has not yet completed GPU-device creation, GPU-buffer allocation, state upload, or GPU readback.

---

## 13. Generated Build Files

Cargo and Trunk generate local build directories:

```text
target/
dist/
```

These directories are intentionally excluded from Git through `.gitignore`.

`target/` contains:

* compiled Rust dependencies,
* intermediate compiler output,
* WebAssembly build intermediates,
* debugging information,
* Cargo build caches.

`dist/` contains:

* generated browser files,
* compiled WebAssembly output,
* generated JavaScript/WASM loader glue,
* processed HTML assets.

Both directories can be regenerated and should not be committed.

The repository currently ignores them with:

```gitignore
/target/
/dist/
```

Before adding any future build tool, asset pipeline, profiler, benchmark system, or generated-data workflow, its output directories should be identified and added to `.gitignore` before they are committed.

### Cargo.lock Is Intentionally Tracked

Do **not** add `Cargo.lock` to `.gitignore`.

`Cargo.lock` records the exact dependency versions selected by Cargo and is intentionally kept in the repository so application builds are reproducible.

---

## 14. Basic Git Workflow

Before making changes, inspect the repository state:

```powershell
git status
```

After editing a specific file, stage only that file when appropriate:

```powershell
git add README.md
```

or:

```powershell
git add src/lib.rs
```

Check what is staged:

```powershell
git status
```

Create a commit:

```powershell
git commit -m "describe the completed checkpoint"
```

Push committed work to the current remote branch:

```powershell
git push
```

Avoid automatically staging every generated or modified file without first checking:

```powershell
git status
```

In particular, generated directories such as `target/` and `dist/` should never appear among files being committed.

---

## 15. Live Server / Go Live Note

The earlier JavaScript prototype could be served directly through the VS Code Live Server / Go Live extension.

The active Rust/WebAssembly project should instead be run with:

```powershell
trunk serve --open
```

Live Server alone cannot compile:

```text
src/lib.rs
```

into WebAssembly.

The current browser-development path is therefore:

```text
index.html
      ↓
Trunk
      ↓
Cargo / Rust
      ↓
WebAssembly
      ↓
browser
```

rather than the previous JavaScript path:

```text
index.html
      ↓
main.js
      ↓
browser
```



# Why the Project Pivoted From Weyl to Majorana

The original project propagated a two-component complex Weyl spinor under a Hamiltonian of the form

```math
H_W
=
\boldsymbol{\sigma}\cdot\left(-i\nabla-\mathbf A(\mathbf x,t)\right).
```

That formulation was attractive for several reasons:

- a Weyl spinor contains only two complex components,
- four `f32` values per lattice site are sufficient,
- the kinetic term is naturally compatible with Fourier/SLAC differentiation,
- FFT-based derivatives avoid the local finite-difference stencil that produces the usual naive lattice-fermion doubling,
- and a Chebyshev/Jacobi-Anger expansion offered a possible route to large time steps without Trotterization error.

The scattering problem changed the design priorities.

The original plan used a spatially varying vector potential as the primary scattering structure. A numerical problem appears when an ideal reflecting structure is represented by taking the magnitude of the vector potential arbitrarily large. Increasing the potential also increases the spectral width of the Hamiltonian, which in turn increases the polynomial order required by a Chebyshev propagator. Taking an idealized limit such as

```math
A_{\max}\rightarrow\infty
```

is therefore incompatible with a fixed finite spectral bound and a fixed finite polynomial order.

A spatially varying **mass profile** provides a different scattering mechanism. In a region of approximately constant mass, the relativistic dispersion relation is

```math
E^2=\mathbf p^2+m^2
```

in natural units. When

```math
|E|<|m|,
```

the momentum in that region becomes imaginary and the wave is evanescent rather than propagating. A semi-infinite massive region therefore has no propagating transmitted mode below the local mass gap; a finite-width region produces exponentially suppressed tunneling that can approach an effective reflector without requiring an unbounded vector potential.

This makes a spatially dependent mass

```math
m=m(\mathbf x,t)
```

a natural object for the simulator.

The mass term, however, exposes an important representation issue.

In a two-component complex Majorana/Weyl formulation, the mass term couples the spinor to its complex conjugate. The equation is therefore **real-linear rather than complex-linear**. If one insists on restoring an ordinary complex-linear evolution by explicitly enlarging the state to contain both a spinor and its conjugate partner, the stored representation grows. That is undesirable for a large three-dimensional GPU lattice where memory bandwidth and buffer size are first-order constraints.

The pivot is therefore not simply a change of gamma matrices. It is a change in the computational representation:

```text
2-component complex Weyl representation
        ↓
Majorana mass introduces real-linearity
        ↓
avoid explicit charge-conjugate state doubling
        ↓
4-component real Majorana representation
        ↓
real anti-Hermitian time generator
        ↓
J-based spectral transform / J-DFT
```

A two-component complex spinor already contains exactly four real numbers. The Majorana representation uses those four real degrees of freedom directly.

---

# Project Goal

The immediate numerical goal is to propagate a real four-component Majorana spinor

```math
\Psi(\mathbf x,t)\in\mathbb R^4
```

on a periodic lattice with a Hamiltonian containing a spatially and eventually temporally varying mass profile.

In natural units, the Dirac/Majorana Hamiltonian is written

```math
H
=
-i\,\boldsymbol\alpha\cdot\nabla
+
\beta\,m(\mathbf x,t).
```

The long-term simulator should support:

- real four-component Majorana wavefunctions,
- spatially varying mass barriers and interfaces,
- moving or time-dependent mass profiles,
- FFT/SLAC-style pseudospectral derivatives,
- norm-preserving time evolution,
- GPU-resident state buffers,
- interactive visualization,
- reproducible numerical error bounds,
- and later connection to the larger D-CTC and Smith-chart-style experiments that motivated the original wave simulator.

---

# Majorana Representation

The project will use a Majorana representation of the gamma matrices in which the four-component Majorana condition can be represented by a real state vector.

The gamma matrices satisfy

```math
\{\gamma^\mu,\gamma^\nu\}
=
2\eta^{\mu\nu}I,
```

with metric convention

```math
\eta=\mathrm{diag}(+1,-1,-1,-1).
```

Define

```math
\alpha^i=\gamma^0\gamma^i,
\qquad
\beta=\gamma^0.
```

In the chosen Majorana representation:

- the physical spinor can be stored as four real numbers,
- the kinetic matrices $\alpha^i$ are real,
- $\beta$ is purely imaginary,
- and the complete time generator can be written as a real operator.

The exact Pauli-product representation of the gamma matrices is representation-dependent and should be centralized in one tested module rather than duplicated throughout the codebase.

---

# Real Time-Evolution Generator

Start from

```math
i\partial_t\Psi
=
H\Psi
```

with

```math
H
=
-i\boldsymbol\alpha\cdot\nabla
+
m\beta.
```

In the Majorana representation, $H$ is purely imaginary and Hermitian. Write

```math
H=iK.
```

Then

```math
K
=
-\boldsymbol\alpha\cdot\nabla
-i\,m\beta,
```

and the state equation becomes

```math
\boxed{
\partial_t\Psi=K\Psi
}
```

with a real generator $K$.

Because $H$ is Hermitian,

```math
K^\dagger=-K.
```

For the real representation this is the skew/anti-Hermitian generator that produces norm-preserving orthogonal evolution:

```math
\Psi(t)=e^{Kt}\Psi(0).
```

The physical norm is simply

```math
\|\Psi\|^2
=
\Psi^T\Psi,
```

and an exact real evolution satisfies

```math
\frac{d}{dt}\left(\Psi^T\Psi\right)=0.
```

This is the fundamental evolution picture of the Majorana simulator.

---

# The Real Complex Structure J

The spatial transform will be organized around

```math
\boxed{
J=i\gamma^5
}
```

where

```math
\gamma^5
=
i\gamma^0\gamma^1\gamma^2\gamma^3.
```

For the intended Majorana representation, $J$ is a real antisymmetric matrix satisfying

```math
J^T=-J,
\qquad
J^2=-I.
```

It therefore plays the role of a complex structure on the real four-dimensional spinor space.

The corresponding real rotation is

```math
\boxed{
e^{J\theta}
=
I\cos\theta
+
J\sin\theta.
}
```

A crucial algebraic property is

```math
[J,\alpha^i]=0,
```

while

```math
\{J,\beta\}=0.
```

This split is central to the proposed implementation:

- the kinetic/translation part is compatible with the chosen J-complex structure,
- the mass term reverses that complex structure,
- therefore the spatial transform can behave like an ordinary complex FFT even though the **full Majorana evolution remains real-linear**.

---

# J-DFT

The proposed J-DFT uses the kernel

```math
\boxed{
e^{-J\mathbf k\cdot\mathbf x}
}
```

for the forward transform and

```math
e^{+J\mathbf k\cdot\mathbf x}
```

for the inverse transform, subject to the final sign and normalization convention chosen by tests.

Because

```math
e^{J\theta}
=
\cos\theta+J\sin\theta,
```

the transform is entirely real at the mathematical level.

A generic real Fourier contribution can be written

```math
\Psi_{\mathbf k}(\mathbf x)
=
C\cos(\mathbf k\cdot\mathbf x)
+
D\sin(\mathbf k\cdot\mathbf x),
```

with arbitrary real four-spinors $C$ and $D$. A single fixed $J$ is sufficient because

```math
e^{J\theta}A+e^{-J\theta}B
=
(A+B)\cos\theta
+
J(A-B)\sin\theta.
```

Choosing

```math
A=\frac12(C-JD),
\qquad
B=\frac12(C+JD)
```

recovers any real sine/cosine mode. The simulator therefore does **not** need multiple independent choices of $J$ to obtain a complete real Fourier basis.

The choice $J=i\gamma^5$ is retained because it commutes with the kinetic $\alpha^i$ matrices and therefore gives a clean spectral representation of spatial derivatives.

---

# J-DFT and Conventional Complex FFT Libraries

The J-DFT is mathematically a real transform, but it can potentially use optimized conventional complex FFT implementations without introducing a physical complex state or a charge-conjugate copy.

If the four real Majorana components are stored in a basis/order that groups the two J-rotation planes together, the same memory can be **viewed** as two complex values:

```text
4 real f32 values
    ⇅ zero-copy reinterpretation
2 complex values
```

This does not change the number of physical degrees of freedom:

```math
4\text{ real scalars}
=
2\text{ complex storage pairs}.
```

The packed complex representation is an implementation device for the J-rotations used by the FFT butterflies.

It is important to distinguish this from explicit charge-conjugate doubling:

```text
J-packed storage:
    4 real values/site
    ↔ 2 complex storage values/site

explicit conjugate/Nambu-style storage:
    spinor + conjugate spinor
    → additional stored state
```

The FFT backend may use ordinary complex arithmetic internally, but the physical state remains the same four real Majorana components.

## Where Complex Encoding Stops Being Ordinary Complex-Linear Evolution

If a real operator $M$ commutes with $J$,

```math
[M,J]=0,
```

then it acts as an ordinary complex-linear operation in the packed J-complex view.

If it anticommutes with $J$,

```math
\{M,J\}=0,
```

then the same real operator appears antilinear in that packed complex view.

This is why the J-DFT/SLAC kinetic operation can use conventional FFT machinery while the full Majorana time propagator cannot simply be treated as an arbitrary two-component complex unitary.

No separate conjugate array is required if those operations are applied directly to the underlying four-real-component memory.

---

# SLAC / Fourier-Pseudospectral Derivative

For the J-DFT convention

```math
\widetilde\Psi(\mathbf k)
=
\sum_{\mathbf x}
 e^{-J\mathbf k\cdot\mathbf x}\Psi(\mathbf x),
```

a spatial derivative maps schematically to

```math
\partial_i
\longrightarrow
Jk_i.
```

Because

```math
[J,\alpha^i]=0,
```

the kinetic part of the real Majorana generator can be applied directly in J-momentum space.

The intended Hamiltonian/generator application therefore has the broad structure

```text
position-space real Majorana spinor
        ↓
J-DFT / packed complex FFT
        ↓
SLAC momentum multiplication
        ↓
kinetic alpha action
        ↓
inverse J-DFT
        ↓
position-space mass action
```

The implementation should investigate whether the momentum multiplication and alpha-matrix action can be fused into the FFT-related compute path.

The wavefunction should remain resident on the GPU whenever possible.

---

# Spatially Varying Mass Strategy

The primary scattering structure is now a mass profile

```math
m(\mathbf x,t).
```

A simple one-dimensional interface may be written

```math
m(x,t)
=
m_0\,\Theta\!\left(x-x_0(t)\right).
```

For a static region with local mass $m_0$, the dispersion relation is

```math
E^2=p^2+m_0^2.
```

When

```math
|E|<|m_0|,
```

there is no propagating momentum in that region. A finite-width mass barrier produces an evanescent wave and exponentially suppressed transmission; a semi-infinite region gives total reflection of the propagating channel below the gap.

This gives the simulator a finite-parameter way to create strong reflecting structures without sending a vector potential to infinity.

---

# Time-Dependent / Moving Mass Profiles

The mass profile may eventually move with time.

The initial strategy remains the same incremental frozen-operator idea developed in the Weyl prototype. During interval $j$, approximate

```math
m(\mathbf x,t)
\approx
m_j(\mathbf x).
```

This defines a frozen generator

```math
K_j
=
-\boldsymbol\alpha\cdot\nabla
-i\beta m_j(\mathbf x).
```

The state is propagated under that static generator, then handed to the next interval:

```text
Psi_0
  --K_0--> Psi_1
  --K_1--> Psi_2
  --K_2--> Psi_3
  --> ...
```

The ordering matters because generators corresponding to different mass profiles generally do not commute.

The first implementation should therefore favor correctness and explicit chronological handoff over a sophisticated continuous time-ordering scheme.

---

# Time Propagation After the Pivot

The original Weyl design selected a Chebyshev/Jacobi-Anger expansion for

```math
e^{-iH\Delta t}.
```

That remains an important candidate because it can fast-forward a static generator without Trotter splitting and because it uses only repeated Hamiltonian applications.

However, the Majorana pivot changes the implementation question.

The physically stored state now obeys

```math
\partial_t\Psi=K\Psi,
\qquad
K^\dagger=-K,
```

rather than being treated everywhere as an unconstrained complex state under a generic complex-linear Hamiltonian.

Therefore the old complex Chebyshev implementation must **not** simply be copied into the new simulator. The following must be established explicitly:

1. how the Bessel/Chebyshev expansion is represented on the real Majorana state,
2. whether the recurrence can remain entirely in four-real-component storage,
3. how the J-complex structure is used, if at all, in the polynomial coefficients,
4. how the J-anticommuting mass term is handled without storing a charge-conjugate state,
5. and how norm preservation is verified at finite truncation order.

The project goal is still to obtain large, controlled time steps without Trotterization error, but the exact real-Majorana propagator is now a design problem rather than an assumed solved component.

---

# Retained Chebyshev / Bessel-Tail Analysis

The following analysis from the Weyl prototype remains useful whenever the evolution is expressed through a Hermitian operator whose spectrum has been rescaled to $[-1,1]$, or when an equivalent Majorana formulation is proven to inherit the same bound.

Suppose a Hermitian operator has spectral bounds

```math
E_{\min},\qquad E_{\max}.
```

Define

```math
a=\frac{E_{\max}-E_{\min}}{2},
\qquad
c=\frac{E_{\max}+E_{\min}}{2},
```

and

```math
\widetilde H=\frac{H-cI}{a}.
```

Then

```math
e^{-iH\Delta t}
=
e^{-ic\Delta t}
\left[
J_0(z)T_0(\widetilde H)
+
2\sum_{n=1}^{\infty}
(-i)^nJ_n(z)T_n(\widetilde H)
\right],
```

where

```math
z=a\Delta t.
```

The Chebyshev vectors satisfy

```math
\phi_0=\psi,
\qquad
\phi_1=\widetilde H\psi,
```

```math
\phi_{n+1}
=
2\widetilde H\phi_n-\phi_{n-1}.
```

No explicit matrix powers are required.

## Truncation Bound

If the series is retained through order $M$, then

```math
\|R_M\|
\le
2\sum_{n=M+1}^{\infty}|J_n(z)|.
```

Using

```math
|J_n(z)|
\le
\frac{(|z|/2)^n}{n!},
```

and defining

```math
x=\frac{|z|}{2},
```

the conservative analytic bound is

```math
\boxed{
\|R_M\|
\le
\frac{
2x^{M+1}
}{
(M+1)!\left(1-\frac{x}{M+2}\right)
}
}
```

provided

```math
M+2>x.
```

This remains a useful reference because the eventual Majorana propagator should also have an explicit, reproducible error criterion rather than a hard-coded polynomial order.

---

# Spectral Width With a Mass Profile

For the free massive Dirac/Majorana Hamiltonian with uniform mass,

```math
H(\mathbf k)
=
\boldsymbol\alpha\cdot\mathbf k
+
\beta m,
```

the continuum eigenvalue magnitude is

```math
|E(\mathbf k)|
=
\sqrt{|\mathbf k|^2+m^2}.
```

For a three-dimensional FFT grid,

```math
|k_x|_{\max}
\approx
|k_y|_{\max}
\approx
|k_z|_{\max}
\approx
\frac{\pi}{\Delta x},
```

so

```math
|\mathbf k|_{\max}
\approx
\frac{\sqrt3\pi}{\Delta x}.
```

For a nonuniform mass profile, a conservative operator bound can use the maximum magnitude

```math
m_{\max}
=
\max_{\mathbf x}|m(\mathbf x)|.
```

A simple triangle-inequality estimate is

```math
\|H\|
\lesssim
|\mathbf k|_{\max}+m_{\max}
```

in natural units. A tighter bound may be possible and should be derived or measured before choosing the final Chebyshev order.

The important numerical lesson from the Weyl prototype remains unchanged:

> The required propagation order must be derived from the actual spectral bound and requested error tolerance. It should not be treated as a universal constant.

---

# Current Spinor Memory Layout

The Majorana representation preserves the original memory target.

A two-component complex Weyl spinor contained

```math
2\text{ complex values}=4\text{ real values}.
```

A Majorana spinor is now stored directly as

```math
\Psi
=
\begin{pmatrix}
\psi_0\\
\psi_1\\
\psi_2\\
\psi_3
\end{pmatrix},
\qquad
\psi_a\in\mathbb R.
```

Using 32-bit floats:

```text
[ psi0, psi1, psi2, psi3 ]
```

requires

```math
4\times4\text{ bytes}=16\text{ bytes/site}.
```

This is the same raw state memory as the original two-component complex Weyl representation.

By contrast, explicitly storing both a two-component complex spinor and a separate conjugate partner would require eight real `f32` values, or 32 bytes/site, before additional work buffers are considered.

The exact component order may be changed to make the J-rotation planes contiguous for FFT packing, but such a change must be benchmarked and documented.

Important layout goals are:

- 16 bytes/site for the physical state,
- coalesced GPU access,
- zero- or low-copy reinterpretation for packed complex FFTs,
- simple WGSL indexing,
- reusable work buffers,
- and no persistent charge-conjugate state buffer.

---

# Rust / WebGPU Architecture

The simulation is implemented with WebGPU rather than WebGL.

The basic hierarchy is:

```text
Rust / WebAssembly
        ↓
wgpu::Instance
        ↓
wgpu::Adapter
        ↓
wgpu::Device + wgpu::Queue
        ↓
wgpu::Buffer
        ↓
Bind Groups
        ↓
Compute Pipelines
        ↓
Command Encoders
        ↓
browser WebGPU
        ↓
GPU
```

The browser's native WebGPU implementation still ultimately provides the GPU access. The `wgpu` Rust crate provides the host-side API used by the simulator and maps to browser WebGPU when compiled for WebAssembly.

The development machine used by the original prototype reported:

```text
vendor: NVIDIA
architecture: Lovelace
fallback adapter: false
```

with an NVIDIA RTX 4070 Super.

A valid `GPUDevice` was successfully created in the Weyl prototype.

---

# WebGPU Storage Limit Observed in the Prototype

The prototype reported

```javascript
device.limits.maxStorageBufferBindingSize
```

as

```text
134217728 bytes
```

or

```math
128\text{ MiB}.
```

This is **not total GPU VRAM**. It is a WebGPU resource/binding limit for a single storage-buffer binding under the current device configuration.

The distinction between physical VRAM and WebGPU binding/resource limits must remain explicit throughout development.

This limit is another reason the physical wavefunction representation should remain compact.

---

# Development Environment

The active project is currently developed with:

- Windows,
- Visual Studio Code,
- Rust,
- Cargo,
- rustup,
- the `wasm32-unknown-unknown` Rust target,
- Trunk,
- WebAssembly,
- `wasm-bindgen`,
- `wasm-bindgen-futures`,
- `web-sys`,
- `wgpu`,
- WGSL for GPU compute shaders,
- Chrome / Chromium with WebGPU support,
- Git,
- GitHub,
- NVIDIA RTX 4070 Super,
- NVIDIA Lovelace architecture.

The earlier JavaScript prototype used VS Code Live Server / Go Live.

The Rust/WebAssembly implementation is instead built and served with Trunk:

trunk serve --open

Live Server alone is not sufficient for the active Rust implementation because the Rust source must first be compiled to WebAssembly.

The active repository continues under the MIT License.

# Rust Build Artifacts and Repository Storage

Cargo and Trunk generate local build artifacts that are not part of the source repository.

The project currently ignores:

target/
dist/

`target/` contains Cargo compilation outputs, dependency builds, caches, and WebAssembly build intermediates.

`dist/` contains the generated website output produced by Trunk.

Both directories can be regenerated and should not be committed.

`Cargo.lock`, by contrast, is intentionally committed. It records the exact dependency versions selected by Cargo and helps make application builds reproducible.

Before introducing new build tools or asset pipelines, generated files and directories should be identified and added to `.gitignore` before they are accidentally committed.

---

# Repository History and Pivot

The previous repository is retained as the historical Weyl implementation:

```text
https://github.com/benjamincommeau2/weyl-webgpu
```

The new active repository is:

```text
https://github.com/benjamincommeau2/majorana-wave-simulator
```

The old repository should not be treated as wasted work. It contains the numerical reasoning, WebGPU experiments, error-bound derivations, and development checkpoints that led to the Majorana architecture.

The pivot should be visible in commit history and documentation rather than silently rewriting the project's origin.

---

# Historical JavaScript Checkpoint

At the last documented Weyl checkpoint, the browser/WebGPU setup had reached:

```text
navigator.gpu
    ↓
GPUAdapter
    ↓
GPUDevice
    ↓
GPUBuffer allocation
```

A four-`Float32` CPU spinor existed and a 16-byte GPU storage buffer had been allocated, but the actual CPU-to-GPU `device.queue.writeBuffer(...)` transfer had not yet been implemented.

The Majorana repo should re-establish this checkpoint using Majorana naming and semantics rather than silently assuming that later GPU functionality already exists.

After the Rust implementation has re-established the GPU adapter, device, and queue, the smallest state-memory implementation steps should remain:

```text
create 4-real Majorana CPU state
        ↓
allocate 16-byte GPU storage buffer
        ↓
upload with device.queue.writeBuffer(...)
        ↓
read back and verify exact values
        ↓
only then introduce the first Majorana matrix / J operation
```

This preserves the original incremental development philosophy.

---

# Current Rust Migration Checkpoint

The active Majorana repository has begun migrating its browser host code from JavaScript to Rust/WebAssembly.

The following Rust browser foundation has been established:

```text
Rust toolchain installed
        ↓
wasm32-unknown-unknown target installed
        ↓
Trunk installed
        ↓
Cargo project created
        ↓
Rust compiled to WebAssembly
        ↓
browser successfully executes Rust startup code
        ↓
Rust successfully accesses the HTML document
        ↓
Rust updates the browser DOM
```

The previous `main.js` entry point has been removed.

The current browser entry path is:

```text
index.html
    ↓
Trunk
    ↓
src/lib.rs
    ↓
Rust / WebAssembly
```

The project has also introduced `wgpu` and `wasm-bindgen-futures` as dependencies in preparation for recreating the previous WebGPU initialization checkpoint in Rust.

A GPU adapter and device have not yet been requested by the Rust implementation.

# Development Philosophy

The project is intentionally implemented in small interactive steps.

Each new WebGPU API call, mathematical operation, memory layout, or transform convention should be understood before moving forward.

The development loop should generally be:

```text
understand
    ↓
implement smallest piece
    ↓
observe
    ↓
test
    ↓
verify
    ↓
commit
    ↓
continue
```

Large blocks of untested implementation code should be avoided.

New code should include explanatory comments describing why each important line exists.

The pivot itself is an example of this philosophy: when the mathematical representation no longer matches the computational goal, change the representation before optimizing the wrong model.

---

# Test-Driven Development Strategy

The intended development cycle is:

```text
write failing test
       ↓
implement smallest change
       ↓
make test pass
       ↓
refactor if needed
       ↓
run tests again
       ↓
commit
```

Testing should be divided into CPU mathematical tests, WebGPU integration tests, and numerical-physics validation tests.

---

## Unit Tests

CPU-side tests should eventually cover:

- real four-component spinor manipulation,
- Majorana gamma-matrix algebra,
- $\alpha^i$ and $\beta$ identities,
- $J=i\gamma^5$,
- $J^2=-I$,
- $J^T=-J$,
- $[J,\alpha^i]=0$,
- $\{J,\beta\}=0$,
- real-generator anti-Hermiticity/skew symmetry,
- wavefunction normalization,
- spatial-grid indexing,
- momentum-grid construction,
- mass-profile construction,
- J-DFT convention tests,
- packed-complex/J-real equivalence tests,
- SLAC derivative reference calculations,
- spectral-bound estimation,
- Chebyshev recurrence if retained,
- Bessel coefficients and analytic tail bounds if retained,
- analytic free Majorana solutions.

CPU-side mathematical tests should use Rust's built-in test framework wherever practical:

cargo test

Browser/WebAssembly integration testing should be selected deliberately once the first wgpu integration checkpoint is working.

Potential tools include Rust/WASM browser-testing infrastructure and, where full browser automation is useful, Playwright.

The testing stack should not require JavaScript unit-test infrastructure merely because the application executes in a browser.

---

## WebGPU Integration Tests

GPU-specific tests should verify:

- GPU buffer creation,
- CPU-to-GPU transfer,
- GPU-to-CPU readback,
- WGSL shader compilation,
- compute-pipeline creation,
- real Majorana matrix operations,
- J rotation correctness,
- packed complex FFT compatibility,
- J-DFT reconstruction,
- SLAC derivative correctness,
- mass-term application,
- complete generator application,
- and eventually the real time-propagation recurrence.

A browser automation framework such as Playwright may eventually be used so tests execute inside a real Chromium/WebGPU environment.

---

# Numerical Physics Tests

Successful code execution is not sufficient evidence that the physics is correct.

## Norm Conservation

For exact real Majorana evolution,

```math
\Psi(t)^T\Psi(t)
=
\text{constant}
```

up to expected floating-point and propagation-approximation error.

## J-DFT Reconstruction

A forward J-DFT followed by its inverse should reconstruct the original real state:

```math
F_J^{-1}F_J\Psi
\approx
\Psi.
```

This should be tested both with a direct small-size reference implementation of the J kernel and with the optimized packed-complex FFT backend.

## J-DFT vs Direct Real Rotation

For small lattices, compute the transform directly from

```math
e^{J\theta}
=
I\cos\theta+J\sin\theta
```

and compare it to the packed-complex FFT implementation.

This test is important because the conventional FFT is only an implementation backend; the mathematical transform being validated is the J-DFT.

## Free Massless Propagation

With

```math
m=0,
```

the numerical solution should be compared against independently computed free propagation.

## Uniform Massive Propagation

For constant $m$, the dispersion should reproduce

```math
E^2=\mathbf p^2+m^2.
```

## Mass-Step Reflection

A wave packet incident on a mass step should be tested in regimes above and below the local mass gap.

For $|E|<|m|$ in a semi-infinite massive region, the transmitted solution should be evanescent rather than propagating.

For a finite barrier, transmission should decrease exponentially as the evanescent width is increased.

## Generator Skew-Symmetry

Small CPU reference matrices should verify

```math
K^T=-K
```

under the chosen discrete derivative and boundary convention.

## Propagator Convergence

Whichever polynomial or other fast-forward propagation method is selected should be tested for convergence against a high-accuracy small-system reference exponential.

## Grid Convergence

Solutions should be compared while varying:

- spatial resolution,
- domain size,
- propagation interval,
- mass-profile resolution,
- polynomial order or other propagator tolerance.

## CPU vs GPU Reference Calculations

Small problems should be computed both on the CPU and GPU. The CPU implementation should remain a high-clarity reference against which optimized GPU kernels are tested.

---

# Performance Goals

The project is intended to maintain smooth interactive visualization while numerical propagation is occurring.

Performance work should be measurement-driven.

Important principles include:

- keep the physical state on the GPU,
- preserve the 16-byte/site four-real-component state representation,
- avoid persistent charge-conjugate state buffers,
- minimize CPU-to-GPU transfers,
- minimize GPU-to-CPU readbacks,
- avoid unnecessary CPU/GPU synchronization,
- reuse GPU buffers,
- reuse compute pipelines,
- reuse bind groups where practical,
- batch GPU commands,
- minimize FFT passes,
- exploit packed J-complex views only where they reduce work,
- maintain GPU-friendly memory access,
- avoid unnecessary allocations during the simulation loop,
- benchmark workgroup sizes,
- separate simulation frequency from rendering frequency,
- profile before optimizing.

---

# Simulation Throughput vs Rendering FPS

The numerical solver and visual renderer are different workloads.

The project should distinguish between

```text
simulation propagation intervals per second
```

and

```text
rendered frames per second
```

A simulation does not need to perform exactly one propagation step for every rendered frame.

Eventually the application may use

```text
physics simulation loop
          ↓
GPU Majorana state
          ↓
render loop
```

with simulation and visualization coordinated without unnecessary synchronization.

---

# GPU Performance Philosophy

The objective is efficient utilization rather than maximum electrical power draw.

Important concerns include:

```text
GPU occupancy
memory bandwidth
workgroup scheduling
buffer reuse
FFT efficiency
J-pair memory layout
command submission
CPU/GPU synchronization
shader arithmetic
rendering workload
```

The application should avoid hardware-specific assumptions unless profiling shows they are justified.

---

# Planned Development Sequence

The revised development path is approximately:

```text
WebGPU initialization
        ↓
4-real-component Majorana CPU state
        ↓
GPU buffer allocation
        ↓
CPU → GPU transfer
        ↓
GPU → CPU readback
        ↓
automated test infrastructure
        ↓
first WGSL compute shader
        ↓
Majorana matrix representation
        ↓
J = i gamma5
        ↓
verify J^2 = -I and J^T = -J
        ↓
alpha / beta algebra tests
        ↓
spatial grid
        ↓
momentum grid
        ↓
direct small-size J-DFT reference
        ↓
packed-complex FFT implementation of J-DFT
        ↓
J-DFT equivalence tests
        ↓
SLAC derivative
        ↓
kinetic alpha operator
        ↓
spatial mass profile m(x)
        ↓
complete real generator K Psi
        ↓
verify skew-symmetry / norm behavior
        ↓
spectral-bound estimation
        ↓
select and validate real Majorana fast-forward propagator
        ↓
static-mass propagation
        ↓
moving / time-dependent m(x,t)
        ↓
visualization
        ↓
profiling
        ↓
optimization
```

This sequence is a roadmap rather than an immutable specification.

The pivot itself demonstrates that numerical or mathematical constraints may require architectural changes.

---

# Public Deployment Goal

A long-term goal of the project is to make the simulator publicly accessible in a web browser so that anyone can experiment with it without installing Rust or building the project locally.

The preferred initial deployment target is GitHub Pages.

The intended deployment path is:

```text
Rust source
    ↓
Cargo / Trunk build
    ↓
WebAssembly + generated browser assets
    ↓
GitHub Actions
    ↓
GitHub Pages
    ↓
public HTTPS website
```

GitHub Pages is suitable because the browser application is ultimately distributed as static HTML, JavaScript/WebAssembly loader code, and `.wasm` assets generated by Trunk.

The repository should continue to keep generated local build directories such as:

```text
target/
dist/
```

out of source control.

Instead of committing generated deployment output directly, the preferred deployment architecture is for GitHub Actions to build the project automatically from the source repository and publish the generated site artifact to GitHub Pages.

The public version should eventually allow visitors to:

* open the simulator directly in a compatible browser,
* run the WebGPU simulation locally on their own GPU,
* interact with simulation parameters,
* visualize the Majorana wavefunction and scattering behavior,
* and explore the project without installing the development toolchain.

The application should remain usable as a static browser application without requiring a dedicated backend server unless a future feature explicitly requires one.

Because WebGPU support depends on the user's browser and hardware, the deployed application should eventually detect unavailable WebGPU support and display a clear compatibility message rather than failing silently.

Deployment should be introduced only after the basic GPU memory path and initial compute functionality are stable enough to provide a meaningful public demonstration.


# Important Numerical Questions Still To Resolve

Several choices remain intentionally unresolved:

- whether the first Majorana simulator should be 1D, 2D, or 3D,
- the physical unit convention,
- whether variables should be nondimensionalized,
- spatial boundary conditions,
- exact Majorana gamma-matrix convention,
- exact component ordering in GPU memory,
- J-DFT sign and normalization convention,
- whether the J-pairs can be used as a zero-copy packed complex FFT view in the final WebGPU FFT backend,
- FFT implementation strategy in WebGPU,
- FFT memory layout,
- grid dimensions,
- momentum-grid conventions,
- SLAC Nyquist-mode convention,
- treatment of J-anticommuting mass operations in packed-complex storage,
- whether all full time-evolution work should remain explicitly in the real view,
- spectral-bound estimation with nonuniform mass,
- the final fast-forward propagator for the real anti-Hermitian generator,
- whether the previous Chebyshev/Jacobi-Anger method can be reformulated without state doubling,
- propagation-error tolerance,
- moving-mass interval selection,
- discontinuity handling,
- Gibbs phenomena near sharp mass steps,
- precision requirements,
- GPU buffer organization,
- visualization architecture,
- GPU timing methodology,
- performance benchmark methodology.

These decisions should not be silently assumed.

Each should be:

```text
discussed
   ↓
documented
   ↓
implemented
   ↓
tested
   ↓
validated
```

---

# Long-Term Optimization Questions

Once correctness is established:

- What four-real-component ordering gives the best J-pair packing?
- Can the J-DFT reuse a conventional WebGPU complex FFT implementation with no physical-state copy?
- Can the alpha-matrix action be fused with spectral momentum multiplication?
- Can the local mass operation be fused with another position-space pass?
- How many FFTs are required for one complete generator application?
- Can polynomial recurrence vectors reuse existing buffers?
- Can ping-pong buffers eliminate allocation?
- Can command encoders contain multiple propagation iterations?
- When does command-buffer size become significant?
- What workgroup sizes perform best?
- Should rendering consume the simulation buffer directly?
- How frequently should diagnostic values be read back to the CPU?
- Can norm calculations be performed entirely on the GPU?
- How should reductions be implemented?
- What grid sizes preserve smooth rendering?
- What operations become memory-bandwidth limited?
- What operations become arithmetic-throughput limited?

These questions should be answered through measurement rather than assumption.

---

# Repository Quality Goals

The repository is intended to be both a physics experiment and a professional software-engineering project.

It should progressively demonstrate:

- clear documentation,
- meaningful commit history,
- consistent commit messages,
- automated tests,
- numerical validation,
- modular architecture,
- documented design decisions,
- reproducible benchmarks,
- performance measurements,
- understandable code comments,
- issue tracking,
- versioned milestones,
- continuous integration where practical.

Preferred commit prefixes may include:

```text
feat:
fix:
test:
docs:
perf:
refactor:
chore:
```

Examples after the pivot:

```text
feat: upload initial real Majorana spinor to GPU storage
```

```text
test: verify J squared equals minus identity
```

```text
test: compare direct J-DFT with packed complex FFT
```

---

# Guidance for Future LLM Sessions

If development continues in another AI conversation, provide the AI with this repository or, at minimum:

- this `README.md`,
- `Cargo.toml`,
- `Cargo.lock`,
- `.gitignore`,
- `index.html`,
- Rust files under `src/`,
- test files,
- WGSL shader files when they exist,
- Trunk configuration if one is later introduced.

The AI should preserve the following development style:

1. Work in very small interactive steps.
2. Introduce only a small number of lines of code at a time.
3. Add explanatory comments to newly introduced code.
4. Explain every new WebGPU concept.
5. Do not skip ahead to large implementations.
6. Introduce tests alongside mathematical functionality.
7. Prioritize correctness before optimization.
8. Keep GPU-performance and memory-bandwidth considerations visible throughout development.
9. Do not silently assume FFT/J-DFT sign conventions.
10. Do not silently assume a gamma-matrix representation.
11. Keep the distinction between the four-real physical state and any packed-complex FFT view explicit.
12. Do not introduce a persistent charge-conjugate state buffer unless the architecture is deliberately changed and documented.
13. Do not assume the full Majorana evolution is ordinary complex-linear merely because a complex FFT backend is used.
14. Do not silently assume Chebyshev scaling or truncation conventions.
15. Maintain a clear distinction between CPU memory and GPU memory.
16. Maintain a clear distinction between simulation throughput and rendering FPS.
17. Commit working checkpoints frequently.
18. Preserve numerical CPU reference implementations when useful for testing optimized GPU code.
19. Treat the Weyl repository as design history, not as the active mathematical specification.
20. When a mathematical assumption conflicts with the memory/performance architecture, surface the conflict explicitly before coding around it.
21. The active browser host implementation is Rust/WebAssembly, not handwritten JavaScript.
22. Use `wgpu` for WebGPU host-side interaction.
23. Keep WGSL as the GPU shader language unless an explicit architectural decision changes it.
24. Introduce Rust code one small line or concept at a time during interactive development.
25. Explain unfamiliar Rust syntax when it is first introduced.
26. Before introducing a new tool that generates files, identify any build/cache/output directories that should be added to `.gitignore`.
27. Do not ignore `Cargo.lock`; it is intentionally retained for reproducible application builds.
28. Do not reintroduce `main.js` merely as a WebGPU host layer unless there is a deliberate documented reason.


---

# Exact Handoff Point for a Future LLM

The Majorana simulator has migrated its browser bootstrap from JavaScript to Rust/WebAssembly.

Confirmed working:

```text
index.html
    ↓
Trunk
    ↓
Rust
    ↓
WebAssembly
    ↓
browser DOM access
```

The old `main.js` file has been removed.

The Rust project currently declares dependencies on:

- `wasm-bindgen`,
- `web-sys`,
- `console_error_panic_hook`,
- `wgpu`,
- `wasm-bindgen-futures`.

The Rust/WASM startup path has been verified in the browser by successfully replacing the page status text with:

Rust/WASM loaded successfully.

The immediate next implementation goal is to recreate the first WebGPU initialization operation in Rust:

create wgpu::Instance
        ↓
request wgpu::Adapter
        ↓
verify adapter acquisition
        ↓
request wgpu::Device + wgpu::Queue

Do not jump ahead to GPU buffers, WGSL shaders, FFTs, J-DFTs, SLAC derivatives, or propagation until each preceding Rust/wgpu checkpoint has been implemented and observed successfully.

---

# License

This project is licensed under the MIT License.

See the repository's `LICENSE` file for details.