
mod memory;
use std::io::Write;
use super::{sys_registers, transmute_slice};

use self::memory::*;
mod vmstate;
use self::vmstate::*;




fn bytes_to_program(bytes: &[u8]) -> Program {
	let words = unsafe { transmute_slice::<u16, u8>(&bytes) };
	let reg_count = words[0] as usize;
	let registers = &words[1..(1+reg_count)];
	let code = &words[(1+reg_count)..];
	Program {
		registers: registers,
		code: code
	}
}

fn init_memory(program: Program) -> Memory {
	let mut mem = Memory::new(program.registers.len() as u16);
	let c = mem.allocate_values(MemorySize(program.code.len() as u16));
	let r = mem.root;
	mem.write_raw(c, program.code);
	mem.write_raw(r, program.registers);
	mem.set_register(sys_registers::CODE_OBJECT, Value::Ptr(c));
	mem
}


fn execute(program: Program, output: &mut Write) {
	let mut vm = VMState::new(init_memory(program));
	while vm.run_cycle(output) {}
}

#[derive(Clone, Debug)]
struct Program<'a> {
	code: &'a [u16],
	registers: &'a [u16]
}




pub fn execute_program(program: &[u8], output: &mut Write) {
    let p = bytes_to_program(program);
	execute(p, output);
}
