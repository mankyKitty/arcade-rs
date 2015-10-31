#[macro_use]
mod events;
pub mod data;
pub mod gfx;

use ::sdl2::render::Renderer;

// We cannot call functions at top-level.
// However, `struct_events` is a macro!
struct_events!(
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down,
        key_left: Left,
        key_right: Right,
        key_space: Space
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
}

impl<'window> Phi<'window> {
    fn new(events: Events, renderer: Renderer<'window>) -> Phi<'window> {
			::sdl2_image::init(::sdl2_image::INIT_PNG);

			Phi {
				events: events,
				renderer: renderer,
			}
		}

    pub fn output_size(&self) -> (u32,u32) {
        self.renderer.output_size().unwrap()
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

 
