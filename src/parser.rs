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
						self.transpiler.store_exact(value);
					} else if value_str.starts_with("+") {
						self.transpiler.store_add(value);
					} else if value_str.starts_with("-") {
						self.transpiler.store_sub(value);
					}
				}
			},

			// storec <char>
			//  store an ascii character at current cell
			"storec" => {
				let char_str = tokens.get(1).ok_or("Missing character argument")?;

				let c = match char_str.strip_suffix("!").unwrap_or(char_str) {
					"<space>" => ' ',
					_ => char_str.chars().next().ok_or("Invalid ASCII character")?
				};

				self.transpiler.store_char(c);
			},

			// put
			//  print value in current cell
			"put" => {
				self.transpiler.put();
			},

			// putw <cells>/*
			//  prints multiple cells at once
			//
			//  if the argument is a number it will print that many cells
			//  if its a star it will print until next null byte
			"putw" => {
				let cells_str = tokens.get(1).ok_or("Missing cells argument")?;

				if let Ok(cells) = cells_str.parse::<usize>() {
					self.transpiler.put_multiple(cells);
				} else if *cells_str == "*" {
					self.transpiler.put_until_null();
				}
			}

			// >
			//  shortcut to increment pointer by 1
			">" => {
				self.transpiler.move_to(self.transpiler.pointer + 1);
			}

			_ => {
				return Err("Unknown command")
			},
		}

		// automatically increment ptr by 1 if line ends with !, synthetic sugar
		if line.ends_with("!") {
			self.transpiler.move_to(self.transpiler.pointer + 1);
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