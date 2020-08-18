use amethyst::{
    assets::{Asset, AssetStorage, Format, Handle, Loader},
    core::transform::Parent,
    ecs::{prelude::Entity, World},
    prelude::*,
    ui::{Anchor, FontAsset, UiButtonActionRetrigger, UiImage, UiImageLoadPrefab, UiTransform},
};

use tracing::{event, instrument, Level};

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

pub fn load_font(world: &World, name: &str) -> Handle<FontAsset> {
    load(world, name, amethyst::ui::TtfFormat)
}

pub fn load_image(world: &World, name: &str) -> Handle<amethyst::renderer::Texture> {
    load(world, name, amethyst::renderer::ImageFormat::default())
}

fn load<A: Asset<Data = D>, D: 'static + Send + Sync, F: Format<D>>(
    world: &World,
    name: &str,
    format: F,
) -> Handle<A> {
    world.read_resource::<Loader>().load(
        name,
        format,
        (),
        &world.read_resource::<AssetStorage<A>>(),
    )
}

#[derive(Debug)]
pub struct Button {
    pub text: String,
    pub id: String,
    pub width: f32,
    pub height: f32,
    pub border: f32,
    pub font: Option<Handle<FontAsset>>,
}

#[derive(Default)]
pub struct ButtonComp;
impl amethyst::ecs::Component for ButtonComp {
    type Storage = amethyst::ecs::NullStorage<ButtonComp>;
}

impl Button {
    pub fn new(text: &str, id: &str) -> Button {
        Button {
            text: text.to_string(),
            id: id.to_string(),
            width: 800.,
            height: 150.,
            border: 5.,
            font: None,
        }
    }

    #[instrument(skip(world), level = "info")]
    pub fn create(self, world: &mut World, parent: Entity, transform: UiTransform) -> Entity {
        let font = load_font(world, "font/mandrill.ttf");

        let border = UiImage::SolidColor(COLOR_BORDER);
        let border_transform = UiTransform::new(
            format!("{}_border", self.id),
            Anchor::Middle,
            Anchor::Middle,
            0.,
            0.,
            0.,
            self.width,
            self.height,
        );
        let border_overlay_1_transform = UiTransform::new(
            format!("{}_border_overlay_1", self.id),
            Anchor::BottomLeft,
            Anchor::BottomLeft,
            0.,
            0.,
            0.2,
            self.width / 5.,
            self.height / 5.,
        );
        let border_overlay_2_transform = UiTransform::new(
            format!("{}_border_overlay_2", self.id),
            Anchor::TopRight,
            Anchor::TopRight,
            0.,
            0.,
            0.2,
            self.width / 5.,
            self.height / 5.,
        );
        let background = UiImage::SolidColor(COLOR_BACKGROUND);
        let background_transform = UiTransform::new(
            format!("{}_background", self.id),
            Anchor::Middle,
            Anchor::Middle,
            0.,
            0.,
            0.1,
            self.width - self.border,
            self.height - self.border,
        );

        let text = amethyst::ui::UiText::new(font.clone(), self.text, COLOR_TEXT_LIGHT, 70.);
        let text_transform = UiTransform::new(
            format!("{}_text", self.id),
            Anchor::Middle,
            Anchor::Middle,
            0.,
            0.,
            0.2,
            self.width - self.border,
            self.height - self.border,
        );
        let catch_transform = UiTransform::new(
            self.id.to_string(),
            Anchor::Middle,
            Anchor::Middle,
            0.,
            0.,
            10.,
            self.width,
            self.height,
        );

        let handle = world
            .create_entity()
            .with(transform)
            .with(Parent { entity: parent })
            .build();
        let border_entity = world
            .create_entity()
            .with(border.clone())
            .with(border_transform)
            .with(Parent { entity: handle })
            .build();
        world
            .create_entity()
            .with(border.clone())
            .with(border_overlay_1_transform)
            .with(Parent {
                entity: border_entity,
            })
            .build();
        world
            .create_entity()
            .with(border)
            .with(border_overlay_2_transform)
            .with(Parent {
                entity: border_entity,
            })
            .build();

        world
            .create_entity()
            .with(background)
            .with(background_transform)
            .with(ButtonComp)
            .with(UiButtonActionRetrigger {
                on_click_start: vec![],
                on_click_stop: vec![],
                on_hover_start: vec![],
                on_hover_stop: vec![],
            })
            .with(Parent { entity: handle })
            .build();
        world
            .create_entity()
            .with(text)
            .with(text_transform)
            .with(Parent { entity: handle })
            .build();
        world
            .create_entity()
            .with(catch_transform)
            .with(Parent { entity: handle })
            .build();

        handle
    }
}
