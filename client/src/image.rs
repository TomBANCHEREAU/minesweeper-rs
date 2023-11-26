use core::tile::{TileContent, TileState};
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

pub enum Sprite {
    Bomb,
    Empty,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Flag,
    Covered,
    Player,
}

impl From<Sprite> for (f64, f64, f64, f64) {
    fn from(value: Sprite) -> Self {
        match value {
            Sprite::Bomb => (64., 39., 16., 16.),
            Sprite::Empty => (0., 23., 16., 16.),
            Sprite::One => (16., 23., 16., 16.),
            Sprite::Two => (32., 23., 16., 16.),
            Sprite::Three => (48., 23., 16., 16.),
            Sprite::Four => (64., 23., 16., 16.),
            Sprite::Five => (80., 23., 16., 16.),
            Sprite::Six => (96., 23., 16., 16.),
            Sprite::Seven => (112., 23., 16., 16.),
            Sprite::Eight => (128., 23., 16., 16.),
            Sprite::Flag => (16., 39., 16., 16.),
            Sprite::Covered => (0., 39., 16., 16.),
            Sprite::Player => (112., 39., 16., 16.),
        }
    }
}

impl From<&TileState> for Sprite {
    fn from(value: &TileState) -> Sprite {
        match value {
            TileState::Untouched => Sprite::Covered,
            TileState::Flagged => Sprite::Flag,
            TileState::Discovered(content) => match content {
                TileContent::Empty => Sprite::Empty,
                TileContent::Number(1) => Sprite::One,
                TileContent::Number(2) => Sprite::Two,
                TileContent::Number(3) => Sprite::Three,
                TileContent::Number(4) => Sprite::Four,
                TileContent::Number(5) => Sprite::Five,
                TileContent::Number(6) => Sprite::Six,
                TileContent::Number(7) => Sprite::Seven,
                TileContent::Number(8) => Sprite::Eight,
                TileContent::Bomb => Sprite::Bomb,
                TileContent::Number(_) => todo!(),
            },
        }
    }
}

pub fn draw_sprite(
    image_element: &HtmlImageElement,
    context: &CanvasRenderingContext2d,
    sprite: Sprite,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
) {
    let (sx, sy, sw, sh) = sprite.into();
    context
        .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            image_element,
            sx,
            sy,
            sw,
            sh,
            x,
            y,
            w,
            h,
        )
        .unwrap();
}
