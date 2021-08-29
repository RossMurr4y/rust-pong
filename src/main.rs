//! Pong Amethyst Tutorial

use amethyst::{
    prelude::*,
    audio::{AudioBundle, DjSystemDesc},
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

mod pong;
mod systems;
mod audio;
use crate::pong::Pong;
use crate::audio::Music;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root
        .join("config")
        .join("display.ron");

    let binding_path = app_root
        .join("config")
        .join("bindings.ron");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(AudioBundle::default())?
        .with_system_desc(
            DjSystemDesc::new(|music: &mut Music| music.music.next()),
            "dj_system",
            &[],
        )
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with(systems::move_balls::MoveBallsSystem, "ball_system", &[])
        .with(systems::bounce::BounceSystem, "collision_system", &["paddle_system", "ball_system"])
        .with(systems::WinnerSystem, "winner_system", &["ball_system"])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.00196, 0.23726, 0.21765, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?;

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, Pong::default(), game_data)?;
    game.run();
    Ok(())
}