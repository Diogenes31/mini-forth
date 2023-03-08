use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use std::collections::VecDeque;
use std::string::String;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const PROMPT: &str = "4th: ";

fn main() -> Result<()> {
    println!("mini-forth {}", VERSION);
    let mut rl = DefaultEditor::new()?;
    let mut stack: VecDeque<String> = VecDeque::new();

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
                print!("[");

                let stack_iter = stack.iter();
                for value in stack_iter {
                    print!("\"{}\", ", value);
                }

                stack.push_back(String::from(line.as_str()));
                println!(
                    "\"{}\"]",
                    stack
                        .back()
                        .expect("Something has gone terribly wrong during stack construction")
                );
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
