/// *********************************************************************
/// Use necessary crates.
/// *********************************************************************

use ggez::graphics;

use ggez::mint::Point2;
use ggez::{Context, GameResult};

const PLAYER_MOVE_RATE: f32 = 2.0;
const PLAYER_JUMP_VELOCITY: f32 = 23.7;

pub const SCREEN_WIDTH: f32 = 320.0;
pub const SCREEN_HEIGHT: f32 = 240.0;
pub const PLAYER_JUMP_TIME: f32 = 0.25;
pub const GROUND: f32 = SCREEN_HEIGHT - 24.0;

/// *********************************************************************
/// Create a struct containing all the assets used by the game.
/// *********************************************************************

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
    pub ground: graphics::Image,
    pub grass: graphics::Image,
    pub moss: graphics::Image,
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
        let ground = graphics::Image::new(ctx, "/ground.png")?;
        let grass = graphics::Image::new(ctx, "/grass.png")?;
        let moss = graphics::Image::new(ctx, "/moss.png")?;

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
                ground,
                grass,
                moss,
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

/// *********************************************************************
/// Create an enumeration of all the different entity types.
/// *********************************************************************

pub enum EntityType {
    Player,
}

pub enum Direction {
    Left,
    Right,
}

/// *********************************************************************
/// Create an enumeration of all entity animation frames.
/// *********************************************************************

pub enum Frame {
    Stand,
    Walk1,
    Walk2,
    Walk3,
    Walk4,
}

/// *********************************************************************
/// Define a struct containing the properties of an entity.
/// *********************************************************************

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

/// *********************************************************************
/// Create a function to draw entities.
/// *********************************************************************

pub fn draw_entity(assets: &mut Assets, ctx: &mut Context, entity: &Entity, coords: (i16, i16)) -> GameResult {
    let pos = pos_to_p2(coords);

    let image = assets.image(entity);
    let drawparams = graphics::DrawParam::new().dest(pos);

    graphics::draw(ctx, image, drawparams)
}

/// *********************************************************************
/// Create a function to handle player input and update the player's
/// properties accordingly.
/// *********************************************************************

pub fn handle_player_input(entity: &mut Entity, input: &InputState) {
    entity.pos.0 += (PLAYER_MOVE_RATE * input.x) as i16;

    // Make sure the player can't go off the edge of the screen
    if entity.pos.0 < 0 {
        entity.pos = (0, entity.pos.1);
    } else if entity.pos.0 > SCREEN_WIDTH as i16 - 16 {
        entity.pos = (SCREEN_WIDTH as i16 - 16, entity.pos.1);
    }

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

/// *********************************************************************
/// Create a struct and impl to store player input.
/// *********************************************************************

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



/// *********************************************************************
/// Helper functions.
/// *********************************************************************

/// *********************************************************************
/// Convert entity cordinates to a Point2 type.
/// *********************************************************************

fn pos_to_p2(coords: (i16, i16)) -> Point2<f32> {
    Point2 {x: coords.0 as f32, y: coords.1 as f32}
}
