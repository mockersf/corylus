use amethyst::{
    ecs::prelude::Entity,
    input::{is_close_requested, is_key_down},
    prelude::*,
    ui::{
        Anchor, ToNativeWidget, UiCreator, UiEvent, UiEventType, UiFinder, UiImagePrefab,
        UiTransformData, UiWidget,
    },
    winit::VirtualKeyCode,
};
use serde::Deserialize;

use tracing::{event, instrument, Level};

use crate::color_scheme::{
    Color, COLOR_ACTING, COLOR_BACKGROUND, COLOR_BACKGROUND_HIGHLIGHTED, COLOR_BORDER,
    COLOR_TEXT_LIGHT,
};
use crate::{about::AboutScreen, game::Game};

const BUTTON_START: &str = "start";
const BUTTON_LOAD: &str = "load";
const BUTTON_OPTIONS: &str = "options";
const BUTTON_ABOUT: &str = "about";

#[derive(Default, Debug)]
pub struct MainMenu {
    ui_root: Option<Entity>,
    button_start: Option<Entity>,
    button_load: Option<Entity>,
    button_options: Option<Entity>,
    button_about: Option<Entity>,
}

fn create_button(id: &str, text: &str, width: f32, height: f32, border: f32) -> UiWidget<Menu> {
    UiWidget::Container::<Menu> {
        transform: UiTransformData::default()
            .with_size(width, height)
            .with_anchor(Anchor::Middle),
        background: Some(UiImagePrefab(COLOR_BORDER.as_solid_color())),
        children: vec![UiWidget::Button::<Menu> {
            transform: UiTransformData::default()
                .with_size(width - border, height - border)
                .with_anchor(Anchor::Middle)
                .with_id(id),
            button: amethyst::ui::UiButtonData {
                id: None,
                text: text.to_string(),
                font_size: 90.,
                normal_text_color: COLOR_TEXT_LIGHT,
                font: Some(amethyst::assets::AssetPrefab::File(
                    "font/mandrill.ttf".to_string(),
                    Box::new(amethyst::ui::TtfFormat),
                )),
                normal_image: Some(COLOR_BACKGROUND.as_solid_color()),
                hover_image: Some(COLOR_BACKGROUND_HIGHLIGHTED.as_solid_color()),
                hover_text_color: None,
                press_image: Some(COLOR_ACTING.as_solid_color()),
                press_text_color: None,
                hover_sound: None,
                press_sound: None,
                release_sound: None,
            },
        }],
    }
}

#[derive(Clone, Deserialize, Debug)]
struct Menu {
    button_width: f32,
    button_height: f32,
    button_spacing: f32,
    button_border: f32,
}

impl ToNativeWidget for Menu {
    type PrefabData = ();

    fn to_native_widget(self, _: ()) -> (UiWidget<Menu>, Self::PrefabData) {
        let buttons = vec![
            create_button(
                BUTTON_START,
                "Start Game",
                self.button_width,
                self.button_height,
                self.button_border,
            ),
            create_button(
                BUTTON_LOAD,
                "Load Game",
                self.button_width,
                self.button_height,
                self.button_border,
            ),
            create_button(
                BUTTON_OPTIONS,
                "Options",
                self.button_width,
                self.button_height,
                self.button_border,
            ),
            create_button(
                BUTTON_ABOUT,
                "About",
                self.button_width,
                self.button_height,
                self.button_border,
            ),
        ];
        let button_count = buttons.len();
        let widget = UiWidget::Container {
            background: None,
            transform: Default::default(),
            children: buttons
                .into_iter()
                .enumerate()
                .map(|(i, button)| UiWidget::Container {
                    transform: UiTransformData::default().with_position(
                        0.,
                        ((button_count as f32) / 2. - 0.5) * self.button_height
                            - (i as f32) * (self.button_height + self.button_spacing),
                        0.,
                    ),
                    background: None,
                    children: vec![button],
                })
                .collect(),
        };
        (widget, ())
    }
}

impl SimpleState for MainMenu {
    #[instrument(skip(data), level = "info")]
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.ui_root =
            Some(world.exec(|mut creator: UiCreator<'_, Menu>| creator.create("ui/menu.ron", ())));
    }

    #[instrument(skip(state_data), level = "info")]
    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // only search for buttons if they have not been found yet
        let StateData { world, .. } = state_data;

        if self.button_start.is_none()
            || self.button_load.is_none()
            || self.button_options.is_none()
            || self.button_about.is_none()
        {
            world.exec(|ui_finder: UiFinder<'_>| {
                self.button_start = ui_finder.find(&format!("{}", BUTTON_START));
                self.button_load = ui_finder.find(&format!("{}", BUTTON_LOAD));
                self.button_options = ui_finder.find(&format!("{}", BUTTON_OPTIONS));
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
                if Some(target) == self.button_load || Some(target) == self.button_options {
                    event!(
                        Level::INFO,
                        "This Buttons functionality is not yet implemented!"
                    );
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
        self.button_load = None;
        self.button_options = None;
        self.button_about = None;
    }
}
