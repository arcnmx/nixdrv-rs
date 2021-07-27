use serde::{Serialize, Deserialize};
use nom::{IResult, Finish};
use nom::combinator::*;
use nom::bytes::streaming::*;
use nom::branch::*;
use nom::sequence::*;
use std::path::PathBuf;
use std::str;

pub mod de;

// Format is simple, like: https://github.com/pbogdan/nix-derivation-pretty

pub type Ident = String;

#[derive(Clone, Debug)]
pub enum Atom {
	Ident(Ident),
	String(Vec<u8>),
	Tuple(Vec<Atom>),
	List(Vec<Atom>),
	Apply(Box<Atom>, Box<Atom>),
}

fn atom_tag(data: &[u8]) -> IResult<&[u8], &[u8]> {
	alt((tag(b"\""), tag(b"("), tag(b"["), is_ident_start))(data)
}

fn drv_atom(data: &[u8]) -> IResult<&[u8], Atom> {
	map(terminated(pair(atom1, atom1), eof), |(lhs, rhs)|
		Atom::Apply(lhs.into(), rhs.into())
	)(data)
}

fn atom1(data: &[u8]) -> IResult<&[u8], Atom> {
	match peek(atom_tag)(data)?.1 {
		b"\"" => map(into(parse_string), Atom::String)(data),
		b"(" => map(parse_tuple, Atom::Tuple)(data),
		b"[" => map(parse_list, Atom::List)(data),
		_ => map(into(parse_ident), Atom::Ident)(data),
	}
}

fn atom_streaming(data: &[u8]) -> IResult<&[u8], Atom> {
	let (data, lhs) = atom1(data)?;

	Ok(if !peek(alt((atom_tag, success(&b""[..]))))(data)?.1.is_empty() {
		let (data, rhs) = atom_streaming(data)?;
		(data, Atom::Apply(Box::new(lhs), Box::new(rhs)))
	} else {
		(data, lhs)
	})
}

fn atom_complete(data: &[u8]) -> IResult<&[u8], Atom> {
	let (data, lhs) = atom1(data)?;

	Ok(if data.is_empty() {
		(data, lhs)
	} else {
		let (data, rhs) = atom_complete(data)?;
		(data, Atom::Apply(Box::new(lhs), Box::new(rhs)))
	})
}

fn is_ident_start(data: &[u8]) -> IResult<&[u8], &[u8]> {
	take_while_m_n(1, 1, is_ident)(data)
}

fn is_ident(c: u8) -> bool {
	c.is_ascii_alphabetic()
}

fn parse_ident(data: &[u8]) -> IResult<&[u8], &str> {
	map(take_while1(is_ident), |ident| unsafe {
		str::from_utf8_unchecked(ident)
	})(data)
}

fn parse_string(data: &[u8]) -> IResult<&[u8], &[u8]> {
	fn escape(data: &[u8]) -> IResult<&[u8], &[u8]> {
		preceded(tag(b"\\"), tag(b"\""))(data)
	}

	fn string_inner(mut data: &[u8]) -> IResult<&[u8], ()> {
		Ok((loop {
			let alt = alt((tag(b"\""), tag(b"\\"), take_while1(|c| c != b'"' && c != b'\\')))(data)?;
			let alt = match alt.1 {
				b"\"" => break data,
				b"\\" => escape(data)?,
				_ => alt,
			};
			data = alt.0;
		}, ()))
	}

	delimited(tag(b"\""), recognize(string_inner), tag(b"\""))(data)
}

fn list_inner(mut data: &[u8]) -> IResult<&[u8], Vec<Atom>> {
	let mut list = Vec::new();
	loop {
		let (new_data, atom) = atom_streaming(data)?;
		list.push(atom);
		data = new_data;

		let (new_data, sep) = alt((tag(b","), success(&b""[..])))(data)?;
		match sep {
			b"," => data = new_data,
			b"" => break Ok((data, list)),
			_ => unreachable!(),
		}
	}
}

fn parse_tuple(data: &[u8]) -> IResult<&[u8], Vec<Atom>> {
	delimited(tag(b"("), list_inner, tag(b")"))(data)
}

fn parse_list(data: &[u8]) -> IResult<&[u8], Vec<Atom>> {
	delimited(tag(b"["), list_inner, tag(b"]"))(data)
}

pub type OutputName = String;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Output {
	name: OutputName,
	path: PathBuf,
	hash_algo: String,
	hash: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Input {
	path: PathBuf,
	outputs: Vec<OutputName>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Derive {
	outputs: Vec<Output>,
	input_drvs: Vec<Input>,
	input_srcs: Vec<PathBuf>,
	platform: String,
	builder: String,
	args: Vec<String>,
	env: Vec<(String, String)>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Derivation {
	Derive(Derive),
}

pub fn parse_drv_atom_bytes(data: &[u8]) -> Result<Atom, Error> {
	drv_atom(data).finish().map(|(_, res)| res)
}

pub fn parse_drv_bytes(data: &[u8]) -> Result<Derive, de::Error> {
	let atom = parse_drv_atom_bytes(data)?;
	let mut de = de::Deserializer::new(&atom);
	Deserialize::deserialize(&mut de).map(|d| match d {
		Derivation::Derive(der) => der,
	})
}

pub type Error<'a> = nom::error::Error<&'a [u8]>;
