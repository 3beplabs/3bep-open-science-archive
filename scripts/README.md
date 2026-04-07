# 3BEP Script Library

Pre-configured simulation scenarios for the 3BEP Sanctuary Engine. Each `.bep` file is a standard JSON with optional academic metadata — ready to run with `3bep validate`.

## Usage

```bash
cd cli_3bep

# Run any script directly
cargo run --release -- validate ../scripts/astro/kepler_circular.bep

# With full trajectory export
cargo run --release -- validate ../scripts/chaos/three_body_figure8.bep --trajectory

# With reproducibility certificate
cargo run --release -- validate ../scripts/astro/kepler_circular.bep --certificate
```

## Categories

### `astro/` — Astrophysical Scenarios

| Script | Description | Integrator | Steps |
|---|---|---|---|
| `kepler_circular.bep` | Two-body circular orbit, 10 orbital periods | Leapfrog | 62,800 |
| `kepler_elliptical.bep` | Elliptical orbit (e=0.5), Vis-Viva validation | RK4 | 50,000 |
| `binary_star.bep` | Equal-mass binary system, symmetric conservation | Leapfrog | 100,000 |
| `sun_earth_moon.bep` | Hierarchical 3-body, orbital stability | Leapfrog | 200,000 |

### `chaos/` — Chaotic Dynamics

| Script | Description | Integrator | Steps |
|---|---|---|---|
| `three_body_figure8.bep` | Figure-8 periodic solution (Chenciner & Montgomery, 2000) | RK4 | 100,000 |
| `three_body_pythagorean.bep` | Pythagorean initial conditions (Szebehely & Peters, 1967) | RK4 | 50,000 |
| `three_body_butterfly.bep` | Butterfly orbit — extreme Lyapunov sensitivity | RK4 | 80,000 |
| `three_body_burrau.bep` | Burrau's problem (1913) — chaotic collapse | RK4 | 30,000 |

## The `.bep` Format

A `.bep` file is valid JSON with an optional `metadata` block for academic context:

```json
{
  "metadata": {
    "title": "Human-readable title",
    "category": "astro | chaos",
    "description": "What this scenario validates",
    "author": "3BEP Labs",
    "references": ["Academic paper citations"],
    "tags": ["searchable", "keywords"],
    "expected_claims": ["energy_conservation", "determinism"]
  },
  "experiment_name": "Machine_Readable_ID",
  "bodies": [...],
  "integrator": "rk4 | leapfrog",
  "dt": "0.001",
  "steps": 100000,
  "export_interval": 100
}
```

The `metadata` field is ignored by the simulation engine — it exists purely for documentation and tooling.

## Contributing

Have a scenario you'd like to see? Open an Issue with your initial conditions and we'll add it to the library.
