// #![feature(test)]

extern crate farmhash;

mod vm;
mod asm;

#[cfg(test)]
mod test;

use std::slice;
use std::fs::File;
use std::env;
use std::io::{Write, stdout, Read};
use std::mem::size_of;


use asm::assemble;
use vm::execute_program;


#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Register(pub u16);


mod sys_registers {
	use super::Register;
	pub const CODE_OBJECT : Register = Register(0);
	pub const LHS : Register = Register(1);
	pub const RHS : Register = Register(2);
	pub const SUM : Register = Register(3);
	pub const NOT : Register = Register(4);
	pub const OUT : Register = Register(5);
	pub const HALT : Register = Register(6);
	pub const PC : Register = Register(7);
	pub const SKIP_INSTR : Register = Register(8);
	pub const MEM_PTR : Register = Register(9);
	pub const MEM_OFFSET : Register = Register(10);
	pub const MEM_VALUE : Register = Register(11);
	pub const MEM_OBJECT_SIZE : Register = Register(12);	
}

unsafe fn transmute_slice<'a, U, T>(slice: &'a [T]) -> &'a [U] {
	slice::from_raw_parts(
		slice.as_ptr() as *const U,
		(slice.len() * size_of::<T>()) / size_of::<U>()
	)
}




fn main() {
	let mut args = env::args();
	// let program_file = args.nth(1).unwrap();

	match args.nth(1).as_ref().map(|x| &**x) {
		Some("assemble") => {
			let program_file = args.next().unwrap();
			let mut program_code = String::new();

			let mut file = File::open(program_file).unwrap();
			file.read_to_string(&mut program_code).unwrap();
			stdout().write(&assemble(&program_code)).unwrap();

		}
		Some("execute") => {
			let program_file = args.next().unwrap();
			let mut program_code : Vec<u8> = Vec::new();

			let mut file = File::open(program_file).unwrap();
			file.read_to_end(&mut program_code).unwrap();

			execute_program(&program_code, &mut stdout());
			
		}
		_ => {
			println!(r#"
Usage: ttavm execute <file>
       Executes the assembly code contained in <file>.
"#)
		}
	}



}
