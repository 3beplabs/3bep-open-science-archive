# Preprint Archaeology

This directory contains deterministic I64F64 audits of arXiv preprints. Each subfolder is a self-contained audit package with:

| File | Purpose |
|------|---------|
| `audit_report.md` | Formal analysis and findings |
| `*.bep` / `*.json` | Input scripts/configuration for the 3BEP engine |
| `*_trajectory.csv` / `*_correlation_results.csv` | Full data output (trajectory or correlation coefficients) |
| `*_results.csv` / `*_background_distribution.csv` | Final state / statistical distributions |
| `*_certificate.svg` | SHA-256 cryptographic reproducibility seal |
| `f64_divergence_evidence.txt` | IEEE 754 vs I64F64 divergence log |

## How to Verify

```bash
cd cli_3bep
cargo run --release -- validate ../preprint_archaeology/<arXiv_ID>/<script>.bep --trajectory --certificate --export csv
```

The output SHA-256 hash **must match** the hash in the SVG certificate. Any discrepancy proves arithmetic non-determinism in your environment.

## Audited Papers

| Paper ID | Topic | Key Finding |
|----------|-------|-------------|
| [2603.24675](arXiv_2603_24675/) | Lyapunov exponents in N-body dynamics | f64 diverges from I64F64 at step 507 of 5,000 |
| [2604.0039](aiVixra_2604_0039/) | Template-free GW detection via cross-correlation | I64F64 reproduces 6.70 sigma detection; f64 divergence < 1e-13 (negligible for GW significance) |
