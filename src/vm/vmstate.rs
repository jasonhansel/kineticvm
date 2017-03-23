use super::super::sys_registers;
use super::super::Register;

use std::io::Write;
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
		match (self.register_read(sys_registers::LHS), self.register_read(sys_registers::RHS)) {
			(Value::Int(l), Value::Int(r)) => Value::Int(op(l, r)),
			_ => Value::Int(0)
		}
	}

	fn register_read(&self, register: Register) -> Value {
		// let registers = state.get_regs();
		match register {
			// Register::Immed(value) => Value::Int(value),
			sys_registers::MEM_VALUE => {
				let ptr = self.register_read(sys_registers::MEM_PTR);
				let offset = self.register_read(sys_registers::MEM_OFFSET);
				
				match (ptr, offset) {
					(Value::Ptr(p), Value::Int(o)) => {
						self.memory.get(p, o as u16)
					},
					_ => { panic!("Cannot read state!"); }
				}
			},
			sys_registers::SUM => {
				self.perform_operation(&|l, r| { l + r })
			},
			sys_registers::PRODUCT => {
				self.perform_operation(&|l, r| { l * r })
			},
			sys_registers::SHIFT_L => { self.perform_operation(&|l, r| { l << r}) },
			sys_registers::SHIFT_LR => { self.perform_operation(&|l, r| { ((l as u16) >> (r as u16)) as i16 }) },
			sys_registers::SHIFT_AR => { self.perform_operation(&|l, r| { l >> r }) },
			sys_registers::BIT_AND => { self.perform_operation(&|l, r| { l & r }) },
			sys_registers::BIT_OR => { self.perform_operation(&|l, r| { l | r }) },
			sys_registers::BIT_XOR => { self.perform_operation(&|l, r| { l ^ r }) },
			
			sys_registers::NOT => { self.perform_operation(&|l, _| { if l == 0 { 1 } else { 0 } }) },
			sys_registers::MEM_OBJECT_SIZE => {
				match self.register_read(sys_registers::MEM_PTR) {
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
				if let Value::Int(i) = self.register_read(sys_registers::OUT) {
					writeln!(output, "{}", i).unwrap();
				}
			},
			sys_registers::SKIP_INSTR => {
				if self.register_read(sys_registers::SKIP_INSTR) != Value::Int(0) {					
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
				let ptr = self.register_read(sys_registers::MEM_PTR);
				let offset = self.register_read(sys_registers::MEM_OFFSET);
				let value = self.memory.get_register(sys_registers::MEM_VALUE);
				// println!("Writing {:?} to {:?} by {:?}", value, ptr, offset);
				match offset {
					Value::Int(o) => {
						self.memory.set(MemoryPtr::try_from(ptr).unwrap(), o as u16, value);
					},
					_ => { panic!("Cannot write memory!"); }
				}
			},
			_ => {}
		}
	}

	fn increment_pc(&mut self) {
		// let registers = self.memory.get_regs();
		let next_pc = match self.register_read(sys_registers::PC) {
			Value::Int(i) => Value::Int(i + 1),
			_ => panic!("Invalid code pointer!")
		};
		self.memory.set_register(sys_registers::PC, next_pc)
	}



	fn get_current_instruction(&mut self) -> Option<(Register, Register)> {
		let pc = match self.register_read(sys_registers::PC) {
			Value::Int(i) => i as u16,
			_ => panic!("Invalid code pointer!")
		};

		if let Value::Ptr(code_obj) = self.register_read(sys_registers::CODE_OBJECT) {
			let source = self.memory.get_carefully(code_obj, pc * 2);
			let destination = self.memory.get_carefully(code_obj, pc * 2 + 1);
			if let (Some(Value::Int(s)), Some(Value::Int(d))) = (source, destination) {
				Some((Register(s as u16), Register(d as u16)))
			} else {
				None
			}
		} else {
			None
		}
	}


	pub fn run_cycle(&mut self, output: &mut Write) -> bool {
		if let Some((from, to)) = self.get_current_instruction() {
			let read = self.register_read(from);
			self.increment_pc();
			self.memory.set_register(to, read);
			self.update_registers(to, output);
			self.register_read(sys_registers::HALT) == Value::Int(0)
		} else {
			false
		}
	}

}