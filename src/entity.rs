/// *********************************************************************
/// Use necessary crates.
/// *********************************************************************
use ggez::graphics::{self, FilterMode};

use ggez::mint::{Point2, Vector2};
use ggez::{Context, GameResult};

use oorandom::Rand32;

const PLAYER_MOVE_RATE: f32 = 2.0;
const PLAYER_JUMP_VELOCITY: f32 = 23.7;
const MONSTER_MOVE_RATE: i8 = 2;

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
    pub zombie: graphics::Image,
    pub zombie2: graphics::Image,
    pub zombie3: graphics::Image,
    pub zombie4: graphics::Image,
    pub zombie5: graphics::Image,
    pub zombie6: graphics::Image,
    pub zombie7: graphics::Image,
    pub zombie8: graphics::Image,
    pub zombie9: graphics::Image,
    pub zombie0: graphics::Image,
    pub skeleton: graphics::Image,
    pub skeleton2: graphics::Image,
    pub skeleton3: graphics::Image,
    pub skeleton4: graphics::Image,
    pub skeleton5: graphics::Image,
    pub skeleton6: graphics::Image,
    pub skeleton7: graphics::Image,
    pub skeleton8: graphics::Image,
    pub skeleton9: graphics::Image,
    pub skeleton0: graphics::Image,
    pub ground: graphics::Image,
    pub grass: graphics::Image,
    pub moss: graphics::Image,
    pub font: graphics::Font,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let mut player = graphics::Image::new(ctx, "/player.png")?;
        let mut player2 = graphics::Image::new(ctx, "/player2.png")?;
        let mut player3 = graphics::Image::new(ctx, "/player3.png")?;
        let mut player4 = graphics::Image::new(ctx, "/player4.png")?;
        let mut player5 = graphics::Image::new(ctx, "/player5.png")?;
        let mut player6 = graphics::Image::new(ctx, "/player6.png")?;
        let mut player7 = graphics::Image::new(ctx, "/player7.png")?;
        let mut player8 = graphics::Image::new(ctx, "/player8.png")?;
        let mut player9 = graphics::Image::new(ctx, "/player9.png")?;
        let mut player0 = graphics::Image::new(ctx, "/player0.png")?;
        let mut zombie = graphics::Image::new(ctx, "/zombie.png")?;
        let mut zombie2 = graphics::Image::new(ctx, "/zombie2.png")?;
        let mut zombie3 = graphics::Image::new(ctx, "/zombie3.png")?;
        let mut zombie4 = graphics::Image::new(ctx, "/zombie4.png")?;
        let mut zombie5 = graphics::Image::new(ctx, "/zombie5.png")?;
        let mut zombie6 = graphics::Image::new(ctx, "/zombie6.png")?;
        let mut zombie7 = graphics::Image::new(ctx, "/zombie7.png")?;
        let mut zombie8 = graphics::Image::new(ctx, "/zombie8.png")?;
        let mut zombie9 = graphics::Image::new(ctx, "/zombie9.png")?;
        let mut zombie0 = graphics::Image::new(ctx, "/zombie0.png")?;
        let mut skeleton = graphics::Image::new(ctx, "/skeleton.png")?;
        let mut skeleton2 = graphics::Image::new(ctx, "/skeleton2.png")?;
        let mut skeleton3 = graphics::Image::new(ctx, "/skeleton3.png")?;
        let mut skeleton4 = graphics::Image::new(ctx, "/skeleton4.png")?;
        let mut skeleton5 = graphics::Image::new(ctx, "/skeleton5.png")?;
        let mut skeleton6 = graphics::Image::new(ctx, "/skeleton6.png")?;
        let mut skeleton7 = graphics::Image::new(ctx, "/skeleton7.png")?;
        let mut skeleton8 = graphics::Image::new(ctx, "/skeleton8.png")?;
        let mut skeleton9 = graphics::Image::new(ctx, "/skeleton9.png")?;
        let mut skeleton0 = graphics::Image::new(ctx, "/skeleton0.png")?;
        let mut ground = graphics::Image::new(ctx, "/ground.png")?;
        let mut grass = graphics::Image::new(ctx, "/grass.png")?;
        let mut moss = graphics::Image::new(ctx, "/moss.png")?;
        let font = graphics::Font::new(ctx, "/MorePerfectDOSVGA.ttf")?;

        player.set_filter(FilterMode::Nearest);
        player2.set_filter(FilterMode::Nearest);
        player3.set_filter(FilterMode::Nearest);
        player4.set_filter(FilterMode::Nearest);
        player5.set_filter(FilterMode::Nearest);
        player6.set_filter(FilterMode::Nearest);
        player7.set_filter(FilterMode::Nearest);
        player8.set_filter(FilterMode::Nearest);
        player9.set_filter(FilterMode::Nearest);
        player0.set_filter(FilterMode::Nearest);
        zombie.set_filter(FilterMode::Nearest);
        zombie2.set_filter(FilterMode::Nearest);
        zombie3.set_filter(FilterMode::Nearest);
        zombie4.set_filter(FilterMode::Nearest);
        zombie5.set_filter(FilterMode::Nearest);
        zombie6.set_filter(FilterMode::Nearest);
        zombie7.set_filter(FilterMode::Nearest);
        zombie8.set_filter(FilterMode::Nearest);
        zombie9.set_filter(FilterMode::Nearest);
        zombie0.set_filter(FilterMode::Nearest);
        skeleton.set_filter(FilterMode::Nearest);
        skeleton2.set_filter(FilterMode::Nearest);
        skeleton3.set_filter(FilterMode::Nearest);
        skeleton4.set_filter(FilterMode::Nearest);
        skeleton5.set_filter(FilterMode::Nearest);
        skeleton6.set_filter(FilterMode::Nearest);
        skeleton7.set_filter(FilterMode::Nearest);
        skeleton8.set_filter(FilterMode::Nearest);
        skeleton9.set_filter(FilterMode::Nearest);
        skeleton0.set_filter(FilterMode::Nearest);
        ground.set_filter(FilterMode::Nearest);
        grass.set_filter(FilterMode::Nearest);
        moss.set_filter(FilterMode::Nearest);

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
                zombie,
                zombie2,
                zombie3,
                zombie4,
                zombie5,
                zombie6,
                zombie7,
                zombie8,
                zombie9,
                zombie0,
                skeleton,
                skeleton2,
                skeleton3,
                skeleton4,
                skeleton5,
                skeleton6,
                skeleton7,
                skeleton8,
                skeleton9,
                skeleton0,
                ground,
                grass,
                moss,
                font,
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

            EntityType::Zombie => {
                match entity.facing {
                    Direction::Left => {
                        match entity.frame {
                            Frame::Stand => &mut self.zombie,
                            Frame::Walk1 => &mut self.zombie3,
                            Frame::Walk2 => &mut self.zombie5,
                            Frame::Walk3 => &mut self.zombie7,
                            Frame::Walk4 => &mut self.zombie9,
                        }
                    }

                    Direction::Right => {
                        match entity.frame {
                            Frame::Stand => &mut self.zombie2,
                            Frame::Walk1 => &mut self.zombie4,
                            Frame::Walk2 => &mut self.zombie6,
                            Frame::Walk3 => &mut self.zombie8,
                            Frame::Walk4 => &mut self.zombie0,
                        }
                    }
                }
            }

            EntityType::Skeleton => {
                match entity.facing {
                    Direction::Left => {
                        match entity.frame {
                            Frame::Stand => &mut self.skeleton,
                            Frame::Walk1 => &mut self.skeleton3,
                            Frame::Walk2 => &mut self.skeleton5,
                            Frame::Walk3 => &mut self.skeleton7,
                            Frame::Walk4 => &mut self.skeleton9,
                        }
                    }

                    Direction::Right => {
                        match entity.frame {
                            Frame::Stand => &mut self.skeleton2,
                            Frame::Walk1 => &mut self.skeleton4,
                            Frame::Walk2 => &mut self.skeleton6,
                            Frame::Walk3 => &mut self.skeleton8,
                            Frame::Walk4 => &mut self.skeleton0,
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

#[derive(Clone)]
pub enum EntityType {
    Player,
    Zombie,
    Skeleton,
}

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
    pub health: i8,
    pub damage: i8,
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

pub fn handle_player_input(entity: &mut Entity, input: &InputState, scale: f32) {
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

    if input.jump {
        entity.falling = true;
        entity.pos = (entity.pos.0, (((4.9 * entity.jump.powf(2.0)) - (PLAYER_JUMP_VELOCITY * entity.jump) + GROUND) * scale) as i16);

        if entity.pos.1 >= (GROUND * scale) as i16 {

            entity.pos = (entity.pos.0, (GROUND * scale) as i16);
            entity.jump = 0.0;

        }

    } else if ! input.jump {

        if entity.pos.1 >= (GROUND * scale) as i16 {

                entity.falling = false;
                entity.jump = 0.0;
                entity.pos = (entity.pos.0, (GROUND * scale) as i16);

        } else if entity.pos.1 < (GROUND * scale) as i16 {

            entity.pos = (entity.pos.0, (((4.9 * entity.jump.powf(2.0)) - (PLAYER_JUMP_VELOCITY * entity.jump) + GROUND) * scale) as i16);

            if entity.pos.1 >= (GROUND * scale) as i16 {

                entity.falling = false;
                entity.jump = 0.0;
                entity.pos = (entity.pos.0, (GROUND * scale) as i16);

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
/// Create a function to spawn monsters.
/// *********************************************************************

pub fn spawn_monsters(rng: &mut Rand32, monster_list: &mut Vec<Entity>) {
    for _ in 0..rng.rand_range(2..5) {
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
            health: 1,
            damage: 1,
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
