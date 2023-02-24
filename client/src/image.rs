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
        }
    }
}

pub struct ImageManager {
    // load_handle: LoadHandle,
    image_element: HtmlImageElement,
}

impl ImageManager {
    pub fn new() -> Self {
        let image_element = HtmlImageElement::new().unwrap();
        image_element.set_src("/images/sprites.png");
        image_element
            .style()
            .set_property("visibility", "none")
            .unwrap();
        Self { image_element }
    }
    pub fn draw_sprite(
        &self,
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
                &self.image_element,
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
}
