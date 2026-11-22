//! `Game::draw_item` head: runs the port's `on_item_draw` over the drawn
//! stack (upstream `GuiGraphicsExtractorMixin`/`AbstractContainerScreen`
//! `item(...)` injection). `true` cancels the engine draw (the port drew the
//! stack itself during a swap animation).
fn helper(game: &mut Game, batch: &mut HudBatch, item: &ItemStack, x: i32, y: i32) -> bool {
    use super::smoothswapping::config::ConfigManager;
    use super::smoothswapping::gui_graphics_extractor::{GraphicsHost, on_item_draw};
    use super::smoothswapping::item::ItemStack as PortStack;

    if super::smoothswapping_glue::in_reentry()
        || !ConfigManager::get_config().get_toggle_mod()
    {
        return false;
    }

    struct Host<'g> {
        game: &'g Game,
    }
    impl GraphicsHost for Host<'_> {
        fn is_config_screen(&self) -> bool {
            false // TODO-WIRE: no config-screen read on the engine yet.
        }
        fn get_game_time_delta_ticks(&self) -> f32 {
            0.0 // TODO-WIRE: no delta-tracker read on the engine yet.
        }
        fn push_matrix(&self) {} // TODO-WIRE: the HudBatch pose.
        fn translate(&self, _x: f32, _y: f32) {} // TODO-WIRE: the HudBatch pose.
        fn pop_matrix(&self) {} // TODO-WIRE: the HudBatch pose.
        fn m20(&self) -> f32 {
            0.0 // TODO-WIRE: the hotbar x-offset read.
        }
        fn item(&self, stack: &PortStack, x: i32, y: i32) {
            let Some(engine_id) = super::smoothswapping_glue::engine_item_for(stack.item_id)
            else {
                return;
            };
            let batch_ptr = super::smoothswapping_glue::draw_batch();
            if batch_ptr.is_null() {
                return;
            }
            let engine_stack = ItemStack {
                item: engine_id.into(),
                count: stack.count.max(0) as u32,
                damage: 0,
                extra: None,
            };
            let batch = unsafe { &mut *(batch_ptr as *mut HudBatch) };
            let game = unsafe { &mut *(self.game as *const Game as *mut Game) };
            super::smoothswapping_glue::begin_reentry();
            game.draw_item(batch, &engine_stack, x, y);
            super::smoothswapping_glue::end_reentry();
        }
        fn item_decorations(
            &self,
            _stack: &PortStack,
            _x: i32,
            _y: i32,
            _count_override: Option<&str>,
        ) {
            // TODO-WIRE: `draw_item_decorations` reads not mapped yet; the
            // count text of an arriving swap stays the item's own.
        }
        fn handler_present(&self) -> bool {
            self.game.remote.is_some()
        }
        fn handler_items(&self) -> Vec<PortStack> {
            Vec::new() // TODO-WIRE: the open menu's stack read.
        }
        fn handler_carried(&self) -> PortStack {
            match &self.game.carried {
                Some(c) => PortStack::new(
                    super::smoothswapping_glue::bridge_item(&c.item, c.count as i32),
                    c.count as i32,
                ),
                None => PortStack::new(0, 0),
            }
        }
    }

    let port_stack = PortStack::new(
        super::smoothswapping_glue::bridge_item(&item.item, item.count as i32),
        item.count as i32,
    );
    let host = Host { game };
    super::smoothswapping_glue::set_draw_batch(batch as *mut HudBatch as *mut ());
    super::smoothswapping_glue::begin_reentry();
    let handled = on_item_draw(&port_stack, x, y, &host);
    super::smoothswapping_glue::end_reentry();
    super::smoothswapping_glue::clear_draw_batch();
    handled
}
