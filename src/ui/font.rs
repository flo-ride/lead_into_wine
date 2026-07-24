use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct UiFont {
    #[asset(path = "fonts/MedievalSharp-Regular.ttf")]
    pub medieval_sharp_font: Handle<Font>,
}

impl UiFont {
    pub fn text(&self, size: f32) -> TextFont {
        TextFont {
            font: bevy::prelude::FontSource::Handle(self.medieval_sharp_font.clone()),
            font_size: bevy::prelude::FontSize::Px(size),
            ..default()
        }
    }
}
