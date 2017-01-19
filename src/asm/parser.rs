use self::grammar::*;


#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Ident<'a>(&'a str);

impl<'a> Ident<'a> {
	pub fn from_str(str: &str) -> Ident {
		Ident(str)
	}
}

#[cfg(not(doc))]
mod grammar {
    include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}

#[cfg(doc)]
mod grammar {
	use super::ObjectR;
	pub fn program(code: &str) -> Option<Vec<ObjectR>> {
		None
	}
}

#[derive(Clone, Debug)]
pub struct ObjectR<'a> {
	pub label: Ident<'a>,
	pub code: Vec<MoveR<'a>>
}

#[derive(Clone, Debug)]
pub struct MoveR<'a>(pub RegisterR<'a>, pub RegisterR<'a>);

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum RegisterR<'a> {
	Immed(i16),
	User(Ident<'a>),
	System(Ident<'a>),
	Label(Ident<'a>)
}


pub fn parse_code(code: &str) -> Vec<ObjectR> {
	program(code).unwrap()
}
