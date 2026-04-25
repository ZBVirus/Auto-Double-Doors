use pumpkin_plugin_api::{Context, Plugin, PluginMetadata, events::EventPriority};
use tracing::*;
mod door_listener;

struct LePlugin;
impl Plugin for LePlugin {
    fn new() -> Self {
        LePlugin
    }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "Auto Double Doors".into(),
            version: "0.0.1".into(),
            authors: vec!["ZBVirus".into()],
            description: "A plugin that automatically opens both double doors when one opens. It works when closing too! :D".into(),
            dependencies: vec![],
        }
    }

    fn on_load(&mut self, _context: Context) -> pumpkin_plugin_api::Result<()> {

        _context.register_event_handler(door_listener::DoorOpenHandler, EventPriority::Normal, false)?;
        info!("Auto Double Doors plugin loaded!");
        Ok(())
    }

    fn on_unload(&mut self, _context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("Auto Double Doors plugin unloaded. Goodbye!");
        Ok(())
    }
}

pumpkin_plugin_api::register_plugin!(LePlugin);
