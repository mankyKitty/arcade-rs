#[macro_use]
mod events;
pub mod data;
pub mod gfx;

use ::std::path::Path;
use ::std::collections::hash_map::HashMap;

use ::phi::gfx::Sprite;

use ::sdl2::render::Renderer;
use ::sdl2::pixels::Color;

// We cannot call functions at top-level.
// However, `struct_events` is a macro!
struct_events!(
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down,
        key_left: Left,
        key_right: Right,
        key_space: Space,
        key_return: Return
    },
    else: {
        quit: Quit { .. }
    }
);

/// Bundles the Phi abstractions in a single structure which
/// can be passed around more easily.
pub struct Phi<'a> {
    pub events: Events,
    pub renderer: Renderer<'a>,
		cached_fonts: HashMap<(&'static str, i32), ::sdl2_ttf::Font>,
}

impl<'window> Phi<'window> {
    fn new(events: Events, renderer: Renderer<'window>) -> Phi<'window> {
			::sdl2_image::init(::sdl2_image::INIT_PNG);

			Phi {
				events: events,
				renderer: renderer,
				cached_fonts: HashMap::new(),
			}
		}

    pub fn output_size(&self) -> (u32,u32) {
        self.renderer.output_size().unwrap()
    }

		pub fn ttf_str_sprite(&mut self, text: &str, font_path: &'static str, size: i32, color: Color) -> Option<Sprite> {
			//? First, we verify whether the font is already cached. If this is the
			//? case, we use it to render the text
			if let Some(font) = self.cached_fonts.get(&(font_path, size)) {
				return font.render(text, ::sdl2_ttf::blended(color)).ok()
					.and_then(|s| self.renderer.create_texture_from_surface(&s).ok())
					.map(Sprite::new)
			}
			//? Start by trying to load the font
			::sdl2_ttf::Font::from_file(Path::new(font_path), size).ok()
				.and_then(|font| {
					//? If this works, we cache the font we acquired
					self.cached_fonts.insert((font_path, size), font);
					//? Then, we call the method recursively. Because we know that
					//? the font has been cached, the `if` block will be executed
					self.ttf_str_sprite(text, font_path, size, color)
				})
				//? Next steps must be wrapped in a closure because of the
				//? borrow checker. `font` must live at least until the texture 
				//? is created.
				//? .and_then(|font| font
					//? If this worked, we try to create a surface from the font.
				//? 	.render(text, ::sdl2_ttf::blended(color)).ok()
					//? If THIS worked, we try to make this surface into a texture.
				//? 	.and_then(|surf| self.renderer.create_texture_from_surface(&surf).ok()
					//? if *THIS* worked, we can load
				//? 	.map(Sprite::new))
		}
}

impl<'window> Drop for Phi<'window> {
	fn drop(&mut self) {
		::sdl2_image::quit();
	}
}
/// A `ViewAction` is a way for the currently executed view to
/// communicate with the game loop. It specifies which action
/// should be executed before the next rendering.
pub enum ViewAction {
    None,
    Quit,
    ChangeView(Box<View>),
}

pub trait View {
    /// Called on every fame to take care of both the logic and 
    /// the rendering of the current view.
    /// 
    /// `elapsed` is expressed in seconds.
    fn render(&mut self, context: &mut Phi, elapsed: f64) -> ViewAction;
}

/// Create a window name `title`, init the underlying libs,
/// start the game with the `View` returned by `init()`.
///
/// # Examples
///
/// Here, we simply show a window with color #ffff00 and exit
/// when escape is pressed or window is closed
///
/// ```
/// struct MyView;
///
/// impl View for MyView {
///   fn render(&mut self, cxt: &mut Phi, _:f64) -> ViewAction {
///     if cxt.events.now.quit {
///       return ViewAction::Quit;
///     }
///     cxt.renderer.set_draw_color(Color::RGB(255,255,0));
///     cxt.renderer.clear();
///     ViewAction::None
///   }
/// }
///
/// spawn("Example", |_| {
///   Box::new(MyView)
/// });
/// ```
pub fn spawn<F>(title: &str, init: F) where F: Fn(&mut Phi) -> Box<View> {
    // Init SDL2
    let sdl_context = ::sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let mut timer = sdl_context.timer().unwrap();
		let _ttf_context = ::sdl2_ttf::init();

    // Create the window
    let window = video.window(title, 800, 600)
        .position_centered().opengl().resizable()
        .build().unwrap();

    // Create the context
    let mut context = Phi::new(
        Events::new(sdl_context.event_pump().unwrap()),
        window.renderer().accelerated().build().unwrap(),
    );

    // Create the default view
    let mut current_view = init(&mut context);
    // Frame timing 
    let interval = 1_000 / 60;
    let mut before = timer.ticks();
    let mut last_second = timer.ticks();
    let mut fps = 0u16;

    loop {
        // Frame timing (bis)
        let now = timer.ticks();
        let dt = now - before;
        let elapsed = dt as f64 / 1_000.0;
        // If the time elapsed since last frame is too small
        // wait out the diff and try again
        if dt < interval {
            timer.delay(interval - dt);
            continue;
        }

        before = now;
        fps += 1;

        if now - last_second > 1_000 {
            println!("FPS: {}", fps);
            last_second = now;
            fps = 0;
        }
        // Pass the renderer to the pump to handle window resizing.
        context.events.pump(&mut context.renderer);

        match current_view.render(&mut context, elapsed) {
            ViewAction::None => 
                context.renderer.present(),

            ViewAction::Quit => 
                break,

            ViewAction::ChangeView(new_view) =>
                current_view = new_view,
        }
    }
}

 
