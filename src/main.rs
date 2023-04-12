mod lexer;

use crate::lexer::{ParseError, Lexeme};

use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::result::Result;
use std::string::String;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const PROMPT: &str = "4th: ";


/**
 * This is a FORTH interpreter based on the introduction at www.forth.com:
 *
 * https://www.forth.com/starting-forth/1-forth-stacks-dictionary/
 */
fn main() -> Result<(), Box<dyn Error>> {
    println!("mini-forth {}", VERSION);
    let mut rl = DefaultEditor::new()?;
    let mut stack: VecDeque<Lexeme> = VecDeque::new();
    let mut dictionary: HashMap<Lexeme, VecDeque<Lexeme>> = HashMap::new();

    if rl
        // This is a hack, obviously a history file should go somewhere better than this.
        .load_history("/tmp/mini-forth_history")
        .is_err()
    {
        println!("No Previous History");
    }

    loop {
        let readline = rl.readline(PROMPT);
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())
                    .expect("Error adding to history");

                match lex(String::from(line.as_str())) {
                    Ok(words) => {
                        for word in words {
                            stack.push_back(word);
                        }
                    }
                    Err(ParseError::UnknownWord(word)) => {
                        println!("Got here");
                        println!("{}?", word);
                    }
                    Err(ParseError::UnmatchedComment(word)) => {
                        println!("Should've started your comment correctly: {}", word);
                    }
                }

                println!("{:?}", stack);
            }
            Err(ReadlineError::Interrupted) => {
                println!("ctrl+c");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("ctrl+d");
                break;
            }
            Err(err) => {
                println!("Error: {:?})", err);
                break;
            }
        }
    }

    // Same hack as above, this should go in a real history file.
    rl.save_history("/tmp/mini-forth_history")
        .expect("Failed to write history");
    Ok(())
}
