use crate::transpiler::Transpiler;

pub struct Parser<'a> {
	transpiler: &'a mut Transpiler
}

impl<'a> Parser<'a> {
	pub fn new(transpiler: &'a mut Transpiler) -> Self {
		Self {
			transpiler
		}
	}

	pub fn parse_line(&mut self, line: &str) -> Result<(), &str> {
		let trimmed = line.trim();

		// if line empty
		if trimmed.is_empty() {
			return Err("Line empty")
		}

		// tokens are just the line split by wh
		let tokens: Vec<&str> = line.split_whitespace().collect();

		let command = tokens.first().ok_or("No command bruh??")?;

		match *command {
			// move :/+/-<cell>
			//  moves the pointer to a cell
			//
			// prefixes:
			//  : = move to exact cell
			//  + = increment pointer
			//  - = decrement pointer
			"move" => {
				let cell_str = tokens.get(1).ok_or("Missing argument (cell to move to)")?;

				let no_prefix = try_remove_num_prefix(cell_str).ok_or("Invalid number prefix")?;

				if let Ok(cell) = no_prefix.parse::<usize>() {
					// : = move exact
					// + = increment
					// - = decrement
					if cell_str.starts_with(":") {
						self.transpiler.move_to(cell);
					} else if cell_str.starts_with("+") {
						self.transpiler.move_to(self.transpiler.pointer + cell);
					} else if cell_str.starts_with("-") {
						self.transpiler.move_to(self.transpiler.pointer - cell);
					}
				}
			}

			// store :/+/-<value>
			//  store value at current cell
			//
			// prefixes:
			//  : = store exact value
			//  + = increment value
			//  - = decrement value
			"store" => {
				let value_str = tokens.get(1).ok_or("Missing value argument")?;

				let no_prefix = try_remove_num_prefix(value_str).ok_or("Invalid number prefix")?;

				if let Ok(value) = no_prefix.parse::<usize>() {
					// : = move exact
					// + = increment
					// - = decrement
					if value_str.starts_with(":") {
						self.transpiler.store_exact(value)
					} else if value_str.starts_with("+") {
						self.transpiler.store_add(value)
					} else if value_str.starts_with("-") {
						self.transpiler.store_sub(value)
					}
				}
			},
			
			// put
			//  print value in current cell
			"put" => {
				self.transpiler.put();
			}

			_ => {
				return Err("Unknown command")
			},
		}

		Ok(())
	}
}

// helper function to remove the :/+/- prefix in move and load commands
// either returns the number with the prefix or none if the number doesnt have any of the prefixes
fn try_remove_num_prefix(num: &str) -> Option<&str> {
	let no_prefix = if let Some(rest) = num.strip_prefix(":") {
		rest
	} else if let Some(rest) = num.strip_prefix("+") {
		rest
	} else if let Some(rest) = num.strip_prefix("-") {
		rest
	} else {
		return None;
	};

	Some(no_prefix)
}