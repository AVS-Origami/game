use std::env;
use std::path;
use ggez::conf;
use ggez::timer;

use ggez::mint::{Point2, Vector2};
use ggez::{Context, ContextBuilder, GameResult, GameError};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler, KeyCode, KeyMods};

use oorandom::Rand32;

const PLAYER_MOVE_RATE: f32 = 1.0;

struct Assets {
    player_image: graphics::Image,
}

impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Assets> {
        let player_image = graphics::Image::new(ctx, "/player.png")?;

        Ok(Assets {
            player_image,
        })
    }

    fn image(&mut self, entity: &Entity) -> &mut graphics::Image {
        match entity.tag {
            EntityType::Player => &mut self.player_image,
        }
    }
}

enum EntityType {
    Player,
}

enum Direction {
    Left,
    Right,
}

struct Entity {
    tag: EntityType,
    pos: (i16, i16),
    facing: Direction,
    falling: bool,
    health: i8,
    damage: i8,
    knockback: i8,
}

fn pos_to_p2(coords: (i16, i16)) -> Point2<f32> {
    Point2 {x: coords.0 as f32, y: coords.1 as f32}
}

fn p2_to_pos(p2: Point2<f32>) -> (i16, i16) {
    (p2.x as i16, p2.y as i16)
}

fn draw_entity(assets: &mut Assets, ctx: &mut Context, entity: &Entity, coords: (i16, i16)) -> GameResult {
    let pos = pos_to_p2(coords);

    let image = assets.image(entity);
    let drawparams = graphics::DrawParam::new().dest(pos);

    graphics::draw(ctx, image, drawparams)
}

fn handle_player_input(entity: &mut Entity, input: &InputState) {
    entity.pos.0 += (PLAYER_MOVE_RATE * input.input_x) as i16
}

struct InputState {
    input_x: f32,
    jump: bool,
    attack: bool,
}

impl Default for InputState {
    fn default() -> Self {
        InputState {
            input_x: 0.0,
            jump: false,
            attack: false,
        }
    }
}

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
                pos: (0, 0),
                facing: Direction::Left,
                falling: false,
                health: 4,
                damage: 1,
                knockback: 1,
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
            handle_player_input(&mut self.player, &self.input)
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

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Left => self.input.input_x -= 1.0,
            KeyCode::Right => self.input.input_x += 1.0,
            KeyCode::Z => self.input.jump = true,
            KeyCode::X => self.input.attack = true,
            _ => (),
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, _keycode: KeyCode, _keymods: KeyMods) {
        match _keycode {
            KeyCode::Left | KeyCode::Right => self.input.input_x = 0.0,
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
