use crate::Atom;
use serde::de::{self, Visitor};
use std::marker::PhantomData;
use std::error::Error as StdError;
use std::{fmt, str, io};

//type Result<'a, T> = std::result::Result<T, Error<'a>>;

#[derive(Copy, Clone, Debug)]
pub struct Deserializer<'a, 'de> {
	atom: &'de Atom,
	_marker: PhantomData<&'a [u8]>,
}

impl<'a, 'de> Deserializer<'a, 'de> {
	pub fn new(atom: &'de Atom) -> Self {
		Self {
			atom,
			_marker: Default::default(),
		}
	}

	/*fn modify<T, F: FnOnce(Atom) -> (Atom, T)>(&mut self, f: F) -> T {
		let atom = self.atom.take().expect("internal error");
		let (atom, res) = f(atom);
		self.atom = Some(atom);
		res
	}*/
}

#[derive(Debug)]
pub enum Error<'a> {
	Io(io::Error),
	Parse(crate::Error<'a>),
}

impl<'a> From<crate::Error<'a>> for Error<'a> {
	fn from(err: crate::Error<'a>) -> Self {
		Error::Parse(err)
	}
}

impl<'a> From<io::Error> for Error<'a> {
	fn from(err: io::Error) -> Self {
		Error::Io(err)
	}
}

impl fmt::Display for Error<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		unimplemented!()
	}
}

impl StdError for Error<'_> { }

impl de::Error for Error<'_> {
	fn custom<T: fmt::Display>(msg: T) -> Self {
		unimplemented!()
	}
}

impl<'a, 'de, 'm> de::Deserializer<'de> for &'m mut Deserializer<'a, 'de> {
	type Error = Error<'a>;

	fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		match self {
			_ => unimplemented!(),
		}
	}

	fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		unimplemented!()
	}

	fn deserialize_i8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		unimplemented!()
	}

	fn deserialize_i16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		unimplemented!()
	}

	fn deserialize_i32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		unimplemented!()
	}

	fn deserialize_i64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		unimplemented!()
	}

	fn deserialize_u8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		unimplemented!()
	}

	fn deserialize_u16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		unimplemented!()
	}

	fn deserialize_u32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		unimplemented!()
	}

	fn deserialize_u64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		unimplemented!()
	}

	fn deserialize_f32<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value, Self::Error> {
		unimplemented!()
	}

	fn deserialize_f64<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value, Self::Error> {
		unimplemented!()
	}

	fn deserialize_char<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		match self.atom {
			Atom::String(buf) => match &buf[..] {
				&[b] if b.is_ascii() => visitor.visit_char(b as char),
				_ => unimplemented!(),
			},
			_ => unimplemented!(),
		}
	}

	fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		match self.atom {
			Atom::String(buf) => visitor.visit_borrowed_str(
				str::from_utf8(buf)
					.map_err(|e| -> Self::Error { unimplemented!() })?
			),
			_ => unimplemented!(),
		}
	}

	fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		self.deserialize_str(visitor)
	}

	fn deserialize_bytes<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		self.deserialize_byte_buf(visitor)
	}

	fn deserialize_byte_buf<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		match self.atom {
			Atom::String(buf) => visitor.visit_borrowed_bytes(buf),
			_ => unimplemented!(),
		}
	}

	fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		unimplemented!()
	}

	fn deserialize_unit<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		unimplemented!()
	}

	fn deserialize_unit_struct<V: Visitor<'de>>(
		self,
		_name: &'static str,
		visitor: V,
	) -> Result<V::Value, Self::Error> {
		self.deserialize_unit(visitor)
	}

	fn deserialize_newtype_struct<V: Visitor<'de>>(
		self,
		_name: &'static str,
		visitor: V,
	) -> Result<V::Value, Self::Error> {
		visitor.visit_newtype_struct(self)
	}

	fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		match self.atom {
			Atom::List(tup) => visitor.visit_seq(Seq::new(tup)),
			_ => unimplemented!(),
		}
	}

	fn deserialize_tuple<V: Visitor<'de>>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error> {
		match self.atom {
			Atom::Tuple(tup) if tup.len() != len => unimplemented!("expected {} vs {}", len, tup.len()),
			Atom::Tuple(tup) => visitor.visit_seq(Seq::new(tup)),
			_ => unimplemented!(),
		}
	}

	fn deserialize_tuple_struct<V: Visitor<'de>>(
		self,
		_name: &'static str,
		len: usize,
		visitor: V,
	) -> Result<V::Value, Self::Error> {
		self.deserialize_tuple(len, visitor)
	}

	fn deserialize_map<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		unimplemented!()
	}

	fn deserialize_struct<V: Visitor<'de>>(
		self,
		_name: &'static str,
		fields: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Self::Error> {
		match self.atom {
			Atom::Tuple(_) => {
				self.deserialize_tuple(fields.len(), visitor)
			},
			_ => unimplemented!(),
		}
	}

	fn deserialize_enum<V: Visitor<'de>>(
		self,
		_name: &'static str,
		_variants: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Self::Error> {
		match self.atom {
			Atom::Apply(k, v) => visitor.visit_enum(Enum(Deserializer::new(k), Deserializer::new(v))),
			_ => unimplemented!(),
		}
	}

	fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		match self.atom {
			Atom::Ident(id) => visitor.visit_borrowed_str(id),
			_ => unimplemented!("ident {:?}", self.atom),
		}
	}

	fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
		self.deserialize_any(visitor)
	}
}

struct Seq<'a, 'de> {
	elements: &'de [Atom],
	idx: usize,
	_marker: PhantomData<&'a [u8]>,
}

impl<'a, 'de> Seq<'a, 'de> {
	fn new(elements: &'de [Atom]) -> Self {
		Self {
			elements,
			idx: 0,
			_marker: PhantomData,
		}
	}
}

impl<'a, 'de> de::SeqAccess<'de> for Seq<'a, 'de> {
	type Error = Error<'a>;

	fn next_element_seed<T: de::DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error> {
		match self.elements.get(self.idx) {
			Some(e) => {
				let res = seed.deserialize(&mut Deserializer::new(e));
				self.idx += 1;
				res.map(Some)
			},
			None => Ok(None),
		}
	}

	/*fn next_element<T: de::Deserialize<'de>>(&mut self) -> Result<Option<T>, Self::Error> {
		match self.elements.get(self.idx) {
			Some(e) => {
				Deserializer::new(e)
			},
			None => unimplemented!(),
		}
	}*/

	fn size_hint(&self) -> Option<usize> {
		Some(self.elements.len().saturating_sub(self.idx))
	}
}

struct Enum<'a, 'de>(Deserializer<'a, 'de>, Deserializer<'a, 'de>);

impl<'a, 'de> de::EnumAccess<'de> for Enum<'a, 'de> {
	type Error = Error<'a>;
	type Variant = Variant<'a, 'de>;

	fn variant_seed<V: de::DeserializeSeed<'de>>(mut self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error> {
		let variant = Variant(self.1);
		seed.deserialize(&mut self.0).map(|value| (value, variant))
	}
}

struct Variant<'a, 'de>(Deserializer<'a, 'de>);

impl<'a, 'de> de::VariantAccess<'de> for Variant<'a, 'de> {
	type Error = Error<'a>;

	fn unit_variant(self) -> Result<(), Self::Error> {
		unimplemented!()
	}

	fn tuple_variant<V: Visitor<'de>>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error> {
		unimplemented!()
	}

	fn struct_variant<V: Visitor<'de>>(self, fields: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error> {
		unimplemented!()
	}

	fn newtype_variant_seed<T: de::DeserializeSeed<'de>>(mut self, seed: T) -> Result<T::Value, Self::Error> {
		seed.deserialize(&mut self.0)
	}
}
