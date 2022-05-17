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
pub const PLAYER_ANIM_RATE: i32 = 3;
pub const PLAYER_JUMP_VELOCITY: f32 = 23.7;
pub const PLAYER_JUMP_TIME: f32 = 0.25; // default 0.3
pub const GROUND: f32 = 208.0;

pub struct Assets {
    pub player: graphics::Image,
    pub player2: graphics::Image,
    pub player3: graphics::Image,
    pub player4: graphics::Image,
    pub player5: graphics::Image,
    pub player6: graphics::Image,
    pub player7: graphics::Image,
    pub player8: graphics::Image,
    pub player9: graphics::Image,
    pub player0: graphics::Image,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let player = graphics::Image::new(ctx, "/player.png")?;
        let player2 = graphics::Image::new(ctx, "/player2.png")?;
        let player3 = graphics::Image::new(ctx, "/player3.png")?;
        let player4 = graphics::Image::new(ctx, "/player4.png")?;
        let player5 = graphics::Image::new(ctx, "/player5.png")?;
        let player6 = graphics::Image::new(ctx, "/player6.png")?;
        let player7 = graphics::Image::new(ctx, "/player7.png")?;
        let player8 = graphics::Image::new(ctx, "/player8.png")?;
        let player9 = graphics::Image::new(ctx, "/player9.png")?;
        let player0 = graphics::Image::new(ctx, "/player0.png")?;

        Ok (
            Assets {
                player,
                player2,
                player3,
                player4,
                player5,
                player6,
                player7,
                player8,
                player9,
                player0,
            }
        )
    }

    fn image(&mut self, entity: &Entity) -> &mut graphics::Image {
        match entity.tag {
            EntityType::Player => {
                match entity.facing {
                    Direction::Left => {
                        match entity.frame {
                            Frame::Stand => &mut self.player,
                            Frame::Walk1 => &mut self.player3,
                            Frame::Walk2 => &mut self.player5,
                            Frame::Walk3 => &mut self.player7,
                            Frame::Walk4 => &mut self.player9,
                        }
                    }

                    Direction::Right => {
                        match entity.frame {
                            Frame::Stand => &mut self.player2,
                            Frame::Walk1 => &mut self.player4,
                            Frame::Walk2 => &mut self.player6,
                            Frame::Walk3 => &mut self.player8,
                            Frame::Walk4 => &mut self.player0,
                        }
                    }
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

pub enum Frame {
    Stand,
    Walk1,
    Walk2,
    Walk3,
    Walk4,
}

pub struct Entity {
    pub tag: EntityType,
    pub pos: (i16, i16),
    pub facing: Direction,
    pub frame: Frame,
    pub falling: bool,
    pub jump: f32,
    pub health: i8,
    pub damage: i8,
    pub knockback: i8,
    pub attack_cooldown: i8
}

pub fn pos_to_p2(coords: (i16, i16)) -> Point2<f32> {
    Point2 {x: coords.0 as f32, y: coords.1 as f32}
}

pub fn draw_entity(assets: &mut Assets, ctx: &mut Context, entity: &Entity, coords: (i16, i16)) -> GameResult {
    let pos = pos_to_p2(coords);

    let image = assets.image(entity);
    let drawparams = graphics::DrawParam::new().dest(pos);

    graphics::draw(ctx, image, drawparams)
}

pub fn handle_player_input(entity: &mut Entity, input: &InputState) {
    entity.pos.0 += (PLAYER_MOVE_RATE * input.x) as i16;

    if input.x != 0.0 {
        match entity.frame {
            Frame::Stand => entity.frame = Frame::Walk1,
            Frame::Walk1 => entity.frame = Frame::Walk2,
            Frame::Walk2 => entity.frame = Frame::Walk3,
            Frame::Walk3 => entity.frame = Frame::Walk4,
            Frame::Walk4 => entity.frame = Frame::Stand,
        }
    } else if input.x == 0.0 {
        entity.frame = Frame::Stand;
    }

    if input.jump {
        entity.falling = true;
        entity.pos = (entity.pos.0, ((4.9 * entity.jump.powf(2.0)) - (PLAYER_JUMP_VELOCITY * entity.jump) + GROUND) as i16);

        if entity.pos.1 > GROUND as i16 {

            entity.pos = (entity.pos.0, GROUND as i16);
            entity.jump = 0.0;

        }

    } else if ! input.jump {

        if entity.pos.1 >= GROUND as i16 {

                entity.falling = false;
                entity.jump = 0.0;
                entity.pos = (entity.pos.0, GROUND as i16);

        } else if entity.pos.1 < GROUND as i16 {

            entity.pos = (entity.pos.0, ((4.9 * entity.jump.powf(2.0)) - (PLAYER_JUMP_VELOCITY * entity.jump) + GROUND) as i16);

            if entity.pos.1 >= GROUND as i16 {

                entity.falling = false;
                entity.jump = 0.0;
                entity.pos = (entity.pos.0, GROUND as i16);

            }

        }
    }

    if entity.falling {
        entity.frame = Frame::Walk2;
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
