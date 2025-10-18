use crate::parser::Parser;
use crate::transpiler::Transpiler;

mod transpiler;
mod parser;

fn main() {
    let lines: Vec<&str> = vec![
        "move :5",
        "move -1",
        "store +5",
    ];

    let mut transpiler: Transpiler = Transpiler::new();
    let mut parser: Parser = Parser::new(&mut transpiler);

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

    println!("{}", transpiler.output)
}
