use super::super::sys_registers;
use super::super::Register;

use std::io::{Read,Write};
use std::cmp;
use super::memory::*;



pub struct VMState {
	memory: Memory
}

impl VMState {

	pub fn new(memory: Memory) -> VMState {
		VMState {
			memory: memory
		}
	}

	fn perform_operation(&self, op : &Fn(i16, i16) -> i16) -> Value {
		match (self.register_read(sys_registers::LHS, None), self.register_read(sys_registers::RHS, None)) {
			(Value::Int(l), Value::Int(r)) => Value::Int(op(l, r)),
			_ => Value::Int(0)
		}
	}

	fn register_read(&self, register: Register, input: Option<&mut Read>) -> Value {
		// let registers = state.get_regs();
		match register {
			// Register::Immed(value) => Value::Int(value),
			sys_registers::IO_CHAR => {
				let mut bytes : Vec<u8> = vec![0];
				let result = input.and_then(|r| 
					{ r.read_exact(&mut bytes).map(|_| bytes[0]).ok() }
				).unwrap_or(4);
				return Value::Int(result as i16);
			},
			sys_registers::MEM_VALUE => {
				let ptr = self.register_read(sys_registers::MEM_PTR, None);
				let offset = self.register_read(sys_registers::MEM_OFFSET, None);
				
				match (ptr, offset) {
					(Value::Ptr(p), Value::Int(o)) => {
						self.memory.get(p, o as u16)
					},
					_ => { panic!("Cannot read state!"); }
				}
			},
			sys_registers::SUM => { self.perform_operation(&|l, r| { l + r }) },
			sys_registers::EQ => {
				Value::Int(if self.register_read(sys_registers::LHS, None) == self.register_read(sys_registers::RHS, None) {
					1
				} else {
					0
				})
			},
			sys_registers::DIFF => { self.perform_operation(&|l, r| { l - r }) },
			sys_registers::PRODUCT => { self.perform_operation(&|l, r| { l * r }) },
			sys_registers::QUOTIENT => { self.perform_operation(&|l, r| {
				if (l % r).abs() >= (r / 2) {
					(l / r) + {
						if (l / r) < 0 {
							-1
						} else {
							1
						}
					}
				} else {
					l / r
				}
			}) },
			sys_registers::SHIFT_L => { self.perform_operation(&|l, r| { l << r}) },
			sys_registers::SHIFT_LR => { self.perform_operation(&|l, r| { ((l as u16) >> (r as u16)) as i16 }) },
			sys_registers::SHIFT_AR => { self.perform_operation(&|l, r| { l >> r }) },
			sys_registers::BIT_AND => { self.perform_operation(&|l, r| { l & r }) },
			sys_registers::BIT_OR => { self.perform_operation(&|l, r| { l | r }) },
			sys_registers::BIT_XOR => { self.perform_operation(&|l, r| { l ^ r }) },
			sys_registers::BIT_NOT => { self.perform_operation(&|l, _| { !l }) },
			sys_registers::MAX => { self.perform_operation(&|l, r| { cmp::max(l, r) }) },
			sys_registers::MIN => { self.perform_operation(&|l, r| { cmp::min(l, r) }) },
			
			sys_registers::NOT => { self.perform_operation(&|l, _| { if l == 0 { 1 } else { 0 } }) },
			sys_registers::MEM_OBJECT_SIZE => {
				match self.register_read(sys_registers::MEM_PTR, None) {
					Value::Ptr(p) => {
						let MemorySize(s) = self.memory.get_size(p);
						Value::Int(s as i16)
					},
					_ => Value::Int(0)
				}
			},
			u => self.memory.get_register(u)
		}
	}

	fn update_registers(&mut self, changed: Register, output: &mut Write) {
		match changed {
			sys_registers::OUT => {
				if let Value::Int(i) = self.register_read(sys_registers::OUT, None) {
					writeln!(output, "{}", i).unwrap();
					output.flush().unwrap();
				}
			},
			sys_registers::IO_CHAR => {
				if let Value::Int(i) = self.memory.get_register(sys_registers::IO_CHAR) {
					write!(output, "{}", char::from(i as u8)).unwrap();
					output.flush().unwrap();
				}
			},
			sys_registers::SKIP_INSTR => {
				if self.register_read(sys_registers::SKIP_INSTR, None) != Value::Int(0) {					
					self.memory.set_register(sys_registers::SKIP_INSTR, Value::Int(0));
					self.increment_pc();
				}
			},
			sys_registers::MEM_OBJECT_SIZE => {
				let v = self.memory.get_register(sys_registers::MEM_OBJECT_SIZE);
				let ptr = self.memory.allocate_values(MemorySize::try_from(v).unwrap());
				self.memory.set_register(sys_registers::MEM_PTR, Value::Ptr(ptr));
			},
			sys_registers::MEM_VALUE => {
				let ptr = self.register_read(sys_registers::MEM_PTR, None);
				let offset = self.register_read(sys_registers::MEM_OFFSET, None);
				let value = self.memory.get_register(sys_registers::MEM_VALUE);
				// println!("Writing {:?} to {:?} by {:?}", value, ptr, offset);
				match offset {
					Value::Int(o) => {
						self.memory.set(MemoryPtr::try_from(ptr).unwrap(), o as u16, value);
					},
					_ => { panic!("Cannot write memory!"); }
				}
			},
			sys_registers::LINK => {
				let new_pc = self.register_read(sys_registers::PC, None);
				let new_link = self.register_read(sys_registers::LINK, None);
				self.memory.set_register(sys_registers::PC, new_link);
				self.memory.set_register(sys_registers::LINK, new_pc);
			}
			_ => {}
		}
	}

	fn increment_pc(&mut self) {
		// let registers = self.memory.get_regs();
		let next_pc = match self.register_read(sys_registers::PC, None) {
			Value::Int(i) => Value::Int(i + 1),
			_ => panic!("Invalid code pointer!")
		};
		self.memory.set_register(sys_registers::PC, next_pc)
	}



	fn get_current_instruction(&mut self) -> Option<(Register, Register)> {
		let pc = match self.register_read(sys_registers::PC, None) {
			Value::Int(i) => i as u16,
			_ => panic!("Invalid code pointer!")
		};

		if let Value::Ptr(code_obj) = self.register_read(sys_registers::CODE_OBJECT, None) {
			let source = self.memory.get_carefully(code_obj, pc * 2);
			let destination = self.memory.get_carefully(code_obj, pc * 2 + 1);
			if let (Some(Value::Int(s)), Some(Value::Int(d))) = (source, destination) {
				Some((Register(s as u16), Register(d as u16)))
			} else if (None, None) == (source, destination) {
				None // Halt!
			} else {
				panic!("Pointers in code are invalid!");
			}
		} else {
			panic!("CODE_OBJECT must be a pointer!");
		}
	}


	pub fn run_cycle(&mut self, input: &mut Read, output: &mut Write) -> bool {
		if let Some((from, to)) = self.get_current_instruction() {
			let read = self.register_read(from, Some(input));
			self.increment_pc();
			self.memory.set_register(to, read);
			self.update_registers(to, output);
			self.register_read(sys_registers::HALT, None) == Value::Int(0)
		} else {
			false
		}
	}

}