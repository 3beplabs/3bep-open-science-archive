# Verification Protocol — Cross-Platform Reproducibility

This document describes how to independently verify the 3BEP Sanctuary engine on your platform and submit your results to the community verification table.

## Why Verify?

The 3BEP engine claims **bit-perfect determinism** across all platforms. This is not a theoretical claim — it has been empirically proven on 3 architectures. But science demands independent verification. Your platform could be number 4, or it could reveal a case we haven't considered.

Both outcomes are valuable.

## Quick Verification (5 minutes)

### Prerequisites
- [Rust](https://rustup.rs/) (any recent stable version)
- Git

### Steps

```bash
# 1. Clone the repository
git clone https://github.com/3beplabs/3bep-open-science-archive.git
cd 3bep-open-science-archive

# 2. Run the full test suite (30 tests, Zero Tolerance Protocol)
cd core_engine
cargo test 2>&1 | tee test_output.txt

# 3. Record your platform info
rustc --version --verbose
```

If all 30 tests pass, your platform produces **bit-identical** results to every other verified platform.

## Full Verification (15 minutes)

For a more thorough verification, run the script library and compare SHA-256 hashes:

```bash
# 4. Run the CLI on a reference script and record the hash
cd ../cli_3bep
cargo run --release -- validate ../scripts/astro/kepler_circular.bep --certificate

# The output includes:
#   State Hash (SHA-256): <64-character hex string>
#
# This hash MUST be identical across all platforms.
# If it differs, you've found a reproducibility issue — please report it.
```

### Reference Hashes (SHA-256)

These are the canonical hashes. Your machine should produce **exactly** these values:

| Script | State Hash (SHA-256) |
|---|---|
| `kepler_circular.bep` | `ebd040aebc191afbf4edfef920d5a48589b7b8009b60a76b4f20eaff42359b29` |
| `three_body_figure8.bep` | `3f944a17823395ee675a246982da67ffa352e6229d758a862f068efe7b12d5b2` |
| `binary_star.bep` | `b8b22a45e4f1c8f63bca392be3a1cb7525b2a2e14b13b24ac551390ddd1daaa1` |
| `three_body_burrau.bep` | `7b2e73d343c326274f07ce2b19ef1e037c7343c356a7b954d720356b6fc1f05c` |

## Submit Your Results

### Option A: GitHub Issue (Preferred)

Open a new Issue using our [Cross-Platform Verification template](https://github.com/3beplabs/3bep-open-science-archive/issues/new?template=cross_platform_verification.md).

### Option B: Manual Report

If you prefer, create an Issue with the following information:

```
## Cross-Platform Verification Report

**Platform:**
- OS: [e.g., Ubuntu 24.04, Windows 11, macOS 15]
- CPU: [e.g., AMD Ryzen 9 7950X, Apple M3 Pro, Intel i9-13900K]
- Architecture: [e.g., x86_64, aarch64]

**Toolchain:**
- Rust version: [output of `rustc --version --verbose`]
- Cargo version: [output of `cargo --version`]

**Results:**
- Tests: [30/30 passed | X failed]
- Test output: [attach or paste]
- Commit hash tested: [output of `git rev-parse HEAD`]

**SHA-256 Hash Verification:**
- kepler_circular.bep: [hash] [MATCH/MISMATCH]
- three_body_figure8.bep: [hash] [MATCH/MISMATCH]
```

## Verified Platforms

| # | CPU | OS | Compiler | Tests | Result | Date |
|---|---|---|---|---|---|---|
| 1 | AMD Ryzen (Zen 3) | Windows 11 | MSVC 19.x | 30/30 | BIT-IDENTICAL | 2026-04-06 |
| 2 | AMD EPYC (Zen 2) | Ubuntu 24.04 | GCC 13.x | 30/30 | BIT-IDENTICAL | 2026-04-06 |
| 3 | Intel i5-6200U (Skylake) | Windows 10 | GCC 13.x | 30/30 | BIT-IDENTICAL | 2026-04-06 |

*Want to be #4? Follow the steps above and submit your results.*

## What Constitutes a Valid Verification?

1. **All 30 tests pass** — No failures, no panics, no skipped tests
2. **SHA-256 hashes match** — The state hash for reference scripts must be identical
3. **No compiler warnings** — `cargo build --release` produces zero warnings
4. **Clean environment** — No local modifications to the source code

## What If Something Fails?

**If a test fails:** Open an Issue immediately with the full error output. This is exactly the kind of finding we need.

**If hashes differ:** This could indicate a platform-specific behavior in the `fixed` crate or a compiler optimization that breaks determinism. This is a critical finding — please report it with full platform details.

**If compilation fails:** Check your Rust version (`rustc --version`). The engine requires Rust 1.70+ for const generics support.

---

*"Every new architecture that produces bit-identical results strengthens the proof. Every one that doesn't reveals something we need to fix. Both outcomes are valuable."*
