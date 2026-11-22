# Smooth Swapping for Horizon

Exact 1:1 Rust port of [Smooth Swapping](https://modrinth.com/mod/smooth-swapping)
by Schauweg (branch 26.3/dev @88e2988), LGPL-3.0-only, ported to the
Horizon client as a runtime splice. Carries the upstream license and credit
per the port policy. Ported by sparticle.

## Layout

- `src/runtime.rs` — the 1:1 port: Vec2 (including the Java `=-` negation
  quirk), SwapStacks, the swaps trio behind AnySwap, the SmoothSwapping state
  object (serde_json in place of Gson, snake_case fields, pretty printing),
  SwapUtil, the CatmullRomWidget config set, the config manager, and the
  mixin set. Self-contained; `cargo test` runs the port's tests.
- `src/glue.rs` — the engine adapters: the item-id bridge (engine item ids are
  registry strings, the port's are `u32` handed out on first sight) and the
  draw re-entry plumbing.
- `src/mixins/` — the injection helpers, one per `[[native.mixin]]` row.
- `src/lib.rs` — the mod's API surface (`api = "0.2"`).

## Wiring state (1.0.1)

Wired rows:

- `Game::new` head → `fabric::on_initialize_client()` (the upstream fabric
  client-started hook).
- `Game::draw_item` head, cancellable → `gui_graphics_extractor::on_item_draw`
  (`GuiGraphicsExtractorMixin`'s first injection).
- `Game::draw_slot_item` head, cancellable → `gui_graphics_extractor::on_draw_item_in_slot`.

Pending wiring (upstream mixins with no engine injection point picked yet;
tracked with the linked horizon issue):

- `ServerboundContainerClickPacketMixin` → the click packet send site: the
  helper conversion needs the engine's `item::Hashed` reads (the port's
  `HashedStack` carries item identity + count).
- `AbstractContainerScreenMixin.render` and
  `AbstractRecipeBookScreenMixin.render` → the engine's container/recipe-book
  screen render points.

Degrading mappings (each marked `TODO-WIRE` in the helpers): the pose stack
(push/translate/pop), the tick delta, the hotbar x-offset read, the config
screen check, the open menu's stack list, and the item-decoration count text.
The swap state machine, config, and swap-tracking logic run 1:1; these
mappings gate the visuals that redraw stacks during the animation.

## License

Upstream Smooth Swapping is LGPL-3.0-only, Copyright (c) Schauweg. This
port carries the upstream license and credit per the port policy
(`LICENSE`, headers in the sources).
