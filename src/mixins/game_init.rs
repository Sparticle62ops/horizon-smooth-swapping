//! `Game::new` head: runs the port's client init once, 1:1 with the upstream
//! `fabric.fabric_api` client `ClientLifecycleEvents.CLIENT_STARTED` hook
//! (`SmoothSwappingMod.client()` init).
fn helper() {
    super::smoothswapping_glue::init_client();
}
