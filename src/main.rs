use amethyst::{
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    input::StringBindings,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

mod game;
mod systems;

use crate::game::Game;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(systems::AsFastAsPossibleSystem, "as_fast_as_possible", &[])
        .with(systems::XPerSecSystem, "x_per_sec", &[])
        .with(systems::TurnDisplaySystem, "turn_display", &[]);

    let assets_dir = app_root.join("assets");
    let mut game = Application::build(assets_dir, Game::default())?
        .with_frame_limit(FrameRateLimitStrategy::Unlimited, 10000)
        .build(game_data)?;
    // let mut game = Application::new(assets_dir, Game::default(), game_data)?;
    game.run();
    Ok(())
}
