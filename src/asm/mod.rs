
mod parser;
use self::parser::*;

use std::ops::RangeFrom;
use super::{Register, sys_registers, transmute_slice};


use std::collections::HashMap;




const SYSTEM_REGISTERS : [(&'static str, Register); 13] = [
	("LHS",  sys_registers::LHS),
	("RHS",  sys_registers::RHS),
	("SUM",  sys_registers::SUM),
	("NOT",  sys_registers::NOT),
	("OUT",  sys_registers::OUT),
	("HALT",  sys_registers::HALT),
	("PC",  sys_registers::PC),
	("SKIP_INSTR",  sys_registers::SKIP_INSTR),
	("MEM_PTR",  sys_registers::MEM_PTR),
	("MEM_OFFSET",  sys_registers::MEM_OFFSET),
	("MEM_VALUE",  sys_registers::MEM_VALUE),
	("MEM_OBJECT_SIZE",  sys_registers::MEM_OBJECT_SIZE),
	("CODE_OBJECT",  sys_registers::CODE_OBJECT),
];

fn make_register_map<'a> (objects : &Vec<ObjectR<'a>>, registers: &mut RangeFrom<u16>) -> HashMap<RegisterR<'a>, Register> {
	let mut register_map : HashMap<RegisterR, Register> =
		SYSTEM_REGISTERS.iter().map(|&(a,b)|
			(RegisterR::System(Ident::from_str(a)), b)
		).collect();

	for object in objects {
		for &MoveR(ref src, ref dest) in &object.code {
			register_map.entry(src.clone()).or_insert_with(|| {
				Register(registers.next().unwrap())
			});
			register_map.entry(dest.clone()).or_insert_with(|| {
				Register(registers.next().unwrap())
			});
		}
	}

	register_map
}

fn make_label_map<'a>(objects: &Vec<ObjectR<'a>>) -> HashMap<Ident<'a>, i16> {
	let mut labels : HashMap<Ident, i16> = HashMap::new();
	let mut move_index = 0;
	for object in objects {
		labels.insert(object.label, move_index);
		move_index += object.code.len() as i16;
	}
	return labels;
}

fn make_register_file<'a>(last_id : u16, register_map: &HashMap<RegisterR<'a>, Register>, labels: HashMap<Ident<'a>, i16>) -> Vec<i16> {
	let mut registers = vec![0 as i16; last_id as usize];
	for (val, &Register(reg)) in register_map {
		if let &RegisterR::Immed(i) = val {
			registers[reg as usize] = i;
		}
		if let &RegisterR::Label(ref i) = val {
			registers[reg as usize] = labels[i];
		}
	}
	return registers;
}

fn encode_objects<'a> (objects: Vec<ObjectR<'a>>) -> Vec<i16> {
	
	let labels: HashMap<Ident, i16> = make_label_map(&objects);

	let mut register_ids = (SYSTEM_REGISTERS.len() as u16)..;
	let register_map = make_register_map(&objects, &mut register_ids);
	let last_id = register_ids.next().unwrap();
	let mut registers = make_register_file(last_id, &register_map, labels);

	let mut bytes : Vec<i16> = vec![];
	bytes.push(registers.len() as i16);
	bytes.append(&mut registers);

	for obj in objects {
		for MoveR(ref src, ref dest) in obj.code {
			let (Register(s), Register(d)) = (register_map[src], register_map[dest]);
			bytes.push(s as i16);
			bytes.push(d as i16);
		}
	}

	bytes

}



pub fn assemble(program: &str) -> Vec<u8> {

	let parsed = parse_code(program);
	
	let trans = encode_objects(parsed);



	let slice_u16 = &trans;
	let slice_ref = &slice_u16;

	let slice_u8 = unsafe {
		transmute_slice::<u8, i16>(slice_ref)
	};
	
	slice_u8.to_vec()
}
