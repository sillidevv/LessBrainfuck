use std::time::Instant;
use crate::parser::Parser;
use crate::transpiler::Transpiler;

mod transpiler;
mod parser;

fn main() {
	let lines: Vec<&str> = vec![
		"storec h!",
		"storec e!",
		"storec l!",
		"storec l!",
		"storec o!",
		"storec <space>!",
		"storec w!",
		"storec o!",
		"storec r!",
		"storec l!",
		"storec d!",
		"storec !!",
		"move :0",
		"putw *",
	];

	let mut transpiler: Transpiler = Transpiler::new();
	let mut parser: Parser = Parser::new(&mut transpiler);

	let start = Instant::now();

	for (_line_num, line) in lines.iter().enumerate() {
		{
			match parser.parse_line(&line) {
				Ok(_) => {},

				Err(err) => {
					println!("{}", err)
				},
			}
		}
	}

	println!("{}", transpiler.output);

	let duration = start.elapsed();
	println!("Transpiled in {:.5} seconds", duration.as_secs_f64());
}
