use pumpkin_plugin_api::{Context, Plugin, PluginMetadata};
use tracing::*;

struct LePlugin;
impl Plugin for LePlugin {
    fn new() -> Self {
        LePlugin
    }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "Auto Double Doors".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            authors: vec!["Bjorn".into()],
            description: "A plugin that automatically opens both double doors when one opens. It works when closing too! :D".into(),
            dependencies: vec![],
        }
    }

    fn on_load(&mut self, _context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("Auto Double Doors plugin loaded!");
        Ok(())
    }

    fn on_unload(&mut self, _context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("Auto Double Doors plugin unloaded. Goodbye!");
        Ok(())
    }
}

pumpkin_plugin_api::register_plugin!(LePlugin);
