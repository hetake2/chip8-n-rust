// Chip8's memory

pub struct Ram {
   pub memory: [u8; 4096]
}

// Ram's methods.
impl Ram {
    pub fn new() -> Self {
        Self {
            memory: [0; 4096]
        }

        // println!("RAM before 0x1FF:");

        // for i in 0..0x1FF {
        //     print!("{:#X} ", ram.memory[i]);
        // }

        // println!("");

        // return ram;
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}