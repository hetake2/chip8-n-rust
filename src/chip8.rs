use crate::memory::Ram;
use crate::cpu::Cpu;
use crate::display::Display;
use crate::keypad::Keypad;

//use sdl2::{render::Canvas, video::Window};

pub const FONT_START: u16 = 0x50;
const FONT_END: u16 = 0xA0;

pub struct Chip8 {
    pub ram: Ram,
    pub cpu: Cpu,
	pub display: Display,
	pub keypad: Keypad,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            ram: Ram::new(),
            cpu: Cpu::new(),
			display: Display::new(),
			keypad: Keypad::new(),
        }
    }

	pub fn run(&mut self) {
		let instruction = self.get_instruction();
		self.cpu.run_instruction(instruction, &mut self.display, &mut self.ram, &mut self.keypad);
	} 

	pub fn get_instruction(&mut self) -> u16 {
		// High bits
        let hi = self.ram.read_byte(self.cpu.pc) as u16;
		// Low bits
        let lo = self.ram.read_byte(self.cpu.pc+1) as u16;
		// Opcode
        let instruction = (hi << 8) | lo;

		return instruction;
	}

    pub fn load_sprites(&mut self) {

        // Characters from 0 to F.
        let sprites: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
		];

        // Allocating all keypad sprites into memory.
        sprites.iter()
            .zip(FONT_START..FONT_END)
            .for_each(|(&byte, address)| {
                self.ram.write_byte(address, byte);
            });
    }

    pub fn load_rom(&mut self, data: &Vec<u8>) {
        let offset = 0x200;
        for i in 0..data.len() {
            self.ram.write_byte((offset + i) as u16, data[i]);
        }
	}
		
    pub fn debug(&mut self) {
        for i in 0..0x1FF {
            print!("{:#X} ", self.ram.memory[i]);
        }

        println!("");
    }
}