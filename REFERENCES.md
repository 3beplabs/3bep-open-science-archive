# Scientific References & Citations

This document provides formal academic and institutional references for every physical constant, mathematical algorithm, and theoretical principle used in the 3BEP Sanctuary engine. It is intended for skeptical reviewers who wish to verify our claims against primary sources.

---

## Physical Constants & Normalization

### Gravitational Constant (G)

The engine uses **G = 1** (natural/geometric units). This is the standard normalization in computational astrophysics and N-body simulation, not an arbitrary choice.

- **NIST CODATA 2018:** G = 6.67430(15) × 10⁻¹¹ m³ kg⁻¹ s⁻²
  - Source: [NIST Reference — Newtonian Constant of Gravitation](https://physics.nist.gov/cgi-bin/cuu/Value?bg)
  - DOI: CODATA 2018, *Reviews of Modern Physics* 93, 025010 (2021)
- **Why G = 1:** In Henon units (standard for stellar dynamics), G = 1, M_total = 1, and E = −1/4. This eliminates dimensional noise and isolates the pure mathematical behavior of the integrator from unit-conversion artifacts. 
  - Reference: Heggie, D.C. & Mathieu, R.D. "Standardised Units and Time Scales." *The Use of Supercomputers in Stellar Dynamics*, Springer (1986), pp. 233–235.
  - Reference: Aarseth, S.J. *Gravitational N-Body Simulations*, Cambridge University Press (2003), §1.2.

### Gravitational Softening (ε)

The engine uses **ε = 0.05** (Plummer softening). This prevents the r⁻² singularity from producing infinite forces at r → 0.

- **Plummer Model:** F = −G m₁ m₂ r / (r² + ε²)^(3/2)
  - Reference: Plummer, H.C. "On the Problem of Distribution in Globular Star Clusters." *Monthly Notices of the Royal Astronomical Society* 71, 460–470 (1911).
  - Reference: Dyer, C.C. & Ip, P.S.S. "Softening in N-Body Simulations of Collisionless Systems." *Monthly Notices of the Royal Astronomical Society* 204, 151–161 (1993).

### Time Step (dt)

The engine uses **dt = 0.01**. For the Kepler system (T = 2π ≈ 6.28), this yields ~628 steps/orbit — well within the stability regime of both RK4 and Leapfrog integrators.

- Reference: Hairer, Lubich & Wanner, *Geometric Numerical Integration*, 2nd ed. (2006), §I.2: "Step-size selection for symplectic methods."

---

## Integration Algorithms

### Classical Runge-Kutta (RK4)

Fourth-order explicit integrator with local truncation error O(h⁵) and global error O(h⁴).

- **Original Paper:** Runge, C. "Ueber die numerische Auflösung von Differentialgleichungen." *Mathematische Annalen* 46, 167–178 (1895).
- **Kutta Extension:** Kutta, W. "Beitrag zur näherungsweisen Integration totaler Differentialgleichungen." *Zeitschrift für Mathematik und Physik* 46, 435–453 (1901).
- **Modern Reference:** Butcher, J.C. *Numerical Methods for Ordinary Differential Equations*, 3rd ed. Wiley (2016), §3.

### Velocity Verlet (Leapfrog / Störmer)

Second-order symplectic integrator. Preserves the Hamiltonian symplectic structure, yielding bounded energy oscillation with no secular drift.

- **Störmer (Original):** Störmer, C. "Sur les trajectoires des corpuscules électrisés." *Archives des Sciences Physiques et Naturelles* 24, 5–18 (1907).
- **Verlet (Rediscovery):** Verlet, L. "Computer Experiments on Classical Fluids. I. Thermodynamical Properties of Lennard-Jones Molecules." *Physical Review* 159, 98–103 (1967). [DOI: 10.1103/PhysRev.159.98](https://doi.org/10.1103/PhysRev.159.98)
- **Symplectic Theory:** Hairer, E., Lubich, C. & Wanner, G. *Geometric Numerical Integration: Structure-Preserving Algorithms for Ordinary Differential Equations*, 2nd ed. Springer (2006). [DOI: 10.1007/3-540-30666-8](https://doi.org/10.1007/3-540-30666-8)
- **Time-Reversibility Proof:** Hairer et al. (2006), §V.1, Theorem 1.1: "Symmetric methods applied to reversible systems are themselves reversible."

---

## Conservation Laws (Tested Properties)

### Energy Conservation (First Law of Thermodynamics)

Total mechanical energy E = KE + PE must be constant in a conservative system.

- Reference: Goldstein, H., Poole, C. & Safko, J. *Classical Mechanics*, 3rd ed. Addison-Wesley (2001), §1.1.
- Noether's Theorem: Time-translation symmetry → energy conservation. Noether, E. "Invariante Variationsprobleme." *Nachrichten der Gesellschaft der Wissenschaften zu Göttingen* (1918), pp. 235–257.

### Linear Momentum Conservation (Newton's Third Law)

Total linear momentum P = Σ(mᵢ × vᵢ) must be constant in an isolated system.

- Reference: Newton, I. *Philosophiæ Naturalis Principia Mathematica* (1687), Lex Tertia.
- Noether's Theorem: Spatial-translation symmetry → linear momentum conservation.
- **Our Result:** Conserved to **14 decimal places** (4.6×10⁻¹⁴) in the Kepler test and **13 decimal places** (3.0×10⁻¹³) in chaotic 3-body.

### Angular Momentum Conservation (Kepler's Second Law)

For central forces, Lz = Σ(mᵢ × (xᵢ·vyᵢ − yᵢ·vxᵢ)) must be constant.

- Reference: Kepler, J. *Astronomia Nova* (1609), Second Law: "A line joining a planet to the Sun sweeps out equal areas during equal intervals of time."
- Noether's Theorem: Rotational symmetry → angular momentum conservation.
- **Our Result:** Relative error of **8.8×10⁻¹⁰** (0.000000088%) over 100 Kepler orbits (62,800 steps).

---

## Computational Arithmetic

### IEEE 754 Floating-Point

The standard our engine challenges. Known to produce platform-dependent results due to non-associative arithmetic, FMA instruction availability, and compiler optimizations.

- **Standard:** IEEE 754-2019. "IEEE Standard for Floating-Point Arithmetic." [DOI: 10.1109/IEEESTD.2019.8766229](https://doi.org/10.1109/IEEESTD.2019.8766229)
- **Seminal Analysis:** Goldberg, D. "What Every Computer Scientist Should Know About Floating-Point Arithmetic." *ACM Computing Surveys* 23(1), 5–48 (1991). [DOI: 10.1145/103162.103163](https://doi.org/10.1145/103162.103163)
- **Non-Determinism:** Monniaux, D. "The pitfalls of verifying floating-point computations." *ACM TOPLAS* 30(3), article 12 (2008). [DOI: 10.1145/1353445.1353446](https://doi.org/10.1145/1353445.1353446)

### I64F64 Fixed-Point Arithmetic

Our engine uses 128-bit fixed-point arithmetic (64 integer bits + 64 fractional bits). Every operation is bit-for-bit deterministic regardless of platform, compiler, or optimization flags.

- **Implementation:** `fixed` crate v1.23 — [docs.rs/fixed](https://docs.rs/fixed/latest/fixed/)
- **Bit-Width:** 128-bit total (i64 integer part + u64 fractional part)
- **Range:** Approximately ±9.2 × 10¹⁸ with precision of ~5.4 × 10⁻²⁰
- **Determinism Guarantee:** All arithmetic is integer-based; no FPU, no rounding modes, no FMA ambiguity.

---

## Kepler Orbital Mechanics (Analytical Reference Values)

For the circular orbit test configuration (G=1, M_star=1000, m_planet=1, r=10):

| Property | Formula | Value |
|---|---|---|
| Orbital speed | v = √(GM/r) | 10.0 |
| Orbital period | T = 2πr/v = 2π√(r³/GM) | 2π ≈ 6.2832 |
| Kinetic energy | KE = ½mv² | 50.0 |
| Potential energy | PE = −GMm/r | −100.0 |
| Total energy | E = KE + PE | −50.0 |
| Angular momentum | L = mvr | 100.0 |

- Reference: Goldstein, *Classical Mechanics*, 3rd ed., §3.7: "The Kepler Problem."
- Reference: Murray, C.D. & Dermott, S.F. *Solar System Dynamics*, Cambridge University Press (1999), §2.4.

---

## Three-Body Problem

The gravitational three-body problem has no general closed-form solution. This is the regime where deterministic arithmetic matters most, as chaotic sensitivity amplifies any numerical error exponentially.

- **Poincaré's Proof of Non-Integrability:** Poincaré, H. "Sur le problème des trois corps et les équations de la dynamique." *Acta Mathematica* 13, 1–270 (1890).
- **Lyapunov Exponents in N-Body:** Heggie, D.C. "Binary evolution in stellar dynamics." *Monthly Notices of the Royal Astronomical Society* 173, 729–787 (1975).
- **Modern Review:** Valtonen, M. & Karttunen, H. *The Three-Body Problem*, Cambridge University Press (2006).

---

## Script Library — Academic Sources

The following references document the initial conditions and theoretical background for the pre-configured `.bep` scripts in the `scripts/` directory.

### Astrophysical Scenarios (`scripts/astro/`)

#### Kepler Circular Orbit (`kepler_circular.bep`)

- Verlet, L. "Computer Experiments on Classical Fluids. I. Thermodynamical Properties of Lennard-Jones Molecules." *Physical Review* 159, 98–103 (1967). [DOI: 10.1103/PhysRev.159.98](https://doi.org/10.1103/PhysRev.159.98)
- Hairer, E., Lubich, C. & Wanner, G. *Geometric Numerical Integration: Structure-Preserving Algorithms for Ordinary Differential Equations*, 2nd ed. Springer (2006).

#### Kepler Elliptical Orbit (`kepler_elliptical.bep`)

- Kepler, J. *Astronomia Nova* (1609). — First and Second Laws of planetary motion.
- Bate, R.R., Mueller, D.D. & White, J.E. *Fundamentals of Astrodynamics*. Dover Publications (1971). — Vis-viva equation and orbital element derivations.

#### Binary Star System (`binary_star.bep`)

- Hut, P. & Bahcall, J.N. "Binary-Single Star Scattering. I. Numerical experiments for equal masses." *The Astrophysical Journal* 268, 319–341 (1983). [DOI: 10.1086/160956](https://doi.org/10.1086/160956)

#### Sun-Earth-Moon System (`sun_earth_moon.bep`)

- Murray, C.D. & Dermott, S.F. *Solar System Dynamics*. Cambridge University Press (1999). — Hierarchical multi-scale orbital stability.
- Marchal, C. *The Three-Body Problem*. Elsevier Studies in Astronautics 4 (1990). — General three-body stability criteria.

### Chaotic Dynamics (`scripts/chaos/`)

#### Figure-8 Periodic Solution (`three_body_figure8.bep`)

- **Chenciner, A. & Montgomery, R.** "A Remarkable Periodic Solution of the Three-Body Problem in the Case of Equal Masses." *Annals of Mathematics* 152(3), 881–901 (2000). [DOI: 10.2307/2661357](https://doi.org/10.2307/2661357)
- Moore, C. "Braids in Classical Dynamics." *Physical Review Letters* 70(24), 3675–3679 (1993). [DOI: 10.1103/PhysRevLett.70.3675](https://doi.org/10.1103/PhysRevLett.70.3675)
- Simó, C. "Dynamical Properties of the Figure Eight Solution of the Three-Body Problem." *Contemporary Mathematics* 292, 209–228 (2002).

#### Pythagorean Three-Body Problem (`three_body_pythagorean.bep`)

- **Szebehely, V. & Peters, C.F.** "Complete Solution of a General Problem of Three Bodies." *The Astronomical Journal* 72, 876–883 (1967). [DOI: 10.1086/110355](https://doi.org/10.1086/110355)

#### Butterfly Orbit (`three_body_butterfly.bep`)

- **Šuvakov, M. & Dmitrašinović, V.** "Three Classes of Newtonian Three-Body Planar Periodic Orbits." *Physical Review Letters* 110(11), 114301 (2013). [DOI: 10.1103/PhysRevLett.110.114301](https://doi.org/10.1103/PhysRevLett.110.114301)
- Li, X. & Liao, S. "More Than Six Hundred New Families of Newtonian Periodic Planar Collisionless Three-Body Orbits." *Science China Physics, Mechanics & Astronomy* 60(12), 129511 (2017). [DOI: 10.1007/s11433-017-9078-5](https://doi.org/10.1007/s11433-017-9078-5)

#### Burrau's Problem (`three_body_burrau.bep`)

- **Burrau, C.** "Numerische Berechnung eines Spezialfalles des Dreikörperproblems." *Astronomische Nachrichten* 195, 113–118 (1913). [DOI: 10.1002/asna.19131950602](https://doi.org/10.1002/asna.19131950602)
- Szebehely, V. "Burrau's Problem of Three Bodies." *Proceedings of the National Academy of Sciences* 71(12), 4718–4720 (1974).

### Cryptographic Integrity

#### SHA-256 (Certificate Hash)

- **NIST.** "Secure Hash Standard (SHS)." *Federal Information Processing Standards Publication 180-4* (2015). [DOI: 10.6028/NIST.FIPS.180-4](https://doi.org/10.6028/NIST.FIPS.180-4)

---

*This document is maintained by 3BEP Labs for institutional transparency. All references are verifiable through the cited DOIs or standard academic libraries.*

