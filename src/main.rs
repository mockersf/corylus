use amethyst::{
    audio::AudioBundle,
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::{application_root_dir, fps_counter::FpsCounterBundle},
};

use tracing::{event, Level};
use tracing_subscriber;

mod color_scheme;
mod credits;
mod events;
mod game;
mod menu;
mod pause;
mod splash;

pub fn main() -> amethyst::Result<()> {
    let _subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_env_filter("info,corylus=debug,gfx_backend_metal=error")
        .init();

    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let display_config_path = app_root.join("config/display.ron");
    let assets_dir = app_root.join("assets");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(AudioBundle::default())?
        .with_system_desc(
            crate::events::UiEventHandlerSystemDesc::default(),
            "ui_event_handler",
            &[],
        )
        // Necessary for the FPS counter in the upper left corner to work.
        // (simply uncommenting will fail at runtime, since the resource is expected to exist, you
        // need to uncomment line 107-114 in game.rs for it to still work)
        .with_bundle(FpsCounterBundle)?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.005, 0.005, 0.005, 1.0]),
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderFlat2D::default()),
        )?;

    let mut game = Application::new(
        assets_dir,
        crate::splash::SplashScreen::default(),
        game_data,
    )?;
    event!(Level::INFO, "Starting...");
    game.run();

    Ok(())
}
