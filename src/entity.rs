pub use std::env;
pub use std::path;
pub use ggez::conf;
pub use ggez::timer;

pub use ggez::mint::Point2;
pub use ggez::{Context, ContextBuilder, GameResult, GameError};
pub use ggez::graphics::{self, Color};
pub use ggez::event::{self, EventHandler, KeyCode, KeyMods};

pub use oorandom::Rand32;

const PLAYER_MOVE_RATE: f32 = 2.0;
const PLAYER_JUMP: [i16; 17] = [-4, -4, -3, -3, -2, -2, -1, -1, 0, 1, 1, 2, 2, 3, 3, 4, 4];

pub struct Assets {
    pub player: graphics::Image,
    pub player2: graphics::Image,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let player = graphics::Image::new(ctx, "/player.png")?;
        let player2 = graphics::Image::new(ctx, "/player2.png")?;

        Ok(Assets {
            player,
            player2,
        })
    }

    fn image(&mut self, entity: &Entity) -> &mut graphics::Image {
        match entity.tag {
            EntityType::Player => {
                match entity.facing {
                    Direction::Left => &mut self.player,
                    Direction::Right => &mut self.player2,
                }
            }
        }
    }
}

pub enum EntityType {
    Player,
}

pub enum Direction {
    Left,
    Right,
}

pub struct Entity {
    pub tag: EntityType,
    pub pos: (i16, i16),
    pub facing: Direction,
    pub falling: bool,
    pub health: i8,
    pub damage: i8,
    pub knockback: i8,
    pub jump: usize,
}

pub fn pos_to_p2(coords: (i16, i16)) -> Point2<f32> {
    Point2 {x: coords.0 as f32, y: coords.1 as f32}
}

pub fn p2_to_pos(p2: Point2<f32>) -> (i16, i16) {
    (p2.x as i16, p2.y as i16)
}

pub fn draw_entity(assets: &mut Assets, ctx: &mut Context, entity: &Entity, coords: (i16, i16)) -> GameResult {
    let pos = pos_to_p2(coords);

    let image = assets.image(entity);
    let drawparams = graphics::DrawParam::new().dest(pos);

    graphics::draw(ctx, image, drawparams)
}

pub fn handle_player_input(entity: &mut Entity, input: &InputState) {
    entity.pos.0 += (PLAYER_MOVE_RATE * input.x) as i16;

    if input.jump {
        if entity.jump > PLAYER_JUMP.len() - 1 {
            entity.jump = 0;
        }
        entity.pos.1 += PLAYER_JUMP[entity.jump];
        entity.jump += 1;
    } else if ! input.jump && entity.jump != 0 {
        if entity.jump > PLAYER_JUMP.len() - 1 {
            entity.jump = 0;
        } else {
            entity.pos.1 += PLAYER_JUMP[entity.jump];
            entity.jump += 1;
        }
    }
}

pub struct InputState {
    pub x: f32,
    pub jump: bool,
    pub attack: bool,
}

impl Default for InputState {
    fn default() -> Self {
        InputState {
            x: 0.0,
            jump: false,
            attack: false,
        }
    }
}
