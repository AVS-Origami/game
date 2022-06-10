/// *********************************************************************
/// Use necessary crates.
/// *********************************************************************
use ggez::graphics;

use ggez::mint::{Point2, Vector2};
use ggez::{Context, GameResult};

use oorandom::Rand32;

use crate::assets::*;

const PLAYER_MOVE_RATE: f32 = 2.0;
const PLAYER_JUMP_VELOCITY: f32 = 23.7;
const MONSTER_MOVE_RATE: i8 = 2;

pub const SCREEN_WIDTH: f32 = 320.0;
pub const SCREEN_HEIGHT: f32 = 240.0;
pub const PLAYER_JUMP_TIME: f32 = 0.25;
pub const GROUND: f32 = SCREEN_HEIGHT - 24.0;

/// *********************************************************************
/// Create an enumeration of all the different entity types.
/// *********************************************************************

#[derive(Clone)]
pub enum EntityType {
    Player,
    Zombie,
    Skeleton,
}

/// *********************************************************************
/// Create an enumeration of all entity directions.
/// *********************************************************************

#[derive(Clone)]
pub enum Direction {
    Left,
    Right,
}

/// *********************************************************************
/// Create an enumeration of all entity animation frames.
/// *********************************************************************

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Entity {
    pub tag: EntityType,
    pub pos: (i16, i16),
    pub facing: Direction,
    pub frame: Frame,
    pub falling: bool,
    pub jump: f32,
    pub jump_from: i16,
    pub health: i8,
    pub ticks: i8,
}

/// *********************************************************************
/// Create a function to draw entities.
/// *********************************************************************

pub fn draw_entity(assets: &mut Assets, ctx: &mut Context, entity: &Entity, coords: (i16, i16), scale: f32) -> GameResult {
    let pos = pos_to_p2(coords);

    let image = assets.image(entity);
    let drawparams = graphics::DrawParam::new().dest(pos).scale(Vector2{x: scale, y: scale});

    graphics::draw(ctx, image, drawparams)
}

/// *********************************************************************
/// Create a function to advance entity animations.
/// *********************************************************************
fn advance_animation(entity: &mut Entity) {
    match entity.frame {
        Frame::Stand => entity.frame = Frame::Walk1,
        Frame::Walk1 => entity.frame = Frame::Walk2,
        Frame::Walk2 => entity.frame = Frame::Walk3,
        Frame::Walk3 => entity.frame = Frame::Walk4,
        Frame::Walk4 => entity.frame = Frame::Stand,
    }
}

/// *********************************************************************
/// Create a function to handle player input and update the player's
/// properties accordingly.
/// *********************************************************************

pub fn handle_player_input(entity: &mut Entity, input: &mut InputState, scale: f32) {
    entity.pos.0 += (PLAYER_MOVE_RATE * input.x * scale) as i16;

    // Make sure the player can't go off the edge of the screen
    if entity.pos.0 < 0 {
        entity.pos = (0, entity.pos.1);
    } else if entity.pos.0 > (SCREEN_WIDTH * scale) as i16 - 16 * scale as i16 {
        entity.pos = ((SCREEN_WIDTH * scale) as i16 - 16 * scale as i16, entity.pos.1);
    }

    if input.x != 0.0 {
        advance_animation(entity);
    } else if input.x == 0.0 {
        entity.frame = Frame::Stand;
    }

    if input.jump &&! input.jump_spam {
        entity.falling = true;
        if entity.jump_from == GROUND as i16 {
            entity.jump_from = GROUND as i16 - 1;
        }
        entity.pos = (entity.pos.0, (((4.9 * entity.jump.powf(2.0)) - (PLAYER_JUMP_VELOCITY * entity.jump) + entity.jump_from as f32 / scale) * scale) as i16);

        if entity.pos.1 >= (GROUND * scale) as i16 {

            entity.pos = (entity.pos.0, (GROUND * scale) as i16);
            entity.jump = 0.0;
            input.jump_spam = true;

        }

    } else if ! input.jump {

        if entity.pos.1 >= (GROUND * scale) as i16 {

                entity.falling = false;
                entity.jump = 0.0;
                entity.pos = (entity.pos.0, (GROUND * scale) as i16);

        } else if entity.pos.1 < (GROUND * scale) as i16 {

            entity.pos = (entity.pos.0, (((4.9 * entity.jump.powf(2.0)) - (PLAYER_JUMP_VELOCITY * entity.jump) + entity.jump_from as f32 / scale) * scale) as i16);

            if entity.pos.1 >= (GROUND * scale) as i16 {

                entity.falling = false;
                entity.jump = 0.0;
                entity.pos = (entity.pos.0, (GROUND * scale) as i16);

            }

        }

        input.jump_spam = false;
    }

    if input.jump_spam {
        entity.falling = false;
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
    pub jump_spam: bool,
}

impl Default for InputState {
    fn default() -> Self {
        InputState {
            x: 0.0,
            jump: false,
            attack: false,
            jump_spam: false,
        }
    }
}

/// *********************************************************************
/// Create a function to spawn monsters.
/// *********************************************************************

pub fn spawn_monsters(rng: &mut Rand32, monster_list: &mut Vec<Entity>, difficulty: u32) {
    for _ in 0..difficulty {
        let tag_gen = rng.rand_range(0..2);
        let tag = if tag_gen == 0 {
            EntityType::Zombie
        } else {
            EntityType::Skeleton
        };

        let pos = (rng.rand_range(16..(SCREEN_WIDTH - 32.0) as u32) as i16, 8);

        let facing_gen = rng.rand_range(0..2);
        let facing = if facing_gen == 0 {
            Direction::Left
        } else {
            Direction::Right
        };

        let monster = Entity {
            tag,
            pos,
            facing,
            frame: Frame::Walk2,
            falling: true,
            jump: 0.0,
            jump_from: pos.1,
            health: 1,
            ticks: 0,
        };

        monster_list.push(monster);
    }
}

/// *********************************************************************
/// Create a function to draw monsters.
/// *********************************************************************
pub fn draw_monsters(monster_list: &mut Vec<Entity>, assets: &mut Assets, ctx: &mut Context, scale: f32) -> GameResult {
    Ok (
        for monster in monster_list {
            draw_entity(assets, ctx, monster, monster.pos, scale)?;
        }
    )
}

/// *********************************************************************
/// Create a function to update monsters.
/// *********************************************************************
pub fn update_monsters(monster_list: &mut Vec<Entity>, scale: f32) {
    for monster in monster_list {
        if ! monster.falling {
            monster.ticks += 1;
        }
        if monster.ticks == MONSTER_MOVE_RATE && ! monster.falling {
            let direction = match monster.facing {
                Direction::Left => -1 * scale as i16,
                Direction::Right => 1 * scale as i16,
            };

            monster.pos = (monster.pos.0 + direction, monster.pos.1);
            if monster.pos.0 < 0 {
                monster.facing = Direction::Right;
                monster.pos = (0, monster.pos.1);
            } else if monster.pos.0 > ((SCREEN_WIDTH - 16.0) * scale) as i16 {
                monster.facing = Direction::Left;
                monster.pos = (((SCREEN_WIDTH - 16.0) * scale) as i16, monster.pos.1);
            }
            advance_animation(monster);
            monster.ticks = 0;
        }

        if monster.falling {
            monster.pos = (monster.pos.0, (((1.09 * monster.jump.powf(2.0)) - 8.0) * scale) as i16);
            monster.jump += 1.0;
        }

        if monster.pos.1 >= (GROUND * scale) as i16 {
            monster.falling = false;
            monster.pos = (monster.pos.0, (GROUND * scale) as i16);
        }
    }
}

/// *********************************************************************
/// Detect entity collisions.
/// *********************************************************************
pub fn is_touching(entity1: &Entity, entity2: &Entity, scale: f32) -> bool {
    if (entity1.pos.0 - entity2.pos.0).abs() <= 16 * scale as i16 && (entity1.pos.1 - entity2.pos.1).abs() <= 16 * scale as i16 {
        return true;
    } else {
        return false;
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
