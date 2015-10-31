extern crate sdl2;
extern crate sdl2_image;
// #[macro_use] asks the compiler to import the macros defined
// in the events module. Macros cannot be namespaced and the
// expansion happens before namespaces exist.
// #[macro_use]
// mod events; 

mod phi;
mod views;

use ::phi::{Events, Phi, View, ViewAction};

fn main() {
    ::phi::spawn("ArcadeRS Shooter", |phi| {
        Box::new(::views::ShipView::new(phi))
    });
}
//     // Init SDL2
//     let sdl_context = sdl2::init().unwrap();
//     let video = sdl_context.video().unwrap();
//     let mut timer = sdl_context.timer().unwrap();
// 
//     // Create the window
//     let window = video.window("ArcadeRS Shooter", 800, 600)
//         .position_centered().opengl()
//         .build().unwrap();
// 
//     let mut context = Phi {
//         events: Events::new(sdl_context.event_pump().unwrap()),
//         renderer: window.renderer()
//             .accelerated()
//             .build().unwrap(),
//     };
// 
//     // Create the default view
//     let mut current_view: Box<View> = Box::new(::views::DefaultView);
// 
//     // Frame timing
//     let interval = 1_000 / 60;
//     let mut before = timer.ticks();
//     let mut last_second = timer.ticks();
//     let mut fps = 0u16;
// 
//     loop {
//         let now = timer.ticks();
//         let dt = now - before;
//         let elapsed = dt as f64 / 1_000.0;
// 
//         // if the time elapsed since the last frame is _too small_
//         // wait out the difference and then try again
//         if dt < interval {
//             timer.delay(interval - dt);
//             continue;
//         }
// 
//         before = now;
//         fps += 1;
// 
//         if now - last_second > 1_000 {
//             println!("FPS: {}", fps);
//             last_second = now;
//             fps = 0;
//         }
// 
//         context.events.pump();
// 
//         match current_view.render(&mut context, 0.01) {
//             ViewAction::None => context.renderer.present(),
//             ViewAction::Quit => break,
//         }
//     }
// }
