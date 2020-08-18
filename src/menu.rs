use amethyst::{
    ecs::prelude::Entity,
    input::{is_close_requested, is_key_down},
    prelude::*,
    ui::{Anchor, UiEvent, UiEventType, UiFinder},
    winit::VirtualKeyCode,
};

use tracing::{event, instrument, Level};

use crate::{about::AboutScreen, game::Game};

const BUTTON_START: &str = "start";
const BUTTON_ABOUT: &str = "about";

#[derive(Default, Debug)]
pub struct MainMenu {
    ui_root: Option<Entity>,
    button_start: Option<Entity>,
    button_about: Option<Entity>,
}

impl SimpleState for MainMenu {
    #[instrument(skip(data), level = "info")]
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let menu = world
            .create_entity()
            .with(amethyst::ui::UiTransform::new(
                format!("menu"),
                Anchor::Middle,
                Anchor::Middle,
                0.,
                0.,
                0.,
                0.,
                0.,
            ))
            .build();

        let buttons = vec![("Start Game", BUTTON_START), ("About", BUTTON_ABOUT)];
        buttons.iter().enumerate().for_each(|(i, (text, id))| {
            crate::ui_scheme::Button::new(text, id).create(
                world,
                menu,
                amethyst::ui::UiTransform::new(
                    format!("{}_container", id),
                    Anchor::Middle,
                    Anchor::Middle,
                    0.,
                    ((buttons.len() as f32) / 2. - 0.5) * 150. - (i as f32) * (150. + 10.),
                    0.,
                    0.,
                    0.,
                ),
            );
        });

        self.ui_root = Some(menu);
    }

    #[instrument(skip(state_data), level = "info")]
    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // only search for buttons if they have not been found yet
        let StateData { world, .. } = state_data;

        if self.button_start.is_none() || self.button_about.is_none() {
            world.exec(|ui_finder: UiFinder<'_>| {
                self.button_start = ui_finder.find(&format!("{}", BUTTON_START));
                self.button_about = ui_finder.find(&format!("{}", BUTTON_ABOUT));
            });
        }

        Trans::None
    }

    #[instrument(skip(_data), level = "info")]
    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    event!(Level::INFO, "Quitting Application!");
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.button_about {
                    event!(Level::INFO, "Switching to AboutScreen!");
                    return Trans::Switch(Box::new(AboutScreen::default()));
                }
                if Some(target) == self.button_start {
                    event!(Level::INFO, "Switching to Game!");
                    return Trans::Switch(Box::new(Game::default()));
                }

                Trans::None
            }
            _ => Trans::None,
        }
    }

    #[instrument(skip(data), level = "info")]
    fn on_stop(&mut self, data: StateData<GameData>) {
        // after destroying the current UI, invalidate references as well (makes things cleaner)
        if let Some(root_entity) = self.ui_root {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to remove MainMenu");
        }

        self.ui_root = None;
        self.button_start = None;
        self.button_about = None;
    }
}
