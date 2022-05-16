mod entity;
use entity::*;

struct MainState {
    player: Entity,
    rng: Rand32,
    assets: Assets,
    input: InputState,
}

impl MainState {
    pub fn new(_ctx: &mut Context) -> GameResult<MainState> {
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
                knockback: 1,
                attack_cooldown: 1,
            };

        // Seed the RNG
        let mut seed: [u8; 8] = [0; 8];
        getrandom::getrandom(&mut seed[..]).expect("aieee, could not seed rng!");
        let mut rng = Rand32::new(u64::from_ne_bytes(seed));

        // Load assets
        let assets = Assets::new(_ctx)?;

        let s = MainState {
            player,
            rng,
            assets,
            input: InputState::default(),
        };

        Ok(s)
    }
}

impl EventHandler<GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;
        // Update code here...
        while timer::check_update_time(_ctx, DESIRED_FPS) {
            if self.player.falling {
                self.player.jump += PLAYER_JUMP_TIME;
            }

            handle_player_input(&mut self.player, &self.input);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::WHITE);

        // Draw the player, as a test
        draw_entity(&mut self.assets, ctx, &self.player, self.player.pos)?;

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
        .window_mode(conf::WindowMode::default().dimensions(320.0, 240.0))
        .add_resource_path(resource_dir)
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let game = MainState::new(&mut ctx)?;

    // Run!
    event::run(ctx, event_loop, game)
}
