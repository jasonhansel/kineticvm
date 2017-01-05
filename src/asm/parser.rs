use self::grammar::*;


extern crate farmhash;
use farmhash::hash64;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Ident(u64);

impl Ident {
	pub fn from_str(str: &str) -> Ident {
		Ident(hash64(str.as_bytes()))
	}
}

mod grammar {
    include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}

#[derive(Clone, Debug)]
pub struct ObjectR {
	pub label: Ident,
	pub code: Vec<MoveR>
}

#[derive(Clone, Debug)]
pub struct MoveR(pub RegisterR, pub RegisterR);

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum RegisterR {
	Immed(i16),
	User(Ident),
	System(Ident),
	Label(Ident)
}


pub fn parse_code(code: &str) -> Vec<ObjectR> {
	program(code).unwrap()
}
