use super::Register;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct MemoryPtr(u16);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct MemorySize(pub u16);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Value {
    Ptr(MemoryPtr),
    Int(i16)
}

impl Default for Value {
	fn default() -> Value {
		Value::Int(0)
	}
}



impl MemoryPtr {
	pub fn try_from(v: Value) -> Result<MemoryPtr, ()> {
		match v {
			Value::Ptr(u) => Ok(u),
			Value::Int(_) => Err(())
		}
	}
}

impl From<MemoryPtr> for Value {
	fn from(p : MemoryPtr) -> Value {
		return Value::Ptr(p);
	}
}

impl MemorySize {
	pub fn try_from(v: Value) -> Result<MemorySize, ()> {
		match v {
			Value::Ptr(_) => Err(()),
			Value::Int(i) => Ok(MemorySize(i as u16))
		}
	}
}




pub struct Memory {
	data: Vec<u16>,
	pub root: MemoryPtr,
	top: MemoryPtr
}

fn get_full_size(size: u16) -> u16 {
	return 1 + size + (1 + (size >> 4));
}



impl Memory {
	pub fn new(root_size: u16) -> Memory {
		let mut mem = Memory {
			data: vec![],
			root: MemoryPtr(0),
			top: MemoryPtr(0)
		};
		mem.root = mem.allocate_values(MemorySize(root_size));
		return mem;
	}

	pub fn get_register(&self, Register(r): Register) -> Value {
		let root = self.root;
		return self.get(root, r)
	}

	pub fn set_register(&mut self, Register(r): Register, value: Value) {
		let root = self.root;
		self.set(root, r, value)
	}

	pub fn get(&self, ptr: MemoryPtr, offset: u16) -> Value {
		return self.get_carefully(ptr, offset).unwrap()
	}

	pub fn get_carefully(&self, ptr: MemoryPtr, offset: u16) -> Option<Value> {
		let MemoryPtr(ptr) = ptr;
		let ptr = ptr as usize;
		let offset = offset as usize;

		let total_offset = ptr + 1 + offset;

		if let Some(&val) = self.data.get(total_offset) {
			let object_size = self.data[ptr] as usize;
			if offset >= object_size {
				None
			} else {
				let type_ptr = ptr + 1 + object_size + (offset >> 4);
				if self.data[type_ptr] & (1 << (offset & 0xf)) == 0 {
					Some(Value::Int(val as i16))
				} else {
					Some(Value::Ptr(MemoryPtr(val as u16)))
				}
			}
		} else {
			None
		}
	}

	pub fn set(&mut self, ptr: MemoryPtr, offset: u16, value: Value) {

		let MemoryPtr(ptr) = ptr;
		let ptr = ptr as usize;
		

		let total_offset : usize = ptr + (offset as usize) + 1;
		let mask = 1 << (offset & 0xf);
		let obj_size = self.data[ptr] as usize;

		if offset >= (obj_size as u16) {
			panic!("Tried to set with invalid offset!")
		}

		let type_ptr : usize = ptr + 1 + obj_size + ((offset >> 4) as usize);

		match value {
			Value::Ptr(MemoryPtr(val)) => {
				self.data[total_offset] = val;
				self.data[type_ptr] |= mask;
			},
			Value::Int(val) => {
				self.data[total_offset] = val as u16;
				self.data[type_ptr] &= !mask;
			}
		}
	}


	pub fn write_raw(&mut self, ptr: MemoryPtr, values: &[u16]) {
		let MemoryPtr(ptr) = ptr;
		let ptr = ptr as usize;
		self.data[(ptr+1)..(ptr+1+values.len())].clone_from_slice(values);
		
	}



	pub fn allocate_values(&mut self, size: MemorySize) -> MemoryPtr {
		let MemorySize(sz) = size;
		let MemoryPtr(t) = self.top;

		self.data.extend(vec![0; get_full_size(sz) as usize]);
		self.data[t as usize] = sz as u16;
		let pointer = t;
		self.top = MemoryPtr(pointer + get_full_size(sz));
		MemoryPtr(pointer)
	}
}
