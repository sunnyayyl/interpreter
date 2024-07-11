use std::io::{self, Write};

use lexer::Lexer;

pub struct Repl {
    pub prompt: String,
}
impl Repl {
    pub fn new(prompt: String) -> Self {
        Self { prompt }
    }
    fn read_line() -> io::Result<String> {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        Ok(buffer)
    }
    pub fn start(&self) {
        loop {
            print!("{}", self.prompt);
            io::stdout().flush().expect("Failed to flush stdout");
            let user_input = Self::read_line();
            let input = user_input.expect("Failed to read stdin");
            if input.is_empty() {
                break;
            } else {
                let l = Lexer::new(input);
                for t in l.into_iter() {
                    println!("{:#?}", t);
                }
            }
        }
    }
}
