use ::phi::{Phi, View, ViewAction};
use ::phi::data::Rectangle;
use ::phi::gfx::{Sprite, CopySprite};

use ::views::shared::Background;

use ::sdl2::pixels::Color;

pub struct MainMenuView {
	actions: Vec<Action>,
	selected: i8, //? Use an i8 (0..) so we don't decrement below 0
	elapsed: f64,
	
	bg_back: Background,
	bg_middle: Background,
	bg_front: Background,
}

impl MainMenuView {
	pub fn new(phi: &mut Phi) -> MainMenuView {
		MainMenuView {
			actions: vec![
				Action::new(phi, "New Game", Box::new(|phi| {
					ViewAction::ChangeView(Box::new(::views::game::ShipView::new(phi)))
				})),
				Action::new(phi, "Quit", Box::new(|_| {
					ViewAction::Quit
				})),
			],
			//? Start with nothing selected.
			selected: 0,
			elapsed: 0.0,
			
			bg_back: Background {
				pos: 0.0,
				vel: 20.0,
				sprite: Sprite::load(&mut phi.renderer, "assets/starBG.png").unwrap(),
			},
			bg_middle: Background {
				pos: 0.0,
				vel: 40.0,
				sprite: Sprite::load(&mut phi.renderer, "assets/starMG.png").unwrap(),
			},
			bg_front: Background {
				pos: 0.0,
				vel: 80.0,
				sprite: Sprite::load(&mut phi.renderer, "assets/starFG.png").unwrap(),
			},
		}
	}
}

impl View for MainMenuView {
	fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
		if phi.events.now.quit || phi.events.now.key_escape == Some(true) {
			return ViewAction::Quit;
		}

		// Execute the currently selected option
		if phi.events.now.key_space == Some(true) || phi.events.now.key_return == Some(true) {
			//? Using the (self.attr_which_is_a_closure)(phi) syntax so that rust
			//? doesn't confuse it with an invocation of a function named `func`.
			//?
			//? Necessary because Rust allows a method to share the same name as an
			//? attribute of a struct. Alledgedly useful for defining accessors.
			return (self.actions[self.selected as usize].func)(phi);
		}
		
		// Change the selected action based on input
		if phi.events.now.key_up == Some(true) {
			self.selected -= 1;
			//? Handle looping on the list of actions
			if self.selected < 0 {
				self.selected = self.actions.len() as i8 - 1;
			}
		}
		if phi.events.now.key_down == Some(true) {
			self.selected += 1;
			//? Handle looping on the list of actions
			if self.selected >= self.actions.len() as i8 {
				self.selected = 0;
			}
		}
		
		// Clear the screen.
		phi.renderer.set_draw_color(Color::RGB(0,0,0));
		phi.renderer.clear();

		// Render the backgrounds
		self.bg_back.render(&mut phi.renderer, elapsed);
		self.bg_middle.render(&mut phi.renderer, elapsed);
		self.bg_front.render(&mut phi.renderer, elapsed);
		
		let (win_w,win_h) = phi.output_size();
		let label_h = 50.0;
		let border_width = 3.0;
		
		self.elapsed += elapsed * 4.0;
		let margin_h = 10.0 + 5.0 * (self.elapsed + 1.0).sin();	
		let box_w = 360.0 + 5.0 * self.elapsed.sin();
		
		let box_h = self.actions.len() as f64 * label_h;
		
		
		// Render the border of the coloured box
		phi.renderer.set_draw_color(Color::RGB(70,15,70));
		phi.renderer.fill_rect(Rectangle {
			w: box_w + border_width * 2.0,
			h: box_h + border_width * 2.0 + margin_h * 2.0,
			x: (win_w as f64 - box_w) / 2.0 - border_width,
			y: (win_h as f64 - box_h) / 2.0 - margin_h - border_width,
		}.to_sdl().unwrap());
		
		// Render the coloured box which holds the labels
		phi.renderer.set_draw_color(Color::RGB(140,30,140));
		phi.renderer.fill_rect(Rectangle {
			w: box_w,
			h: box_h + margin_h * 2.0,
			x: (win_w as f64 - box_w) / 2.0,
			y: (win_h as f64 - box_h) / 2.0 - margin_h,
		}.to_sdl().unwrap());
		
		for (i, action) in self.actions.iter().enumerate() {
			if self.selected as usize == i {
				let (w,h) = action.hover_sprite.size();
				phi.renderer.copy_sprite(&action.hover_sprite, Rectangle {
					x: (win_w as f64 - w) / 2.0,
					//? Place Every element under the previous one
					y: (win_h as f64 - box_h + label_h - h) / 2.0 + label_h * i as f64,
					w: w,
					h: h,
				});			
			} else {
				let (w,h) = action.idle_sprite.size();
				phi.renderer.copy_sprite(&action.idle_sprite, Rectangle {
					x: (win_w as f64 - w) / 2.0,
					//? Place Every element under the previous one
					y: (win_h as f64 - box_h + label_h - h) / 2.0 + label_h * i as f64,
					w: w,
					h: h,
				});				
			}

		}

		ViewAction::None
	}
}

struct Action {
	/// The function which should be executed if the action is chosen.
	//? Stored in a `Box` because `Fn` is a trait, so we can only interact
	//? with it via a pointer.
	func: Box<Fn(&mut Phi) -> ViewAction>,

	/// The sprite which is rendered when not the focus
	idle_sprite: Sprite,

	/// The sprite for when in focus
	hover_sprite: Sprite,
}

impl Action {
	fn new(phi: &mut Phi, label: &'static str, func: Box<Fn(&mut Phi) -> ViewAction>) -> Action {
		Action {
			func: func,
			idle_sprite: phi.ttf_str_sprite(label, "assets/belligerent.ttf", 32, Color::RGB(220,220,220)).unwrap(),
			hover_sprite: phi.ttf_str_sprite(label, "assets/belligerent.ttf", 38, Color::RGB(255,255,255)).unwrap(),
		}
	}
}