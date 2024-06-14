use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Mesh, Rect, DrawMode, DrawParam};
use ggez::event::{self, EventHandler};
use ggez::mint;
use ggez::glam;
use ggez::input::keyboard::{KeyCode, KeyMods, KeyInput};
use rand::{thread_rng, Rng};

fn main() 
{
    let (mut ctx, event_loop) = ContextBuilder::new("pong", "SmigorX")
        .window_setup(ggez::conf::WindowSetup {
            title: "My Game".to_owned(),
            vsync: true,
            icon: "".to_owned(),
            srgb: true,
            samples: ggez::conf::NumSamples::One,
        })
        .window_mode(ggez::conf::WindowMode {
            width: 800.0,  
            height: 600.0, 
            maximized: false,
            fullscreen_type: ggez::conf::FullscreenType::Windowed,
            borderless: false,
            min_width: 1.0,
            min_height: 1.0,
            max_width: 1.0,
            max_height: 1.0,
            resizable: false,
            logical_size: None,
            resize_on_scale_factor_change: false,
            transparent: false,
            visible: true,
        })
        .build()
        .expect("aieee, could not create ggez context!");

    let my_game = MyGame::new(&mut ctx);

    event::run(ctx, event_loop, my_game);
}


struct Paddle 
{
    x: f32,
    y: f32,
    mesh: Mesh,
}

impl Paddle 
{
    fn create_mesh (ctx: &mut Context) -> GameResult<Mesh>  
    {
        Mesh::new_rectangle(ctx, DrawMode::fill(), Rect::new(0.0, 0.0, 10.0, 100.0), Color::WHITE)
    }
}

struct Ball
{
    x: f32,
    y: f32,
    mesh: Mesh,
    velocity: [f32; 2],
}

impl Ball
{
    fn create_mesh (ctx: &mut Context) -> GameResult<Mesh> 
    {
        Mesh::new_circle(ctx, DrawMode::fill(), mint::Point2 { x: 0.0, y: 0.0 }, 10.0, 1.0, Color::WHITE)
    }

    fn start_ball () -> [f32; 2]
    {
        let mut rng = thread_rng();
        let random_value = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
        let velocity: [f32; 2] = [1.0*random_value, 0.0];
        return velocity;
    }
}

struct MyGame 
{
    left_block: Paddle,
    right_block: Paddle,
    ball: Ball,
}

impl MyGame 
{
    pub fn new(_ctx: &mut Context) -> MyGame 
    {
        MyGame 
        { 
            left_block: Paddle { x: 40.0, y: 0.0, mesh: Paddle::create_mesh(_ctx).unwrap() },
            right_block: Paddle { x: 750.0, y: 100.0, mesh: Paddle::create_mesh(_ctx).unwrap() },
            ball: Ball { x: 300.0, y: 200.0, mesh: Ball::create_mesh(_ctx).unwrap(), velocity: Ball::start_ball() },
        }
    }
}

impl EventHandler for MyGame 
{
   fn update(&mut self, ctx: &mut Context) -> GameResult 
    {
        let k_ctx = &ctx.keyboard;
        let screen_height = 600;

        if k_ctx.is_key_pressed(KeyCode::W) {
            if self.left_block.y > 0.0 {
                self.left_block.y -= 2.0;
            }
        }
        if k_ctx.is_key_pressed(KeyCode::S) {
            if f64::from(self.left_block.y) < f64::from(screen_height - 100) {
                self.left_block.y += 2.0;
            }    
        }

        if k_ctx.is_key_pressed(KeyCode::Up) {
            if self.right_block.y > 0.0 {
                self.right_block.y -= 2.0;
            }    
        }
        if k_ctx.is_key_pressed(KeyCode::Down) {
            if f64::from(self.right_block.y) < f64::from(screen_height - 100) {
                self.right_block.y += 2.0;
            }    
        }

        self.ball.x = self.ball.x + self.ball.velocity[0];
        self.ball.y = self.ball.y + self.ball.velocity[1];

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult 
    {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
     
        canvas.draw(&self.left_block.mesh, DrawParam::default().dest(glam::vec2(*&self.left_block.x, *&self.left_block.y)));
        canvas.draw(&self.right_block.mesh, DrawParam::default().dest(glam::vec2(*&self.right_block.x, *&self.right_block.y)));
        canvas.draw(&self.ball.mesh, DrawParam::default().dest(glam::vec2(*&self.ball.x, *&self.ball.y)));

        canvas.finish(ctx)
    }
}
