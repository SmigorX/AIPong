use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Mesh, Rect, DrawMode, DrawParam};
use ggez::event::{self, EventHandler};
use ggez::mint;
use ggez::glam;

fn main() 
{
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("pong", "SmigorX")
        .window_setup(ggez::conf::WindowSetup {
            title: "My Game".to_owned(),
            vsync: true,
            icon: "".to_owned(),
            srgb: true,
            samples: ggez::conf::NumSamples::One,
        })
        .window_mode(ggez::conf::WindowMode {
            width: 800.0,  // Set the desired width of the window
            height: 600.0, // Set the desired height of the window
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

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}


struct Object 
{
    x: f32,
    y: f32,
    mesh: Mesh,
}

impl Object 
{
    pub fn new(x: f32, y: f32, mesh: Mesh) -> Self 
    {
        Object {x, y, mesh}
    }
}


fn create_rectangle_mesh (ctx: &mut Context) -> GameResult<Mesh>  
{
    Mesh::new_rectangle(ctx, DrawMode::fill(), Rect::new(0.0, 0.0, 10.0, 100.0), Color::WHITE)
}

fn create_circle_mesh (ctx: &mut Context) -> GameResult<Mesh> 
{
    Mesh::new_circle(ctx, DrawMode::fill(), mint::Point2 { x: 0.0, y: 0.0 }, 10.0, 1.0, Color::WHITE)
}


struct MyGame 
{
    left_block: Object,
    right_block: Object,
    ball: Object,
}

impl MyGame 
{
    pub fn new(_ctx: &mut Context) -> MyGame 
    {
        let rectangle = create_rectangle_mesh(_ctx).unwrap();
        let circle = create_circle_mesh(_ctx).unwrap();

        // Load/create resources such as images here.
        MyGame 
        { 
            left_block: Object { x: 50.0, y: 50.0, mesh: rectangle.clone() },
            right_block: Object { x: 750.0, y: 100.0, mesh: rectangle },
            ball: Object { x: 300.0, y: 200.0, mesh: circle },
        }
    }
}

impl EventHandler for MyGame 
{
   fn update(&mut self, _ctx: &mut Context) -> GameResult 
    {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult 
    {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
     
        //canvas.set_screen_coordinates(Rect::new(100.0, 100.0, 0.0, 0.0));
        //let (width, height) = canvas.screen_coordinates(ctx);
        //println!("Window dimensions: {} x {}", width, height);

        canvas.draw(&self.left_block.mesh, DrawParam::default().dest(glam::vec2(*&self.left_block.x, *&self.left_block.y)));
        canvas.draw(&self.right_block.mesh, DrawParam::default().dest(glam::vec2(*&self.right_block.x, *&self.right_block.y)));
        canvas.draw(&self.ball.mesh, DrawParam::default().dest(glam::vec2(*&self.ball.x, *&self.ball.y)));

        // Draw code here...
        canvas.finish(ctx)
    }
}
