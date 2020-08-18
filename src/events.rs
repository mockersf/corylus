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

use amethyst::ecs::storage::{ReadStorage, WriteStorage};
use amethyst::ecs::Join;
use amethyst::ui::{UiImage, UiText, UiTransform};

use crate::ui_scheme::ButtonComp;

impl<'a> System<'a> for UiEventHandlerSystem {
    type SystemData = (
        Write<'a, EventChannel<UiEvent>>,
        ReadStorage<'a, UiTransform>,
        ReadStorage<'a, ButtonComp>,
        WriteStorage<'a, UiImage>,
    );

    #[instrument(skip(events, transforms, buttons, images), level = "info")]
    fn run(&mut self, (events, transforms, buttons, mut images): Self::SystemData) {
        // Reader id was just initialized above if empty
        for ev in events.read(&mut self.reader_id) {
            event!(
                Level::DEBUG,
                "[SYSTEM] You just interacted with an ui element: {:?}",
                ev
            );
            let target = transforms.get(ev.target);
            if let Some(target) = target {
                for (_, transform, image) in (&buttons, &transforms, &mut images).join() {
                    if transform.id.starts_with(&target.id) {
                        match ev.event_type {
                            amethyst::ui::UiEventType::HoverStart => {
                                *image = UiImage::SolidColor(
                                    crate::ui_scheme::COLOR_BACKGROUND_HIGHLIGHTED,
                                );
                            }
                            amethyst::ui::UiEventType::HoverStop => {
                                *image = UiImage::SolidColor(crate::ui_scheme::COLOR_BACKGROUND);
                            }
                            amethyst::ui::UiEventType::ClickStart => {
                                *image = UiImage::SolidColor(crate::ui_scheme::COLOR_ACTING);
                            }
                            amethyst::ui::UiEventType::ClickStop => {
                                *image = UiImage::SolidColor(crate::ui_scheme::COLOR_BACKGROUND);
                            }
                            _ => (),
                        }
                    }
                }
            }
        }
    }
}
