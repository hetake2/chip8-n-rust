use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::Canvas; 
use sdl2::video::Window;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
const SCALE_FACTOR: u32 = 16;

pub struct Display {
	pub screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
}

impl Display {
	pub fn new() -> Self {
		Self {
			screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT]
		}
	}

	pub fn reset(&mut self) {
		self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT]
	}

	pub fn draw(&self, canvas: &mut Canvas<Window>) {
		canvas.set_draw_color(Color::RGB(0, 255, 0));

		self.screen.iter().enumerate().for_each(|(i, &pixel_on)| {
			if pixel_on == true {
				let x = (i % SCREEN_WIDTH) as u32;
            	let y = (i / SCREEN_WIDTH) as u32;

				let rect = Rect::new(
					(x * SCALE_FACTOR) as i32,
					(y * SCALE_FACTOR) as i32,
					SCALE_FACTOR,
					SCALE_FACTOR,
				);

				canvas.fill_rect(rect).unwrap();
			}
		});
	} 
}