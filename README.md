# parachute

A Rust workspace for bare-metal / OS kernel boot infrastructure, targeting the portal-co ecosystem. The stated package description is "Modern WASM compatibility layer and oS".

Repository: https://github.com/portal-co/parachute

## Workspace layout

```
parachute/
  Cargo.toml                          # virtual workspace root (resolver = "3")
  crates/
    parachute-kernel-boot/            # only crate in the workspace
      Cargo.toml
      src/lib.rs
```

`crates/_` exists as an empty placeholder file; it is not a crate.

## Crates

### `parachute-kernel-boot` (v0.1.0)

A `#![no_std]` library crate. Its current implementation is a stub:

- Defines `BootMem(Mutex<BootMemInternal>)` — a newtype wrapping a `spin::Mutex`.
- `BootMemInternal` is an enum with one variant, gated behind `#[cfg(feature = "limine")]`:
  - `DormantLimine { memmap: &'static limine::request::MemoryMapRequest }` — holds a reference to a Limine boot memory-map request.
- No methods, trait impls, or public functions are implemented yet.
- Conditionally pulls in `extern crate alloc` under the `alloc` feature flag.

**Features**

| Feature | Effect |
|---|---|
| `limine` | Enables the `DormantLimine` boot-memory variant via `dep:limine` |
| `uefi` | Pulls in `dep:uefi`; no code currently uses it |
| `alloc` | Enables `extern crate alloc` |
| `allocator-api` | Implies `alloc`; enables the nightly `allocator_api` feature gate |

**Architecture-gated dependencies** (from workspace):

| Target | Dependency |
|---|---|
| `x86_64` | `x86_64 = "0.15.4"` |
| `aarch64` | `aarch64 = "0.0.14"` |
| `riscv32` or `riscv64` | `riscv = "0.16.0"` |

None of these architecture crates are currently referenced in source.

## Workspace dependencies

Declared in the root `Cargo.toml` but not all are used by existing source:

| Crate | Version | Purpose |
|---|---|---|
| `portal-pc-waffle` | `^0.5` | Portal WASM compatibility layer |
| `pit-core` | `^0.4.2` | Portal interface types (core) |
| `pit-patch` | `^0.4.2` | Portal interface types (patch) |
| `awaiter-trait` | `^0.2.7` | Async awaiter abstraction |
| `bysyncify` | `^0.2.2` | Asyncify/syncify bridge |
| `spin` | `0.10.0` | `no_std` spinlock (used in `parachute-kernel-boot`) |
| `limine` | `0.5.0` | Limine bootloader protocol types |
| `uefi` | `0.36.1` | UEFI protocol types |
| `x86_64` | `0.15.4` | x86_64 hardware abstractions |
| `aarch64` | `0.0.14` | AArch64 hardware abstractions |
| `riscv` | `0.16.0` | RISC-V hardware abstractions |

## License

MPL-2.0

## Status

Early scaffolding. One crate exists with a single source file containing a partially-defined type and no implemented logic. The WASM-layer workspace dependencies (`portal-pc-waffle`, `pit-*`, `awaiter-trait`, `bysyncify`) are declared but not yet wired into any crate.
