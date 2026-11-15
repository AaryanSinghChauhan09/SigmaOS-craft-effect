# SigmaOS-craft-effect

> **EffectCraft — VFX Compositor** — Creative application module in the 20-module Craft Suite.

Part of the [SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS) ecosystem.  
Node-based VFX compositor and colour grading pipeline (parity: Nuke/Natron)

## Quickstart

```bash
cargo test    # run deterministic AAA unit tests
cargo check   # verify compilation
```

## Component Role

This crate implements the `effect` subsystem for SigmaOS:
- Fully real Rust implementation (no stubs)
- Fail-fast validation with typed errors
- Deterministic unit tests (Arrange-Act-Assert pattern)
- Single Responsibility, Modular, Zero-Copy where applicable

## API

```rust
use sigmaos_craft_effect::*;
// See src/lib.rs for full public API
```

## Documentation

Full documentation is in the [SigmaOS GitHub Wiki → Craft-Creative-Suite](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Craft-Creative-Suite).

## License

MIT
