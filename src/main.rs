use std::env;
use std::path;
use ggez::conf;

use ggez::mint::Point2;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};

use oorandom::Rand32;

struct Assets {
    player_img: graphics::Image,
}

impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Assets> {
        let player_img = graphics::Image::new(ctx, "/player.png")?;

        Ok(Assets {
            player_img,
        })
    }

    fn image(&mut self, entity: &Entity) -> &mut graphics::Image {
        match entity.tag {
            EntityType::Player => &mut self.player_img,
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
}

struct MainState {
    player: Entity,
    rng: Rand32,
    assets: Assets,
}

fn pos_to_p2(coords: (i16, i16)) -> Point2<f32> {
    Point2 {x: coords.0 as f32, y: coords.1 as f32}
}

fn draw_entity(assets: &mut Assets, ctx: &mut Context, entity: &Entity, coords: (i16, i16)) -> GameResult {
    let pos = pos_to_p2(coords);

    let image = assets.image(entity);
    let drawparams = graphics::DrawParam::new().dest(pos);

    graphics::draw(ctx, image, drawparams)
}

impl MainState {
    pub fn new(_ctx: &mut Context) -> GameResult<MainState> {
        // Load/create resources such as images here.
        let player = Entity {
                tag: EntityType::Player,
                pos: (0, 0),
                facing: Direction::Left,
                falling: false,
                health: 1,
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
        };

        Ok(s)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::WHITE);

        // Draw the player, as a test
        draw_entity(&mut self.assets, ctx, &self.player, (79, 79))?;
        // Draw code here...
        graphics::present(ctx)
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
