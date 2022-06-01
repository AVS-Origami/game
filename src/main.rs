/// *********************************************************************
/// Use necessary crates.
/// *********************************************************************

use std::env;
use std::path;
use ggez::conf;
use ggez::mint::Vector2;
use ggez::timer;

use ggez::{Context, ContextBuilder, GameResult, GameError};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::mint::Point2;

use oorandom::Rand32;

/// *********************************************************************
/// Import modules.
/// *********************************************************************

mod entity;
mod settings;
use entity::*;
use settings::*;

struct MainState {
    player: Entity,
    rng: Rand32,
    monsters: Vec<Entity>,
    spawn_cycle: f32,
    ticks: f32,
    assets: Assets,
    input: InputState,
    scale: f32,
}

impl MainState {
    pub fn new(ctx: &mut Context, scale: f32) -> GameResult<MainState> {
        // Load/create resources such as images here.
        let player = Entity {
                tag: EntityType::Player,
                pos: (0, GROUND as i16),
                facing: Direction::Left,
                frame: Frame::Stand,
                falling: false,
                jump: 0.0,
                health: 4,
                damage: 1,
                ticks: 0,
            };

        // Seed the RNG
        let mut seed: [u8; 8] = [0; 8];
        getrandom::getrandom(&mut seed[..]).expect("aieee, could not seed rng!");
        let mut rng = Rand32::new(u64::from_ne_bytes(seed));

        let monsters: Vec<Entity> = Vec::new();

        // Load assets
        let assets = Assets::new(ctx)?;

        let spawn_cycle = rng.rand_range(4..9) as f32;

        let s = MainState {
            player,
            rng,
            assets,
            monsters,
            spawn_cycle,
            ticks: 0.0,
            input: InputState::default(),
            scale,
        };

        Ok(s)
    }
}

impl EventHandler<GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;
        // Update code here...
        while timer::check_update_time(ctx, DESIRED_FPS) {
            if self.player.falling {
                self.player.jump += PLAYER_JUMP_TIME;
            }
            
            if self.player.health > 0 {
                handle_player_input(&mut self.player, &self.input, self.scale);
            }
        }

        self.ticks += 1.0;
        if (self.ticks / 60.0) == self.spawn_cycle {
            spawn_monsters(&mut self.rng, &mut self.monsters);
            self.spawn_cycle = self.rng.rand_range(4..9) as f32;
            self.ticks = 0.0;
        }

        update_monsters(&mut self.monsters, self.scale);

        let mut alive_monsters = Vec::new();

        for monster in self.monsters.clone() {
            if is_touching(&self.player, &monster, self.scale) {
                if ! self.player.falling {
                    alive_monsters.push(monster);
                    if self.player.health > 0 {
                        self.player.health -= 1;
                    }
                }
            } else {
                alive_monsters.push(monster);
            }
        }

        self.monsters = alive_monsters;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::WHITE);

        // Draw the player
        draw_entity(&mut self.assets, ctx, &self.player, self.player.pos, self.scale)?;

        // Draw the monsters
        draw_monsters(&mut self.monsters, &mut self.assets, ctx, self.scale)?;

        // Draw the ground
        draw_ground(&mut self.assets, ctx, self.scale)?;

        // Draw code here...

        graphics::present(ctx)?;

        timer::yield_now();

        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, keymods: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Left => {
                if keymods.contains(KeyMods::SHIFT) {
                    self.input.x = -2.5;
                } else {
                    self.input.x = -1.0;
                }

                self.player.facing = Direction::Left;
            }

            KeyCode::Right => {
                if keymods.contains(KeyMods::SHIFT) {
                    self.input.x = 2.5;
                } else {
                    self.input.x = 1.0;
                }
                
                self.player.facing = Direction::Right;
            }

            KeyCode::LShift => {
                if self.input.x == 1.0 {
                    self.input.x = 2.5;
                } else if self.input.x == -1.0 {
                    self.input.x = -2.5
                }
            }

            KeyCode::Z => {
                if ! self.player.falling {
                    self.input.jump = true;
                }
            }

            KeyCode::X => self.input.attack = true,
            _ => (),
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, _keycode: KeyCode, _keymods: KeyMods) {
        match _keycode {
            KeyCode::Left | KeyCode::Right => self.input.x = 0.0,
            KeyCode::Z => self.input.jump = false,
            KeyCode::X => self.input.attack = false,
            _ => (),
        }
    }
}

fn main() -> GameResult {
    let scale = fetch_setting("scale");
    // Add CARGO_MANIFEST_DIR/resources to resource paths
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("game", "AVS Origami")
        .window_setup(conf::WindowSetup::default().title("game"))
        .window_mode(conf::WindowMode::default().dimensions(SCREEN_WIDTH * scale, SCREEN_HEIGHT * scale))
        .add_resource_path(resource_dir)
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let game = MainState::new(&mut ctx, scale)?;

    // Run!
    event::run(ctx, event_loop, game)
}

fn draw_ground(assets: &mut Assets, ctx: &mut Context, scale: f32) -> GameResult {
    let mut pos = Point2 {x: 0.0, y: GROUND * scale + 16.0 * scale};
    let image = &mut assets.moss;

    Ok (
        for _ in 0..(SCREEN_WIDTH * scale / 8.0) as i32 {
            let drawparams = graphics::DrawParam::new().dest(pos).scale(Vector2{x: scale, y: scale});
            graphics::draw(ctx, image, drawparams)?;
            pos.x += 8.0;
        }
    )
}
