use amethyst::{
    assets::{AssetStorage, Loader},
    assets::Handle,
    prelude::*,
    renderer::{SpriteSheet, SpriteRender, ImageFormat, Texture, SpriteSheetFormat},
};

// スプライトの作成
pub fn create_sprite(asset_name: &str, setting_name: &str, number: usize, world: &mut World) -> SpriteRender {
    let sprite_sheet = load_sprite_sheet(asset_name, setting_name, world);

    SpriteRender {
        sprite_sheet: sprite_sheet,
        sprite_number: number,
    }
}

// アトラスの読み込み
fn load_sprite_sheet(asset_name: &str, setting_name: &str, world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle : Handle<Texture> = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            asset_name,
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        setting_name,
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}
