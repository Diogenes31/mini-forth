mod lexer;

use crate::lexer::LexError;

use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

use std::error::Error;
use std::result::Result;
use std::string::String;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/**
 * This is a FORTH interpreter based on the introduction at www.forth.com:
 *
 * https://www.forth.com/starting-forth/1-forth-stacks-dictionary/
 */
fn main() -> Result<(), Box<dyn Error>> {
    println!("mini-forth {}", VERSION);
    let mut rl = DefaultEditor::new()?;
    let mut lexer = lexer::Lexer::new();

    if rl
        // This is a hack, obviously a history file should go somewhere better than this.
        .load_history("/tmp/mini-forth_history")
        .is_err()
    {
        println!("No Previous History");
    }

    loop {
        let readline = rl.readline("");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())
                    .expect("Error adding to history");

                match lexer.lex(String::from(line.as_str())) {
                    Ok(()) => {}
                    Err(LexError::UnmatchedComment(word)) => {
                        println!("Should've started your comment correctly: {}", word);
                    }
                }
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
