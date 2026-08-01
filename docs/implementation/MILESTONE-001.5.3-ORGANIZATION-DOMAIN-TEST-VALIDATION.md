# Milestone 001.5.3 — Organization Domain Test Execution and Refinement

**Status:** Blocked — Environment Limitation  
**Version:** 1.0.0  
**Crate:** `forgeos-organization-domain`  
**Authority:** `docs/implementation/MILESTONE-001.5-ORGANIZATION-DOMAIN.md`

---

# 1. Issue Found

`cargo test --workspace` fails with:

```
error: error calling dlltool 'dlltool.exe': program not found
error: could not compile `getrandom` (lib) due to 1 previous error
```

**Root Cause:** The Windows GNU toolchain (`stable-x86_64-pc-windows-gnu`) is installed but missing the `dlltool.exe` utility required to build import libraries for dependencies like `getrandom`. The `rust-mingw-x86_64-pc-windows-gnu` component is listed as installed, but the actual `dlltool.exe` binary is not present in the toolchain's `bin/` directory.

**Contributing Factor:** The `.cargo/config.toml` file contained an incorrect `[target.x86_64-pc-windows-gnu]` section that pointed to the MSVC linker (`rust-lld.exe`), which would have caused linker failures if the GNU toolchain had been functional. This section was removed to avoid conflicting configurations.

---

# 2. Resolution

## 2.1 Configuration Fix

Removed the erroneous `[target.x86_64-pc-windows-gnu]` section from `implementation/rust/.cargo/config.toml`:

```toml
[host.x86_64-pc-windows-msvc]
linker = "C:\\Users\\ricky\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\bin\\rust-lld.exe"
```

The `[host.x86_64-pc-windows-msvc]` section is retained for potential future MSVC use, but the GNU target no longer has an explicit linker override.

## 2.2 Toolchain Change

| Aspect | Detail |
|---|---|
| **Previous toolchain** | `stable-x86_64-pc-windows-gnu` (default) |
| **New toolchain** | `stable-x86_64-pc-windows-gnu` (unchanged) |
| **Reason** | The GNU toolchain is the only installed option that supports `cargo check` without Visual Studio Build Tools. Switching to MSVC would require `link.exe` (Visual Studio C++ build tools), which is not installed. |
| **Impact** | `cargo check --workspace` passes. `cargo test --workspace` remains blocked due to missing `dlltool.exe` in the GNU toolchain installation. |
| **Validation result** | `cargo check --workspace` ✅ Passed. `cargo test --workspace` ❌ Blocked by missing `dlltool.exe`. |

---

# 3. Test Result

## 3.1 cargo check --workspace

```
Checking forgeos-organization-domain v0.1.0
Checking forgeos-organization-infrastructure v0.1.0
Checking forgeos-create-organization-application v0.1.0
Checking forgeos-desktop-platform v0.1.0
Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.57s
```

**Result:** ✅ Passed — all four workspace members compile without errors.

## 3.2 cargo test --workspace

```
Compiling getrandom v0.4.3
Compiling uuid v1.24.0
error: error calling dlltool 'dlltool.exe': program not found
error: could not compile `getrandom` (lib) due to 1 previous error
```

**Result:** ❌ Blocked — `dlltool.exe` is missing from the GNU toolchain installation.

## 3.3 git diff --check

**Result:** ✅ Passed — no whitespace errors detected.

---

# 4. Remaining Blockers

## 4.1 Primary Blocker: Missing dlltool.exe

`cargo test --workspace` cannot execute because the GNU toolchain's `dlltool.exe` is not present. This is an environment limitation, not a code or dependency issue.

**Resolution Paths:**

1. **Install full MinGW-w64:** Install a complete MinGW-w64 distribution that includes `dlltool.exe` (e.g., via MSYS2 or standalone MinGW-w64 installer). Ensure the `bin/` directory is in `PATH` or configure the linker path in `.cargo/config.toml`.

2. **Install Visual Studio Build Tools:** Install "Build Tools for Visual Studio" with the "Desktop development with C++" workload. This provides `link.exe` (MSVC linker), allowing use of the `stable-x86_64-pc-windows-msvc` toolchain. Then switch the default toolchain back to MSVC.

3. **Use pre-built dependencies:** Configure Cargo to use pre-built `getrandom` artifacts if available, avoiding the need for `dlltool.exe` during test compilation. This may not be feasible for all dependencies.

## 4.2 Secondary: Outdated Baseline Documentation

`docs/implementation/MILESTONE-001-IMPLEMENTATION-BASELINE.md` records "Cargo is not installed" as a validation limitation. This is now outdated — Cargo 1.97.1 is installed and `cargo check` passes. The baseline document should be updated in a future documentation sync.

---

# 5. Test Inventory (Unaffected by Blocker)

The following deterministic unit tests are present and expected to pass once the linker environment is resolved:

| File | Test Count | Coverage |
|---|---|---|
| `value_objects.rs` | 15 | `OrganizationId`, `OrganizationName`, `OrganizationType`, `OrganizationStatus`, `OrganizationVersion` |
| `id_generation.rs` | 2 | `DefaultOrganizationIdGenerator` (distinct IDs, v4 UUID text) |
| `organization.rs` | 7 | Valid creation, invalid name/type rejection, `OrganizationCreated` event recording, event drain, deterministic identity injection, distinct IDs from default generator, exact preservation of supplied name/type |
| **Total** | **24** | |

---

# 6. Stop Condition

This milestone stops here. The blocker is fully documented. No code changes are required to resolve the environment limitation.

**Do not proceed to the Application Layer** until `cargo test --workspace` passes or the environment is fully documented as an accepted constraint.

---

# 7. Next Step

Once a working linker environment is available (full MinGW-w64 or Visual Studio Build Tools):

1. Run `cargo test --workspace` and confirm all 24 deterministic unit tests pass.
2. Address any test failures.
3. Commit the implementation with validated test results.
4. Proceed to the Application layer (`forgeos-create-organization-application`) per MILESTONE-001.5 scope.