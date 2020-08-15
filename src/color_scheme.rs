use amethyst::ui::UiImageLoadPrefab;

pub const COLOR_BORDER: [f32; 4] = [0.78, 0.933, 0.106, 1.];
pub const COLOR_TEXT_LIGHT: [f32; 4] = [0.282, 0.624, 0.71, 1.];
pub const COLOR_3: [f32; 4] = [0.569, 0.961, 0.678, 1.];
pub const COLOR_ACTING: [f32; 4] = [0.988, 0.624, 0.357, 1.];
pub const COLOR_BACKGROUND_HIGHLIGHTED: [f32; 4] = [0.329, 0.071, 0.094, 1.];
pub const COLOR_BACKGROUND: [f32; 4] = [0.196, 0.043, 0.055, 1.];

pub trait Color {
    fn as_solid_color(&self) -> UiImageLoadPrefab;
}

impl Color for [f32; 4] {
    fn as_solid_color(&self) -> UiImageLoadPrefab {
        UiImageLoadPrefab::SolidColor(self[0], self[1], self[2], self[3])
    }
}
