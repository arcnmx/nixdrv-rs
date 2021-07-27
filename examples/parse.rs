use nixdrv::{parse_drv_bytes, Error};
use std::io::{self, Read};
use std::fs::File;
use std::env;

fn main() -> io::Result<()> {
	let mut args = env::args_os();
	let p = args.nth(1).expect("call with path");
	let mut f = File::open(p)?;
	let mut data = Vec::new();
	f.read_to_end(&mut data)?;

	let drv = parse_drv_bytes(&data[..])
		.map_err(|err| io::Error::new(io::ErrorKind::InvalidData, format!("{:?}", err)))?;
	println!("{:#?}", drv);

	Ok(())
}
