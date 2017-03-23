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


/// System registers
pub mod sys_registers {
	use super::Register;
	// Control flow
	pub const PC : Register = Register(7);
	pub const SKIP_INSTR : Register = Register(8);
	pub const CODE_OBJECT : Register = Register(0);

	// Input operands	
	pub const LHS : Register = Register(1);
	pub const RHS : Register = Register(2);

	// Arithmetic
	pub const SUM : Register = Register(3);
	pub const PRODUCT : Register = Register(13);
	pub const DIFF : Register = Register(20);
	pub const QUOTIENT : Register = Register(21);

	// Shifts
	pub const SHIFT_L : Register = Register(14);
	pub const SHIFT_LR : Register = Register(15);
	pub const SHIFT_AR : Register = Register(16);

	// Bitwise operations
	pub const NOT : Register = Register(4);
	pub const BIT_AND : Register = Register(17);
	pub const BIT_OR : Register = Register(18);
	pub const BIT_XOR : Register = Register(19);

	// Memory
	pub const MEM_PTR : Register = Register(9);
	pub const MEM_OFFSET : Register = Register(10);
	pub const MEM_VALUE : Register = Register(11);
	pub const MEM_OBJECT_SIZE : Register = Register(12);	

	// I/O
	pub const OUT : Register = Register(5);
	pub const HALT : Register = Register(6);


	// Number of system registers for which we allocate space
	pub const MAX_SYS_REGISTER : u16 = 128;
}


unsafe fn transmute_slice<'a, U, T>(slice: &'a [T]) -> &'a [U] {
	slice::from_raw_parts(
		slice.as_ptr() as *const U,
		(slice.len() * size_of::<T>()) / size_of::<U>()
	)
}


/// The entry point for the CLI.
fn main() {
	let mut args = env::args();

	match args.nth(1).as_ref().map(|x| &**x) {
		Some("assemble") => {
			// If the first argument is "assemble", we first get the filename
			// and read the file's contents into a string.
			let filename = args.next().unwrap();
			let mut code = String::new();
			let mut file = File::open(filename).unwrap();
			file.read_to_string(&mut code).unwrap();

			// Now we assemble the file's contents and write it to STDOUT.
			let object = assemble(&code);
			stdout().write(&object).unwrap();
		}
		Some("execute") => {
			// If the first argument is "execute", we first get the name of the file and
			// read the bytecode it contains into a string.
			let filename = args.next().unwrap();
			let mut object : Vec<u8> = Vec::new();
			let mut file = File::open(filename).unwrap();
			file.read_to_end(&mut object).unwrap();

			// Now we execute it, printing the program's output to STDOUT.
			execute_program(&object, &mut stdout());
		}
		_ => {
			// Print usage information.
			println!(r#"
Usage: kineticvm assemble <file>
       Assemble the code contained in <file>, printing the resulting bytecode to STDOUT.

       kineticvm execute <file>
       Executes the bytecode contained in <file>.
"#)
		}
	}



}
