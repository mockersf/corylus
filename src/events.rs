use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{System, SystemData, Write},
    shrev::{EventChannel, ReaderId},
    ui::UiEvent,
};

use tracing::{event, instrument, Level};

/// This shows how to handle UI events. This is the same as in the 'ui' example.
#[derive(SystemDesc, Debug)]
#[system_desc(name(UiEventHandlerSystemDesc))]
pub struct UiEventHandlerSystem {
    #[system_desc(event_channel_reader)]
    reader_id: ReaderId<UiEvent>,
}

impl UiEventHandlerSystem {
    pub fn new(reader_id: ReaderId<UiEvent>) -> Self {
        Self { reader_id }
    }
}

impl<'a> System<'a> for UiEventHandlerSystem {
    type SystemData = Write<'a, EventChannel<UiEvent>>;

    #[instrument(skip(events), level = "info")]
    fn run(&mut self, events: Self::SystemData) {
        // Reader id was just initialized above if empty
        for ev in events.read(&mut self.reader_id) {
            event!(
                Level::INFO,
                "[SYSTEM] You just interacted with an ui element: {:?}",
                ev
            );
        }
    }
}
