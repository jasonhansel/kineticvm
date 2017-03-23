// must enable beta for this

// extern crate test;
use vm::execute_program;
// use test::test::Bencher;
use asm::assemble;


fn output_for_code(code: &str) -> String {
	let mut output : Vec<u8> = vec![];
	let asm = assemble(code);
	execute_program(&asm, &mut output);
	return String::from_utf8(output).unwrap();
}

macro_rules! test_program {
	( $x:ident ) => {
		#[test]
		fn $x() {
			let code_str = include_str!(concat!("../tests/", stringify!($x), ".s"));
			let output = include_str!(concat!("../tests/", stringify!($x), ".out"));
			assert_eq!(output_for_code(code_str), output);
		}
	}
}

macro_rules! bench_program {
	( $x:ident ) => {
		

		
			// extern crate test;
			// use super::output_for_code;
			// use test::test::Bencher;
			
			#[bench]
			fn bench(b : &mut Bencher) {
				let code_str = include_str!(concat!("../tests/", stringify!($x), ".s"));
				let output = include_str!(concat!("../tests/", stringify!($x), ".out"));
				b.iter(|| {
					let o = output_for_code(code_str);
					assert_eq!(o, output);
					o
				});
			}
		
	}
}

test_program!(addition);
test_program!(arithmetic);
test_program!(label_addr);
test_program!(loops);
test_program!(memory);
test_program!(memory_types);
test_program!(factorial);
test_program!(bitwise);