use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    input::{InputBundle, StringBindings},
    utils::application_root_dir,
};

extern crate breakout_amethyst_sample as breakout;
use breakout::state::start::StartState;
use breakout::bundle::GameBundle;

fn main() -> amethyst::Result<()> {
    // ログの開始
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    // 読み込む設定ファイルの場所
    // リソース置き場であるassetsディレクトリのパス
    let assets_dir = app_root.join("assets");
    // ウィンドウの設定を定義したdisplay.ronのパス
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let game_data= GameDataBuilder::default()
        .with_bundle(GameBundle)?
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    // ディスプレイのプラグイン
                    RenderToWindow::from_config_path(display_config_path)?
                        // 背景色の設定
                        .with_clear([0.97, 0.95, 0.898, 1.0]),
                )
                // 2D描画のプラグイン
                .with_plugin(RenderFlat2D::default())
        )?;

    let mut game = Application::new(assets_dir, StartState::default(), game_data)?;
    game.run();

    Ok(())
}
