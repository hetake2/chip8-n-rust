use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

use std::collections::HashMap;

pub struct Keypad {
	pub keys: [bool; 16],
}

impl Keypad {
	pub fn new() -> Self {
		Self {
			keys: [false; 16]
		}
	}

	fn get_keymap(&mut self) -> HashMap<Keycode, usize> {
		let mut map = HashMap::new();

		map.insert(Keycode::Num1, 0x1);
		map.insert(Keycode::Num2, 0x2);
		map.insert(Keycode::Num3, 0x3);
		map.insert(Keycode::Num4, 0xC);
		map.insert(Keycode::Q, 0x4);
		map.insert(Keycode::W, 0x5);
		map.insert(Keycode::E, 0x6);
		map.insert(Keycode::R, 0xD);
		map.insert(Keycode::A, 0x7);
		map.insert(Keycode::S, 0x8);
		map.insert(Keycode::D, 0x9);
		map.insert(Keycode::F, 0xE);
		map.insert(Keycode::Z, 0xA);
		map.insert(Keycode::X, 0x0);
		map.insert(Keycode::C, 0xB);
		map.insert(Keycode::V, 0xF);

		return map;
	}

	pub fn handle_input(&mut self, event_pump: &mut EventPump) -> bool {
		for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return true;
                },

				Event::KeyDown { keycode, .. } => {
					if let Some(key) = keycode {
						if let Some(&key_index) = self.get_keymap().get(&key) {
							self.keys[key_index] = true;
						}
					}
				},

				Event::KeyUp { keycode, .. } => {
					if let Some(key) = keycode {
						if let Some(&key_index) = self.get_keymap().get(&key) {
							self.keys[key_index] = false;
						}
					}
				},
                _ => {}
            }
        }

		return false;
	}
}