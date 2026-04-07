# Preprint Archaeology

This directory contains deterministic I64F64 audits of arXiv preprints. Each subfolder is a self-contained audit package with:

| File | Purpose |
|------|---------|
| `audit_report.md` | Formal analysis and findings |
| `*.bep` | Input scripts for the 3BEP Sanctuary engine |
| `*_trajectory.csv` | Full trajectory data (position, velocity per step) |
| `*_results.csv` | Final state of all bodies |
| `*_certificate.svg` | SHA-256 cryptographic reproducibility seal |
| `f64_divergence_evidence.txt` | IEEE 754 vs I64F64 divergence log |

## How to Verify

```bash
cd cli_3bep
cargo run --release -- validate ../preprint_archaeology/<arXiv_ID>/<script>.bep --trajectory --certificate --export csv
```

The output SHA-256 hash **must match** the hash in the SVG certificate. Any discrepancy proves arithmetic non-determinism in your environment.

## Audited Papers

| arXiv ID | Topic | Key Finding |
|----------|-------|-------------|
| [2603.24675](arXiv_2603_24675/) | Lyapunov exponents in N-body dynamics | f64 diverges from I64F64 at step 507 of 5,000 |
