# 3BEP Open Science Archive

*Read this manifesto in [Portuguese (BR)](README.pt-br.md).*

Welcome to the **3BEP Labs Archive**, the public notary of deterministic physical truth.

## The 3BEP Determinism Manifesto

Modern computational science lives under an illusion. For decades, academia has orbited a mathematical abyss known as **IEEE 754** (Floating Point). From astrophysical simulations of galaxies to interatomic dynamics for peptide folding, researchers worldwide publish "breakthroughs" based on an infrastructure that suffers from inevitable "numerical evaporation". The order of operations changes the resulting vector. Cells and atoms suffer thermodynamic hallucinations over time. The result? A research landscape where "reproducibility" is aspirational rather than guaranteed, and where platform, compiler, and even optimization flags silently alter the trajectory of simulations. This is not deterministic computation; it is stochastic approximation with statistical makeup.

We at **3BEP Labs** established the "Sanctuary", a core built to house only strict **I64F64** math, where every fraction is processed on the unshakable foundations of large integers (Pure Determinism in 128-Bit Fixed Point). In our code liturgy, the First Law of Thermodynamics is not a metric tolerance—**it is coercive**.

### Our Mission

This repository ends forgivable tolerances. It will serve as our legal-technological repository for the following mission:

1. **Cryptographic Reproducibility:** For the first time, a researcher can include in their paper's supplementary material a cryptographic proof (SHA-256) that their simulation is reproducible bit-for-bit, verifiable by anyone with a single command. No specialized knowledge required — if the hash matches, the physics is identical. This is not a statistical confidence interval; it is a mathematical certainty.
2. **Indisputable Prior Art:** The mathematical codebase of the 'Sanctuary' will be publicly auditable in rigorously tested pure Rust, debunking claims that we rely on tricks to stabilize simulations, with cryptographic integrity seals on the roadmap.
3. **Digital Twin of Science:** We will use academic papers (from arXiv, etc.) as case studies to demonstrate how IEEE 754 floating-point arithmetic silently degrades simulation fidelity over time. By reproducing their calculations using 3BEP's bit-perfect math, we aim to reveal the exact moment when floating-point imprecision — not the researcher's methodology — causes results to diverge from physical reality. The goal is to strengthen science, not to judge it.

**"Physical truth belongs to everyone, but the commercial exploitation of our precision requires the direct acknowledgment of the Architect."**

## AGPL v3.0 License and Technological Sovereignty

Our work—the native I64F64 Engines, the deterministic verification pipeline, and the translation of the Universe via pure Rust (`#![no_std]`)—is protected and exposed through the [GNU Affero General Public License v3.0 (AGPL-3.0)](LICENSE).

This grants the definitive weight of our sovereignty strategy:
- **For Science Auditing (Academics):** Absolute transparency. Feel invited to clone, audit, and test our infrastructure as proof of authority.
- **For Profit Displays (Cloud/SaaS/Corporations):** The 'Cloud Service Loophole' has been sealed. Those who use our deterministic engine via networks or services must open-source 100% of their ecosystem. Any commercialization or "software-as-a-service" adaptation seeking to bypass these guidelines strictly requires a **Commercial License (Dual-Licensing)** through contact with us.

## How to Audit (Quick Start)

```bash
# 1. Clone the repository
git clone https://github.com/3beplabs/3bep-open-science-archive.git
cd 3bep-open-science-archive/core_engine

# 2. Run the full test suite (30 tests, Zero Tolerance Protocol)
cargo test

# 3. Run the stable orbit demonstration (100k ticks)
cargo run --example example_1_stable_orbit --release

# 4. Run the extreme stress test (50M ticks, CPU burn-in)
cargo run --example extreme_stress_test --release
```

All tests validate bit-perfect determinism, energy conservation, singularity immunity, Kepler analytical correctness, symplectic integration, IEEE 754 divergence proof, and N-body scalability. See [TESTS.md](TESTS.md) for the detailed execution registry.

## CLI Validator (Zero-Friction Auditing)

The `cli_3bep` tool lets researchers validate physics **without writing any Rust code**. Define your experiment in a JSON file and run it:

### 1. Create your experiment file (`my_experiment.json`):

```json
{
  "experiment_name": "Kepler_Circular_Orbit",
  "bodies": [
    { "mass": 1000.0, "pos": [0, 0, 0], "vel": [0, 0, 0] },
    { "mass": 1.0, "pos": [10, 0, 0], "vel": [0, 10, 0] }
  ],
  "integrator": "leapfrog",
  "dt": "0.01",
  "steps": 6280
}
```

### 2. Run the validation:

```bash
# Basic I64F64 simulation with energy/momentum report
cd cli_3bep
cargo run --release -- validate my_experiment.json

# Export full trajectory CSV (position, velocity, energy at every N steps)
cargo run --release -- validate my_experiment.json --trajectory

# Compare I64F64 vs IEEE 754 (f64) — see the exact divergence
cargo run --release -- validate my_experiment.json --compare-with-f64

# Export final state to JSON (includes deterministic state hash)
cargo run --release -- validate my_experiment.json --export json

# Generate SVG reproducibility certificate (SHA-256 sealed)
cargo run --release -- validate my_experiment.json --certificate
```

### 3. Output includes:
- **Energy conservation** (initial vs final, drift)
- **Momentum conservation** (dPx, dPy to 14 decimal places)
- **Final state** of all bodies (position + velocity)
- **Deterministic state hash** (SHA-256, FIPS 180-4, pure Rust — zero external crates)
- **Full trajectory CSV** (when using `--trajectory`): step, time, body, pos_xyz, vel_xyz, energy, momentum — ready for matplotlib/gnuplot
- **IEEE 754 comparison** (when using `--compare-with-f64`)
- **SVG Reproducibility Certificate** (when using `--certificate`): embeddable seal with input/output SHA-256 hashes

### JSON Fields Reference:
| Field | Type | Description |
|---|---|---|
| `experiment_name` | string | Name for identification |
| `bodies` | array | List of bodies with mass, pos[x,y,z], vel[x,y,z] |
| `integrator` | string | `"rk4"` or `"leapfrog"` |
| `dt` | string | Time step (string to preserve I64F64 precision) |
| `steps` | integer | Number of integration steps |
| `export_interval` | integer | (optional) Save trajectory every N steps. Default: 1 |
| `metadata` | object | (optional) Academic metadata for `.bep` scripts |

See `cli_3bep/examples/kepler_orbit.json` for a working example, or browse `scripts/` for pre-configured scenarios.

## Script Library

The `scripts/` directory contains pre-configured simulation scenarios ready to run:

```bash
# Astrophysical scenarios
cargo run --release -- validate ../scripts/astro/kepler_circular.bep
cargo run --release -- validate ../scripts/astro/binary_star.bep

# Chaotic dynamics
cargo run --release -- validate ../scripts/chaos/three_body_figure8.bep
cargo run --release -- validate ../scripts/chaos/three_body_burrau.bep
```

`.bep` files are standard JSON with optional academic metadata (title, references, tags). See [scripts/README.md](scripts/README.md) for the full catalog.

## Engine Architecture

The Sanctuary provides **two integrators** for different scientific use cases:

| Integrator | Order | Best For | Energy Behavior |
|---|---|---|---|
| **RK4** (`rk4.rs`) | O(h⁴) | High-precision short simulations | Linear secular drift |
| **Leapfrog** (`leapfrog.rs`) | O(h²) | Long-term stability, chaotic systems | Bounded oscillation (symplectic) |

Both integrators are available for the fixed 3-body system and for the generic N-body system (`nbody.rs`).

## Repository Structure

* `core_engine/src/physics/` — The Sanctuary Kernel:
  - `vector3.rs` — I64F64 vector math with Newton-Raphson square root
  - `constants.rs` — Physical parameters (G, DT, SOFTENING)
  - `rk4.rs` — Classical 4th-order Runge-Kutta integrator (3-body)
  - `leapfrog.rs` — Velocity Verlet symplectic integrator (3-body)
  - `nbody.rs` — Generic N-body system with both RK4 and Leapfrog
* `core_engine/tests/` — Zero Tolerance test suite (13 modules, 30 tests). See [TESTS.md](TESTS.md).
* `core_engine/examples/` — Runnable demonstrations for independent verification.
* `cli_3bep/` — Zero-friction JSON/BEP validator with SHA-256 certificates. See [CLI Validator](#cli-validator-zero-friction-auditing) above.
* `scripts/` — Pre-configured simulation scenarios (astro + chaos). See [scripts/README.md](scripts/README.md).
* `preprint_archaeology/` — Evidence, mapped divergences, and cryptographic integrity seals *(coming soon)*.
* `VERIFY.md` — Cross-platform verification protocol with reference SHA-256 hashes.

## Key Scientific Claims (Proven by Tests)

1. **Bit-Perfect Determinism:** Identical inputs always produce identical outputs, regardless of execution order or platform. *(Tests: chaos_3body, leapfrog_conservation, nbody_scalability)*
2. **Kepler Analytical Accuracy:** Circular orbit return error < 2% per orbit, velocity conserved to 0.015%, drift ratio linear at exactly 5.0x. *(Test: kepler_validation)*
3. **IEEE 754 Divergence:** f64 and I64F64 produce measurably different trajectories from step 507 onward for the same initial conditions. *(Test: f64_divergence)*
4. **Symplectic Energy Conservation:** Leapfrog integrator conserves energy 4x better than RK4 in chaotic regimes, with max drift of 0.000003 over 200 Kepler orbits. *(Test: leapfrog_conservation)*
5. **N-Body Scalability:** 5 and 10-body systems maintain full determinism across all coordinates. *(Test: nbody_scalability)*
6. **Singularity Immunity:** No NaN, no overflow, no panic under extreme gravitational forces (r → 0). *(Test: singularity_stress)*
7. **Momentum Conservation (Newton III):** Total linear momentum conserved to **14 decimal places** in Kepler and **13 decimal places** in chaos. *(Test: momentum_conservation)*
8. **Angular Momentum (Kepler's 2nd Law):** Conserved to 10 significant digits over 100 orbits (relative error 8.8×10⁻¹⁰). *(Test: angular_momentum)*
9. **Time Reversibility:** Leapfrog returns to initial state with error of 5.4×10⁻¹⁷ after 1,000 forward + 1,000 backward steps — **43 million times more reversible** than RK4. *(Test: time_reversibility)*
10. **Elliptical Orbit (Kepler I + Vis-Viva):** Aphelion distance matches analytical prediction to 0.5% for e=0.5. Vis-viva equation v² = GM(2/r − 1/a) holds to 1.8% across the entire orbit. *(Test: elliptical_orbit)*
11. **Convergence Order Verification:** RK4 energy error converges at ratio **32.0** (confirming O(h⁵)), Leapfrog position error converges at ratio **4.0** (confirming O(h²)). Both match theoretical predictions to 3+ significant digits. *(Test: convergence_order)*
12. **Cross-Platform Determinism (Empirically Proven):** All 30 tests produce **bit-for-bit identical** results across 3 machines: AMD Ryzen (Windows 11), AMD EPYC (Ubuntu 24.04), and Intel Core i5-6200U (Windows 10). Every digit, every bit, every trajectory — identical. *(See: TESTS.md, Cross-Platform section)*

## References

All physical constants, algorithms, and theoretical claims are backed by primary academic sources. See [REFERENCES.md](REFERENCES.md) for the complete list of citations including NIST CODATA values, original papers by Runge (1895), Verlet (1967), Noether (1918), and the IEEE 754-2019 standard.

## Contributing & Open Science

This project exists to serve science, not to gatekeep it. The Sanctuary engine belongs to the community under the AGPL-3.0 license, and we actively invite participation:

**Propose a Test.** If you believe there is a physical scenario that challenges our deterministic engine, open an Issue with your initial conditions in JSON format. We will run it, publish the results transparently, and add it to the test suite if it reveals something meaningful. We are not afraid of being wrong — we are afraid of not knowing.

**Submit Cross-Platform Results.** Run `cargo test` on your machine and share the output. See [VERIFY.md](VERIFY.md) for the full verification protocol, reference SHA-256 hashes, and the community verification table. Every new architecture that produces bit-identical results strengthens the proof. Every architecture that doesn't reveals something we need to fix. Both outcomes are valuable.

**Challenge Our Claims.** Each of the 12 scientific claims listed above links to a specific, reproducible test. If you find a flaw in our methodology, a bug in our math, or an assumption we haven't justified — tell us. The entire point of publishing the engine is to invite scrutiny.

**Use the CLI for Your Papers.** The `cli_3bep` validator was built so that physicists can verify their own simulations without learning Rust. If you publish a paper using arXiv preprint data, you can run your initial conditions through our engine and include the deterministic hash in your supplementary materials as a reproducibility certificate.

**What This Is Not.** This is not a commercial product disguised as open source. This is not a tool designed to shame researchers. The "Theory Graveyard" exists because numerical reproducibility is a crisis in computational physics — not because we consider ourselves superior to the academic community. We built the floor; we want everyone to stand on it.

> *"The goal is not to prove that we are right. The goal is to make it impossible for anyone — including ourselves — to be wrong without knowing it."*

**About Us.** 3BEP Labs is currently a one-person project — an independent developer with a conviction and the right tools. All code, documentation, and infrastructure were developed with AI assistance as a force multiplier. We believe that the origin of an idea matters less than its verifiability: every line of code is tested, every claim is reproducible, every hash is auditable. That said, one person does not build a revolution alone. If you share the vision that scientific reproducibility deserves better infrastructure, join us.

---
**3BEP Labs** | The Infrastructure of Physical Truth.
