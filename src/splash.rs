use amethyst::{
    ecs::prelude::Entity,
    input::{is_close_requested, is_key_down, is_mouse_button_down},
    prelude::*,
    ui::UiCreator,
    winit::{MouseButton, VirtualKeyCode},
};

use tracing::{event, instrument, Level};

#[derive(Default, Debug)]
pub struct SplashScreen {
    ui_handle: Option<Entity>,
    frame_displayed: u32,
}

impl SimpleState for SplashScreen {
    #[instrument(skip(data), level = "info")]
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.ui_handle =
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/splash.ron", ())));
    }

    #[instrument(skip(_data), level = "info")]
    fn fixed_update(&mut self, _data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        self.frame_displayed += 1;
        // method is called every 1/60th seconds, 60 should be 1 seconds
        if self.frame_displayed > 60 {
            event!(Level::INFO, "Switching to MainMenu!");
            Trans::Switch(Box::new(crate::menu::MainMenu::default()))
        } else {
            Trans::None
        }
    }

    #[instrument(skip(_data, event), level = "info")]
    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    event!(Level::INFO, "[Trans::Quit] Quitting Application!");
                    Trans::Quit
                } else if is_mouse_button_down(&event, MouseButton::Left) {
                    event!(Level::INFO, "Switching to MainMenu!");
                    Trans::Switch(Box::new(crate::menu::MainMenu::default()))
                } else {
                    Trans::None
                }
            }
            _ => Trans::None,
        }
    }

    #[instrument(skip(data), level = "info")]
    fn on_stop(&mut self, data: StateData<GameData>) {
        if let Some(root_entity) = self.ui_handle {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to remove SplashScreen");
        }

        self.ui_handle = None;
    }
}
