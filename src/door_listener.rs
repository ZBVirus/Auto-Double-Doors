use pumpkin_plugin_api::{
    Server,
    events::{EventData, EventHandler, PlayerInteractEvent},
};


pub struct DoorOpenHandler;
impl EventHandler<PlayerInteractEvent> for DoorOpenHandler {
    fn handle<'a>(&'a self, server: Server, event: EventData<PlayerInteractEvent>) -> EventData<PlayerInteractEvent> {
        let player = event.player.get_name();
        let target = event.block.as_str();
        server.broadcast(&format!("{player} has interacted with {target}."));

        event
    }
}
