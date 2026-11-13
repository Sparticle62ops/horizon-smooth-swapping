# horizon-smooth-swapping (1:1 port)

Exact 1:1 Rust translation of [Smooth Swapping](https://modrinth.com/mod/smooth-swapping)
(Schauweg, LGPL-3.0-only, branch 26.3/dev @88e2988) for Horizon.

Upstream logic carried 1:1 in `mixins/smoothswapping/runtime.rs` as a Horizon
client runtime splice (`crates/horizon-client/src/game.rs`): Vec2 (including
the Java `=-` negation quirk), SwapStacks, the swaps trio (InventorySwap,
ItemToCursorInventorySwap, ItemToItemInventorySwap) behind the AnySwap enum,
the SmoothSwapping state object (Gson serialized config, the AtomicReference +
ReentrantLock cursor-stack critical section collapsed into one Mutex), SwapUtil
(quadrants, i2i/i2c assignment, hasArrived), the CatmullRomWidget config set,
the config manager (serde_json in place of Gson; snake_case field naming and
pretty printing match upstream), and all six mixins (ItemStackAccessor,
SimpleContainerAccessor, ServerboundContainerClickPacketMixin,
AbstractContainerScreenMixin, AbstractRecipeBookScreenMixin,
GuiGraphicsExtractorMixin). The fabric entrypoint `SmoothSwappingFabric` maps
to the carrier's client-init hook; the ModMenu entrypoint joins the
ConfigScreen batch later.

Ported by sparticle.

## License

Upstream Smooth Swapping is LGPL-3.0-only, Copyright (c) Schauweg. This port
carries the upstream license and credit per the port policy.
