//! Basic hello world example.

use ggez::event;
use ggez::event::ControlFlow;
use ggez::graphics;
use ggez::graphics::Image;
use ggez::{Context, GameResult, filesystem};
use std::cell::RefCell;
use std::env;
use std::io::Read;
use std::path;
use std::rc::Rc;
use std::sync::Arc;
use tego::Map;

fn game_to_tego_error(err: ggez::error::GameError) -> tego::Error {
    use ggez::error::GameError::*;
    match err {
        FilesystemError(_) => todo!(),
        ConfigError(_) => todo!(),
        EventLoopError(_) => todo!(),
        ResourceLoadError(_) => todo!(),
        ResourceNotFound(_, _) => todo!(),
        RenderError(_) => todo!(),
        AudioError(_) => todo!(),
        WindowError(_) => todo!(),
        WindowCreationError(_) => todo!(),
        IOError(e) => tego::Error::IO(std::io::Error::new(e.kind(), e.to_string())),
        FontError(_) => todo!(),
        VideoError(_) => todo!(),
        ShaderProgramError(_) => todo!(),
        GamepadError(_) => todo!(),
        LyonError(_) => todo!(),
        CustomError(_) => todo!(),
    }
}

fn tego_to_game_error(err: tego::Error) -> ggez::error::GameError {
    use tego::Error::*;
    use ggez::error::GameError;
    match err {
        StructureError { tag: _, msg: _ } => todo!(),
        ParseError(_) => todo!(),
        IO(e) => GameError::IOError(Arc::new(e)),
        UnsupportedFeature(_) => todo!(),
        PropertyTypeError => todo!(),
    }
}

struct GGEZProvider {
    ctx: Rc<RefCell<Context>>,
}

impl tego::Provider for GGEZProvider {
    fn read(&mut self, base_path: &str, path: &str) -> tego::Result<Vec<u8>> {
        let mut buffer = Vec::new();
        let mut file =
            filesystem::open(&self.ctx.borrow(), format!("{}/{}", base_path, path))
            .map_err(game_to_tego_error)?
        ;

        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
}

impl tego::ImageLoader for GGEZProvider {
    fn load(&mut self, path: &str) -> tego::Result<Box<dyn std::any::Any>> {
        let img = Image::new(&mut self.ctx.borrow_mut(), path).map_err(game_to_tego_error)?;
        Ok(Box::new(img))
    }
}

// First we make a structure to contain the game's state
struct MainState {
    frames: usize,
    text: graphics::Text,
    map: Map,
}

impl MainState {
    fn new(ctx: &mut Context, map: Map) -> GameResult<MainState> {
        // The ttf file will be in your resources directory. Later, we
        // will mount that directory so we can omit it in the path here.
        let font = graphics::Font::new(ctx, "/LiberationMono-Regular.ttf")?;
        let text = graphics::Text::new(("Hello world!", font, 48.0));

        let s = MainState { frames: 0, text, map };
        Ok(s)
    }
}

// Then we implement the `ggez:event::EventHandler` trait on it, which
// requires callbacks for updating and drawing the game state each frame.
//
// The `EventHandler` trait also contains callbacks for event handling
// that you can override if you wish, but the defaults are fine.
impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.frames += 1;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        // Drawables are drawn from their top-left corner.
        let offset = self.frames as f32 / 10.0;
        let dest_point = glam::Vec2::new(offset, offset);
        graphics::draw(ctx, &self.text, (dest_point,))?;

        for layer in self.map.iter_layers() {
            use tego::Layer::*;
            if let (Tile(tile_layer), _) = layer {
                for (pos, gid) in tile_layer.tiles_in_renderorder(&self.map).filter(|t| t.1.is_some()) {
                    let (img, src_rect) = self.map.tile_image(gid.unwrap()).unwrap();

                    let img = img.downcast_ref::<graphics::Image>().unwrap();
                    let w = img.width() as f32;
                    let h = img.height() as f32;

                    let pos = pos * self.map.tile_size;

                    let params = graphics::DrawParam::default()
                        .src(graphics::Rect::new(
                            src_rect.upper_left.x as f32 / w,
                            src_rect.upper_left.y as f32 / h,
                            src_rect.size.x as f32 / w,
                            src_rect.size.y as f32 / h,
                        ))
                        .dest(glam::vec2(pos.x as f32, pos.y as f32))
                    ;
                    graphics::draw(ctx, img, params)?;
                }
            }
        }

        graphics::present(ctx)?;

        if (self.frames % 100) == 0 {
            println!("FPS: {}", ggez::timer::fps(ctx));
        }

        Ok(())
    }
}

// Now our main function, which does three things:
//
// * First, create a new `ggez::ContextBuilder`
// object which contains configuration info on things such
// as screen resolution and window title.
// * Second, create a `ggez::game::Game` object which will
// do the work of creating our MainState and running our game.
// * Then, just call `game.run()` which runs the `Game` mainloop.
pub fn main() -> GameResult {
    // We add the CARGO_MANIFEST_DIR/resources to the resource paths
    // so that ggez will look in our cargo project directory for files.
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("helloworld", "ggez").add_resource_path(resource_dir);
    let (ctx, event_loop) = cb.build()?;

    let ctx = Rc::new(RefCell::new(ctx));

    let mut loader = tego::ResourceManager::new(GGEZProvider{ctx: ctx.clone()}, GGEZProvider{ctx:ctx.clone()});

    loader.set_base_path("/".to_string());
    let tmx = loader.load_text("main.tmx").map_err(tego_to_game_error)?;
    let map = Map::from_xml_str(&tmx, &mut loader).map_err(tego_to_game_error)?;

    let mut state = MainState::new(&mut ctx.borrow_mut(), map)?;

    // Handle events. Refer to `winit` docs for more information.
    event_loop.run(move |mut event, _window_target, control_flow| {
        if !ctx.borrow().continuing {
            *control_flow = ControlFlow::Exit;
            return;
        }

        *control_flow = ControlFlow::Poll;

        let ctx = &mut ctx.borrow_mut();

        // This tells `ggez` to update it's internal states, should the event require that.
        // These include cursor position, view updating on resize, etc.
        event::process_event(ctx, &mut event);
        use ggez::event::winit_event::{Event, KeyboardInput, WindowEvent};
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => event::quit(ctx),
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(keycode),
                            ..
                        },
                    ..
                } => {
                    if let event::KeyCode::Escape = keycode {
                        *control_flow = ControlFlow::Exit
                    }
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                // Tell the timer stuff a frame has happened.
                // Without this the FPS timer functions and such won't work.
                ctx.timer_context.tick();

                use event::EventHandler;

                // Update
                state.update(ctx).unwrap();

                // Draw
                state.draw(ctx).unwrap();

                // reset the mouse delta for the next frame
                // necessary because it's calculated cumulatively each cycle
                ctx.mouse_context.reset_delta();

                ggez::timer::yield_now();
            }
            _ => {}
        }
    });
}
