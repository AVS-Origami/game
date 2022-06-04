use ggez::graphics::{self, FilterMode};

use ggez::{Context, GameResult};

use crate::entity::*;

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
    pub play: graphics::Image,
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
        let mut play = graphics::Image::new(ctx, "/gui/play.png")?;
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
        play.set_filter(FilterMode::Nearest);

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
                play,
                font,
            }
        )
    }

    pub fn image(&mut self, entity: &Entity) -> &mut graphics::Image {
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
