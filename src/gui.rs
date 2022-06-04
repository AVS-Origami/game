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
    pub scale: f32,
}

pub fn draw_button(ctx: &mut Context, button: &mut Button, image: &mut graphics::Image) -> GameResult {
    let button_dest = Point2 {x: button.pos.0 as f32 * button.scale, y: button.pos.1 as f32 * button.scale};
    let drawparams = DrawParam::new().dest(button_dest).scale(Vector2 {x: button.scale, y: button.scale});
    graphics::draw(ctx, image, drawparams)
}

pub struct Gui {
    pub play: Button,
}