use crate::chip8::FONT_START;
use crate::memory::Ram;
use crate::keypad::Keypad;
use crate::display::{Display, SCREEN_WIDTH, SCREEN_HEIGHT};

const SPRITE_WIDTH: usize = 8;

pub struct Cpu {
    // CPU registers V0-VF
    pub vx: [u8; 16],
    // Index register
    pub index: u16,
    // Program counter
    pub pc: u16,
    // Stack
    pub stack: [u16; 16],
    // Stack pointer
    pub sp: u16,
	// Sound timer
	pub st: u8,
	// Delay timer
	pub dt: u8,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            vx: [0; 16],
            index: 0,
            pc: 0x200, // PROGRAM START address
            stack: [0; 16],
            sp: 0,
			st: 0,
			dt: 0,
        }
    }

    pub fn run_instruction(&mut self, instruction: u16, display: &mut Display, ram: &mut Ram, keypad: &mut Keypad) {

        // Chip-8's opcodes symbols:
        let nnn = instruction & 0xFFF; // address, A 12-bit value, the lowest 12 bits of the instruction
        let nn = (instruction & 0x0FF) as u8; // An 8-bit value, the lowest 8 bits of the instruction
        let n = (instruction & 0x00F) as u8; // A 4-bit value, the lowest 4 bits of the instruction
        let x = ((instruction & 0xF00) >> 8) as usize; // A 4-bit value, the lower 4 bits of the high byte of the instruction
        let y = ((instruction & 0x0F0) >> 4) as usize; // A 4-bit value, the upper 4 bits of the low byte of the instruction
        
        //("Running instruction: {:#X}", instruction);
        
        match (instruction & 0xF000) >> 12 {
			0x0 => {
				match nn {
					0xE0 => {
						//("opcode: 00E0");
						display.reset();
						self.pc += 2;
					}

					0xEE => {
						//("opcode: 00EE");
						self.sp -= 1;
						self.pc = self.stack[self.sp as usize];
					}

					_ => panic!("Unrecognized instruction: {:#X} at {:#X}", instruction, self.pc)
				}
			}

            0x1 => {
                //("opcode: 1NNN");
                self.pc = nnn;
            }

            0x2 => {
                //("opcode: 2NNN");
                self.stack[self.sp as usize] = self.pc + 2;
                self.sp += 1;
                self.pc = nnn;
            }

            0x3 => {
                //("opcode: 3XNN");
                if self.vx[x] == nn {
                    self.pc += 4;
                } else { self.pc += 2; }
            }

            0x4 => {
                //("opcode: 4XNN");                
                if self.vx[x] != nn {
                    self.pc += 4;
                } else { self.pc += 2; }
            }

            0x5 => {
                //("opcode: 5XY0");
                if self.vx[x] == self.vx[y] {
                    self.pc += 4;
                } else { self.pc += 2; }
            }
            
            0x6 => {
                //("opcode: 6XNN");
                self.vx[x] = nn;
				self.pc += 2;
            }

            0x7 => {
                //("opcode: 7XNN");
                self.vx[x] = self.vx[x].wrapping_add(nn);
				self.pc += 2;
            }

            0x8 => {
                match n {
                    0x0 => {
                        //("opcode: 8XY0");
                        self.vx[x] = self.vx[y];
						self.pc += 2;
                    }
                
                    0x1 => {
                        //("opcode: 8XY1");
                        self.vx[x] = self.vx[x] | self.vx[y];
						self.pc += 2;
                    }
                
                    0x2 => {
                        //("opcode: 8XY2");
                        self.vx[x] = self.vx[x] & self.vx[y];   
						self.pc += 2;
                    }
                
                    0x3 => {
                        //("opcode: 8XY3");
                        self.vx[x] = self.vx[x] ^ self.vx[y];
						self.pc += 2;                        
                    }
                
                    0x4 => {
                        //("opcode: 8XY4");
                        let (result, carry) = self.vx[x].overflowing_add(self.vx[y]);
                        self.vx[x] = result; 
                        self.vx[15] = if carry { 0x1 } else { 0x0 }; 
						self.pc += 2;
                    }

                    0x5 => {
                        //("opcode: 8XY5");
                        let (result, borrow) = self.vx[x].overflowing_sub(self.vx[y]); 
                        self.vx[x] = result; 
                        self.vx[15] = if borrow { 0x0 } else { 0x1 };
						self.pc += 2;
                    }

                    0x6 => {
                        //("opcode: 8XY6");
						let tmp = self.vx[x] & 0x1;
                        self.vx[x] >>= 1;
						self.vx[15] = tmp;
						self.pc += 2;
                    }

                    0x7 => {
                        //("opcode: 8XY7");
                        let (result, borrow) = self.vx[y].overflowing_sub(self.vx[x]); 
                        self.vx[x] = result; 
                        self.vx[15] = if borrow { 0x0 } else { 0x1 };
						self.pc += 2;
                    }

                    0xE => {
                        //("opcode: 8XYE");
						let tmp = (self.vx[x] & 0x80) >> 7;
                        self.vx[x] <<= 1;
						self.vx[15] = tmp;
						self.pc += 2;
                    }

                    _ => panic!("Unrecognized instruction: {:#X} at {:#X}", instruction, self.pc)
                }
            }

            0x9 => {
                //("opcode: 9XY0");
                if self.vx[x] != self.vx[y] {
					self.pc += 4
				} else { self.pc += 2;}
            }

            0xA => {
                //("opcode: ANNN");
                self.index = nnn;
				self.pc += 2;
            }

            0xB => {
                //("opcode: BNNN");
                self.pc = nnn + self.vx[0] as u16;
            }

            0xC => {
                //("opcode: CXNN");
                self.vx[x] = rand::random::<u8>() & nn;
				self.pc += 2;
            }

            0xD => {
                //("opcode: DXYN");
				for y_offset in 0..n {
					let byte = ram.read_byte(self.index + y_offset as u16);

					for x_offset in 0..SPRITE_WIDTH {
						if (byte >> (7 - x_offset)) & 1 == 1 {
							let x_coord = (self.vx[x] + x_offset as u8) as usize % SCREEN_WIDTH;
							let y_coord = (self.vx[y] + y_offset) as usize % SCREEN_HEIGHT;

							let pixel = (y_coord * SCREEN_WIDTH) + x_coord;

							if display.screen[pixel] == true {
								self.vx[15] = 1;
							}

							display.screen[pixel] ^= true;
						}
					}
				}
				self.pc += 2;
            }

			0xE => {
				match nn {
					0x9E => {
						//("opcode: EX9E");
						if keypad.keys[self.vx[x] as usize] {
							self.pc += 4;
						} else { self.pc += 2; }
					}

					0xA1 => {
						//("opcode: EXA1");
						if !keypad.keys[self.vx[x] as usize] {
							self.pc += 4;
						} else { self.pc += 2; }
					}

					_ => panic!("Unrecognized instruction: {:#X} at {:#X}", instruction, self.pc)
				}
			}

			0xF => {
				match nn {
					0x07 => {
						//("opcode: FX07");
						self.vx[x] = self.dt;
						self.pc += 2;
					}

					0x0A => {
						//("opcode: FX0A");
						if let Some(key) = keypad.keys.iter().position(|&key| key == true) {
							self.vx[x] = key as u8;
							self.pc += 2;
						}
					}

					0x15 => {
						//("opcode: FX15");
						self.dt = self.vx[x];
						self.pc += 2;
					}

					0x18 => {
						//("opcode: FX18");
						self.st = self.vx[x];
						self.pc += 2;
					}

					0x1E => {
						//("opcode: FX1E");
						self.index = self.index + self.vx[x] as u16;
						self.pc += 2;
					}

					0x29 => {
						//("opcode: FX29");
						let digit = self.vx[x] as u16;
						self.index = FONT_START + (digit * 5);
						self.pc += 2;
					}

					0x33 => {
						//("opcode: FX33");
						let value = self.vx[x];
						ram.write_byte(self.index, value / 100);
						ram.write_byte(self.index + 1, (value / 10) % 10);
						ram.write_byte(self.index + 2, value % 10);
						self.pc += 2;
					}

					0x55 => {
						//("opcode: FX55");
						for i in 0..=x {
							ram.write_byte(self.index + i as u16, self.vx[i]);
						}

						self.index += (x + 1) as u16;
						self.pc += 2;
					}

					0x65 => {
						//("opcode: FX65");
						for i in 0..=x {
							self.vx[i] = ram.read_byte(self.index + i as u16);
						}

						self.index += (x + 1) as u16;
						self.pc += 2;
					}

					_ => panic!("Unrecognized instruction: {:#X} at {:#X}", instruction, self.pc)
				}
			}

            _ => panic!("Unrecognized instruction: {:#X} at {:#X}", instruction, self.pc)
        }
    }
}