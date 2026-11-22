//! Engine adapters for the smoothswapping splice (`smoothswapping_glue`,
//! spliced beside `smoothswapping` in `game.rs` by a second `[[native.module]]`
//! row). Bridges the engine's string item ids onto the port's `u32` ids and
//! carries the draw re-entry plumbing. Everything else (stack conversion,
//! host impls) lives beside the injection sites in `src/mixins/`, where the
//! engine types are in scope.
//!
//! Mappings the engine does not expose where this splice can reach them are
//! marked `TODO-WIRE`; see the README and the linked horizon issue.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

struct IdBridge {
    fwd: HashMap<String, u32>,
    rev: HashMap<u32, String>,
    next: u32,
}

static BRIDGE: OnceLock<Mutex<IdBridge>> = OnceLock::new();

fn bridge() -> &'static Mutex<IdBridge> {
    BRIDGE.get_or_init(|| {
        Mutex::new(IdBridge {
            fwd: HashMap::new(),
            rev: HashMap::new(),
            next: 1,
        })
    })
}

/// The engine's item id is a registry string (`Arc<str>`); the port's is a
/// `u32` handed out on first sight, like the Java `Item` reference identity
/// the upstream hash and `isSameItem` reads use.
pub fn bridge_item(engine_item: &str, _count: i32) -> u32 {
    let mut ids = bridge().lock().unwrap();
    if let Some(&id) = ids.fwd.get(engine_item) {
        return id;
    }
    let id = ids.next;
    ids.next += 1;
    ids.fwd.insert(engine_item.to_string(), id);
    ids.rev.insert(id, engine_item.to_string());
    id
}

/// Reverse: the engine item id string for a port id, or `None` when the id
/// was never bridged (a stack created in the port's own tests).
pub fn engine_item_for(port_id: u32) -> Option<String> {
    bridge().lock().unwrap().rev.get(&port_id).cloned()
}

/// `fabric::on_initialize_client()`.
pub fn init_client() {
    super::smoothswapping::fabric::on_initialize_client();
}

/// Draw re-entry plumbing: the `HudBatch` the slot draw is in, set around the
/// port's draw hooks so the host's redraw can land back on it. Opaque
/// pointer: the mixins cast it to the batch type they hold.
thread_local! {
    static DRAW_BATCH: std::cell::Cell<*mut ()> = const { std::cell::Cell::new(std::ptr::null_mut()) };
    static REENTRY: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

pub fn set_draw_batch(ptr: *mut ()) {
    DRAW_BATCH.with(|c| c.set(ptr));
}

pub fn clear_draw_batch() {
    DRAW_BATCH.with(|c| c.set(std::ptr::null_mut()));
}

pub fn draw_batch() -> *mut () {
    DRAW_BATCH.with(|c| c.get())
}

/// True while the port's draw hooks are running: the head mixins skip
/// themselves so the host's redraw reaches the engine's own draw directly.
pub fn begin_reentry() {
    REENTRY.with(|c| c.set(true));
}

pub fn end_reentry() {
    REENTRY.with(|c| c.set(false));
}

pub fn in_reentry() -> bool {
    REENTRY.with(|c| c.get())
}
