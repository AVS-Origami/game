use ggez::{GameResult, Context};
use ggez::graphics::{self, DrawParam};
use ggez::mint::{Point2, Vector2};

#[derive(PartialEq)]
pub enum Screen {
    Title,
    Game,
    Death,
}

pub struct Button {
    pub pos: (i16, i16),
    pub width: i16,
    pub scale: f32,
    pub hover: bool,
}

impl Button {
    pub fn draw(ctx: &mut Context, button: &mut Button, image: &mut graphics::Image) -> GameResult {
        let button_dest = Point2 {x: button.pos.0 as f32 * button.scale, y: button.pos.1 as f32 * button.scale};
        let drawparams = DrawParam::new().dest(button_dest).scale(Vector2 {x: button.scale, y: button.scale});
        graphics::draw(ctx, image, drawparams)
    }

    pub fn hover(button: &mut Button, mx: f32, my: f32, scale: f32) {
        if mx >= button.pos.0 as f32 * scale && mx <= (button.pos.0 + button.width) as f32 * scale && my >= button.pos.1 as f32 * scale && my <= (button.pos.1 + button.width) as f32 * scale {

            if ! button.hover {
                button.pos = (button.pos.0, button.pos.1 - (4.0 * scale) as i16);
                button.hover = true;
            }

        } else if button.hover {
            button.pos = (button.pos.0, button.pos.1 + (4.0 * scale) as i16);
            button.hover = false;
        }
    }
}

pub struct Gui {
    pub play: Button,
}