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
	}

	// storing
	pub fn store_exact(&mut self, value: usize) {
		self.output.push_str("[-]");
		self.output.push_str(&"+".repeat(value));
	}

	pub fn store_add(&mut self, value: usize) {
		self.output.push_str(&"+".repeat(value));
	}

	pub fn store_sub(&mut self, value: usize) {
		self.output.push_str(&"-".repeat(value));
	}

	// printing
	pub fn put(&mut self) {
		self.output.push_str(".");
	}
	
	pub fn get_output(&mut self) -> String {
		self.output.clone()
	}
}