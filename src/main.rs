use std::{io, process};
use std::fs::File;
use std::io::{BufRead, Write};
use std::path::Path;
use std::time::Instant;
use crate::parser::Parser;
use crate::transpiler::Transpiler;

// modules
mod transpiler;
mod parser;

fn main() {
	let mut transpiler: Transpiler = Transpiler::new();
	let mut parser: Parser = Parser::new(&mut transpiler);

	let args: Vec<String> = std::env::args().skip(1).collect::<Vec<_>>();

	// if no file name supplied as first argument then show a usage message
	let input_file_path = match args.first() {
		Some(f) => f,
		None => {
			println!("Usage: lessbrainfuck <file> [output]");
			return;
		}
	};

	let output_file_path = args.get(1);

	// check if file exists
	if !check_file_exists(input_file_path) {
		println!("'{input_file_path}' is not a file");
		return;
	}

	// finally get the code after all the checks
	let lines: Vec<String> = match try_read_file_lines(input_file_path) {
		Some(l) => l,
		None => {
			println!("Failed to read from file");
			return;
		}
	};

	// timer
	let start = Instant::now();

	for (line_num, line) in lines.iter().enumerate() {
		{
			match parser.parse_line(&line) {
				Ok(_) => {},

				Err(err) => {
					println!("Error at line {}: {err}", line_num + 1);
					process::exit(1);
				},
			}
		}
	}

	let duration = start.elapsed();
	println!("\nTranspiled in {:.5} seconds", duration.as_secs_f64());
	
	let final_output_path = match output_file_path {
		Some(p) => p,
		None => &input_file_path.replace(".lbf", ".bf"),
	};

	if let Some(path) = output_file_path {
		write_to_file(path, &transpiler.output).expect("Failed to write output file");
	} else {
		write_to_file(&final_output_path, &transpiler.output).expect("Failed to write output file");
	}

	println!("Wrote to {final_output_path}");
}

fn try_read_file_lines(path: &str) -> Option<Vec<String>> {
	if let Ok(file) = File::open(path) {
		let lines = io::BufReader::new(file)
			.lines()
			.collect::<Result<Vec<_>, _>>()
			.ok()?;

		Some(lines)
	} else {
		None
	}
}

fn write_to_file(path: &str, content: &String) -> io::Result<()> {
	let mut file = File::create(path)?;
	file.write_all(content.as_bytes())?;
	Ok(())
}

fn check_file_exists(path: &str) -> bool {
	let path = Path::new(path);
	path.exists() && path.is_file()
}