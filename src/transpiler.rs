use std::collections::HashMap;

pub struct Transpiler {
	pub pointer: usize,
	cell_names: HashMap<String, String>,
	pub output: String,
}

impl Transpiler {
	pub fn new() -> Self {
		Self {
			pointer: 0,
			cell_names: HashMap::new(),
			output: String::new(),
		}
	}

	// ----------------- BASIC COMMANDS -----------------
	pub fn move_to(&mut self, target: usize) {
		if target > self.pointer {
			self.output.push_str(&">".repeat(target - self.pointer));
		} else {
			self.output.push_str(&"<".repeat(self.pointer - target));
		}
		self.pointer = target;
		println!("{} ", self.pointer);
	}

	// storing
	pub fn store_exact(&mut self, value: usize) {
		self.output.push_str("[-]");
		self.output.push_str(&"+".repeat(value));
	}

	pub fn store_vec(&mut self, v: Vec<u8>) {
		for byte in v {
			self.store_exact(byte as usize);
			self.move_to(self.pointer + 1);
		}
	}

	pub fn store_add(&mut self, value: usize) {
		self.output.push_str(&"+".repeat(value));
	}

	pub fn store_sub(&mut self, value: usize) {
		self.output.push_str(&"-".repeat(value));
	}

	pub fn store_char(&mut self, c: char) {
		let ascii = c as u8 as usize;
		self.store_exact(ascii);
	}

	// printing
	pub fn put(&mut self) {
		self.output.push('.');
	}

	pub fn put_multiple(&mut self, n: usize) {
		for _ in 0..n {
			self.put();
			self.move_to(self.pointer + 1);
		}
	}

	pub fn put_until_null(&mut self) {
		self.output.push_str("[.>]");
	}

	// reading
}