use std::fs::File;
use std::io::Read;
use std::time::Duration;

use sdl2::audio::{AudioCallback, AudioSpecDesired};
use sdl2::pixels::Color;

use chip8::chip8::Chip8;

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

fn main() {
    // Chip-8 code.
    let rom: String = std::env::args().nth(1).expect("No ROM given");

    let rom_path: String = format!("{rom}");

    let mut file = File::open(rom_path).unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data).expect("Can't read bytes");

    let mut chip8 = Chip8::new();
    chip8.load_sprites();
    chip8.load_rom(&data);
	chip8.debug();

    println!("Data: {:?}", data);

    // SDL2 code.
    let sdl_context = sdl2::init().unwrap();
	let audio_subsystem = sdl_context.audio().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

	let desired_spec = AudioSpecDesired {
		freq: Some(44100),
		channels: Some(1),  // mono
		samples: None       // default sample size
	};

	let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
		SquareWave {
			phase_inc: 440.0 / spec.freq as f32,
			phase: 0.0,
			volume: 0.25
		}
	}).unwrap();

    let window = video_subsystem.window("Chip-8 Emu", 1024, 512)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

		if chip8.keypad.handle_input(&mut event_pump) {
			break 'running
		}
        
        // The rest of the game loop goes here...
		chip8.run();

		if chip8.cpu.dt > 0 {
			chip8.cpu.dt -= 1;
		}

		if chip8.cpu.st > 0 {
			device.resume();
			chip8.cpu.st -= 1;
		} else { device.pause(); }

		chip8.display.draw(&mut canvas);
        canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}