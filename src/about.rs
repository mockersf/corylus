use amethyst::{
    ecs::prelude::Entity,
    input::{is_close_requested, is_key_down},
    prelude::*,
    ui::{
        Anchor, ToNativeWidget, UiCreator, UiEvent, UiEventType, UiFinder, UiTransformData,
        UiWidget,
    },
    winit::VirtualKeyCode,
};

use serde::Deserialize;
use tracing::{event, instrument, Level};

use crate::color_scheme::*;
use crate::menu::MainMenu;

#[derive(Debug, Default)]
pub struct AboutScreen {
    ui_handle: Option<Entity>,
    twitter_link: Option<Entity>,
}

#[derive(Clone, Deserialize, Debug)]
struct About {
    version: String,
}

impl ToNativeWidget for About {
    type PrefabData = ();

    fn to_native_widget(self, _: ()) -> (UiWidget<About>, Self::PrefabData) {
        let widget = UiWidget::Container {
            background: None,
            transform: UiTransformData::default()
                .with_size(20., 20.)
                .with_anchor(Anchor::Middle)
                .with_stretch(amethyst::ui::Stretch::XY {
                    x_margin: 0.,
                    y_margin: 0.,
                    keep_aspect_ratio: false,
                }),
            children: vec![
                UiWidget::Label {
                    transform: UiTransformData::default()
                        .with_size(480., 720.)
                        .with_anchor(Anchor::Middle),
                    text: amethyst::ui::UiTextData {
                        text: format!(
                            r###"
Corylus {}

by François Mockers
"###,
                            self.version
                        ),
                        font: Some(amethyst::assets::AssetPrefab::File(
                            "font/mandrill.ttf".to_string(),
                            Box::new(amethyst::ui::TtfFormat),
                        )),
                        font_size: 60.,
                        color: COLOR_3,
                        password: false,
                        align: Some(Anchor::Middle),
                        line_mode: Some(amethyst::ui::LineMode::Wrap),
                        editable: None,
                    },
                },
                UiWidget::Container {
                    transform: UiTransformData::default()
                        .with_size(700., 150.)
                        .with_anchor(Anchor::BottomRight),
                    background: None,
                    children: vec![UiWidget::Label {
                        transform: UiTransformData::default()
                            .with_size(600., 70.)
                            .with_anchor(Anchor::TopLeft)
                            .with_id("twitter-link"),
                        text: amethyst::ui::UiTextData {
                            text: "Find me on twitter: @FrancoisMockers".to_string(),
                            font: Some(amethyst::assets::AssetPrefab::File(
                                "font/mandrill.ttf".to_string(),
                                Box::new(amethyst::ui::TtfFormat),
                            )),
                            font_size: 40.,
                            color: COLOR_TEXT_LIGHT,
                            password: false,
                            align: Some(Anchor::Middle),
                            line_mode: Some(amethyst::ui::LineMode::Wrap),
                            editable: None,
                        },
                    }],
                },
            ],
        };
        (widget, ())
    }
}

impl SimpleState for AboutScreen {
    #[instrument(skip(data), level = "info")]
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.ui_handle = Some(
            world.exec(|mut creator: UiCreator<'_, About>| creator.create("ui/about.ron", ())),
        );
    }

    #[instrument(skip(state_data), level = "info")]
    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // only search for buttons if they have not been found yet
        let StateData { world, .. } = state_data;

        if self.twitter_link.is_none() {
            world.exec(|ui_finder: UiFinder<'_>| {
                self.twitter_link = ui_finder.find("twitter-link");
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
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.twitter_link {
                    event!(Level::INFO, "Opening Twitter");
                    if let Err(err) = webbrowser::open("https://twitter.com/FrancoisMockers") {
                        event!(Level::WARN, "Error opening browser: {}", err);
                    }
                    Trans::None
                } else {
                    event!(Level::INFO, "Switching to MainMenu!");
                    Trans::Switch(Box::new(MainMenu::default()))
                }
            }
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    event!(Level::INFO, "Quitting Application!");
                    Trans::Quit
                } else if is_key_down(&event, VirtualKeyCode::Escape)
                // || is_mouse_button_down(&event, MouseButton::Left)
                {
                    event!(Level::INFO, "Switching to MainMenu!");
                    Trans::Switch(Box::new(MainMenu::default()))
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
                .expect("Failed to remove CreditScreen");
        }

        self.ui_handle = None;
    }
}
