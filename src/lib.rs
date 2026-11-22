//! Smooth Swapping for Horizon: the mod crate wiring.
//!
//! The 1:1 port's logic lives in `src/runtime.rs`, which the installer
//! splices into the game (`[[native.module]]`); the engine injection points
//! are wired through the `[[native.mixin]]` rows in `horizon-mod.toml`
//! (`src/mixins/`). This crate is the mod's own API surface: it keeps the
//! port's tests building (`cargo test`) and carries the `#[mod_api]` impl
//! the manifest's `api = "0.2"` requires. The port's config stays its own
//! `config/smoothswapping.json` (`serde_json`, as upstream's Gson), so
//! `serde_json` is the game build's dependency, not this crate's.
#[derive(Default)]
struct Mod;

#[mod_api::horizon_mod]
impl mod_api::Hooks for Mod {}

#[cfg(test)]
mod runtime;
