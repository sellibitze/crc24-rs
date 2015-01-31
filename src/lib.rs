//! Crate provoding a CRC-24 hasher based on IETF RFC2440 specification.

#![feature(hash)]

use std::default::Default;
use std::hash::{ self, Hasher };

const INIT: u32 = 0xB704CE;

include! { concat!(env!("OUT_DIR"), "/table.inc") }

/// CRC-24 hasher based on IETF RFC2440 specification.
#[derive(Copy,Clone,PartialEq,Eq)]
pub struct Crc24Hasher {
	state: u32
}

impl Crc24Hasher {
	/// creates a new CRC-24 hasher initialized with the given state
	pub fn init(v: u32) -> Crc24Hasher {
		Crc24Hasher { state: v & 0xFFFFFF }
	}
}

impl Default for Crc24Hasher {
	/// creates a new CRC-24 hasher initialized with a nonzero state
	/// specified in RFC2240
	fn default() -> Crc24Hasher {
		Crc24Hasher { state: INIT }
	}
}

impl Hasher for Crc24Hasher {
	type Output = [u8; 3];

	/// resets the state to the special nonzero value specified RFC2240
	fn reset(&mut self) { self.state = INIT; }

	fn finish(&self) -> [u8; 3] {
		[(self.state >> 16) as u8,
		 (self.state >>  8) as u8,
		  self.state        as u8]
	}
}

impl hash::Writer for Crc24Hasher {
	fn write(&mut self, msg: &[u8]) {
		let mut s = self.state;
		for &octet in msg.iter() {
			let index = ((octet as u32) ^ (s >> 16)) & 0xFF;
			s = (s << 8) ^ CRC24_TABLE[index as usize];
		}
		self.state = s & 0xFFFFFF;
	}
}

