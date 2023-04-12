use std::collections::VecDeque;

#[derive(Debug)]
pub enum Lexeme {
    Word(String),
    Number(i64),
    Comment(String),
    CompileBegin(u64),
    CompileEnd(u64),
}

#[derive(Debug)]
pub enum LexError {
    UnmatchedComment(String),
}

#[derive(Debug)]
pub struct Lexer {
    comment_depth: u64,
    compile_depth: u64,
    comment: String,
    words: VecDeque<Lexeme>,
}

impl Lexer {
    pub fn lex(&mut self, input: String) -> Result<(), LexError> {
        for word in input.split_whitespace() {
            match word {
                // Handle comments
                "(" => {
                    self.comment_depth += 1;
                }
                ")" => {
                    if self.comment_depth == 0 {
                        return Err(LexError::UnmatchedComment(String::from(word)));
                    }
                    self.comment_depth -= 1;
                    self.words
                        .push_back(Lexeme::Comment(self.comment.to_owned()));
                }
                word if self.comment_depth > 0 => {
                    if !self.comment.is_empty() {
                        self.comment.push(' ');
                    }
                    self.comment.push_str(word)
                }

                // Handle numbers
                word if word.parse::<i64>().is_ok() => {
                    self.words.push_back(Lexeme::Number(
                        word.parse::<i64>()
                            .expect("Failed parsing number, this should have been impossible"),
                    ));
                }

                // Handle compile mode
                ":" => {
                    self.words
                        .push_back(Lexeme::CompileBegin(self.compile_depth));
                    self.compile_depth += 1;
                }
                ";" => {
                    self.compile_depth -= 1;
                    self.words.push_back(Lexeme::CompileEnd(self.compile_depth));
                }

                // Handle words
                word => {
                    self.words.push_back(Lexeme::Word(String::from(word)));
                }
            };
        }

        println!("{:?}", self.words);

        Ok(())
    }

    pub fn new() -> Lexer {
        Lexer {
            comment_depth: 0,
            compile_depth: 0,
            comment: String::new(),
            words: VecDeque::new(),
        }
    }
}
