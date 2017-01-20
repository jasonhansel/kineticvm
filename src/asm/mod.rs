
mod parser;
use self::parser::*;
use std::ops::RangeFrom;
use super::{sys_registers, transmute_slice};
use std::collections::HashMap;


fn add_to_register_map<'a>(register_map: &mut HashMap<RegisterR<'a>, Register>, register: RegisterR<'a>, registers: &mut RangeFrom<u16>) {
	if !register_map.contains_key(&register) {
		let new_register = match register {
			RegisterR::System(u) => u,
			_ => Register(registers.next().unwrap())
		};
		register_map.insert(register, new_register);
	}
}

fn make_register_map<'a> (objects : &Vec<ObjectR<'a>>, registers: &mut RangeFrom<u16>) -> HashMap<RegisterR<'a>, Register> {
	let mut register_map : HashMap<RegisterR, Register> = HashMap::new();
	for object in objects {
		for &MoveR(ref src, ref dest) in &object.code {
			add_to_register_map(&mut register_map, src.clone(), registers);
			add_to_register_map(&mut register_map, dest.clone(), registers);
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

	let mut register_ids = (sys_registers::MAX_SYS_REGISTER + 1)..;
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
