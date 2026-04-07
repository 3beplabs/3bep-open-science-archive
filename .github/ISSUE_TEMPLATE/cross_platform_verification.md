---
name: Cross-Platform Verification
about: Submit your platform's test results to the community verification table
title: "[VERIFY] Platform: <CPU> / <OS>"
labels: verification, cross-platform
assignees: ''
---

## Cross-Platform Verification Report

**Please fill in all fields. This helps us maintain the verification table.**

### Platform

- **OS:** <!-- e.g., Ubuntu 24.04, Windows 11 23H2, macOS 15.1 -->
- **CPU:** <!-- e.g., AMD Ryzen 9 7950X, Apple M3 Pro, Intel i9-13900K -->
- **Architecture:** <!-- e.g., x86_64, aarch64 -->
- **RAM:** <!-- e.g., 32 GB -->

### Toolchain

- **Rust version:** <!-- Output of: rustc --version --verbose -->
- **Cargo version:** <!-- Output of: cargo --version -->

### Test Results

- **Total tests passed:** <!-- e.g., 30/30 -->
- **Any failures?** <!-- Yes/No — if yes, paste full error below -->
- **Commit tested:** <!-- Output of: git rev-parse HEAD -->

<details>
<summary>Full test output (click to expand)</summary>

```
<!-- Paste the output of: cargo test 2>&1 -->
```

</details>

### SHA-256 Hash Verification

Run the following and report the State Hash:

```bash
cd cli_3bep
cargo run --release -- validate ../scripts/astro/kepler_circular.bep
```

- **kepler_circular.bep hash:** <!-- 64-char hex string -->
- **Matches reference?** <!-- Yes/No — Reference: ebd040aebc191afbf4edfef920d5a48589b7b8009b60a76b4f20eaff42359b29 -->

### Additional Notes

<!-- Any observations, warnings, or anomalies -->

### Checklist

- [ ] All 30 tests passed
- [ ] Zero compiler warnings (`cargo build --release`)
- [ ] SHA-256 hash matches reference
- [ ] No local modifications to source code
- [ ] Platform info is complete and accurate
