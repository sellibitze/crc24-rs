#![feature(io)]
#![feature(os)]
#![feature(path)]
#![feature(core)]

use std::os;
use std::old_io::{ File, Writer };

const INC_FILE: &'static str = "table.inc";
const CRC24_POLY: u32 = 0x864CFB; // CRC-24 (IETF RFC2440), used by OpenPGP

fn main() {
	let dst = Path::new(os::getenv("OUT_DIR").unwrap());
	let mut f = File::create(&dst.join(INC_FILE)).unwrap();
	f.write_str(&*into_code(table_gen())).unwrap();
}

fn table_gen() -> Vec<u32> {
	let mut v = Vec::new();
	for hi in 0..256u32 {
		let mut temp = hi << 16;
		for _ in 0..8 {
			let x = if (temp & 0x800000) == 0 { 0 } else { CRC24_POLY };
			temp = ((temp & 0x7FFFFF) << 1) ^ x;
		}
		v.push(temp);
	}
	v
}

fn into_code(tab: Vec<u32>) -> String {
	let mut out: Vec<u8> = Vec::new();
	writeln!(&mut out, "const CRC24_TABLE: [u32; 256] = [").unwrap();
	for row in tab.chunks(4) {
		writeln!(&mut out, "\t0x{:06x}, 0x{:06x}, 0x{:06x}, 0x{:06x},",
			row[0], row[1], row[2], row[3]).unwrap();
	}
	writeln!(&mut out, "];").unwrap();
	String::from_utf8(out).unwrap()
}

