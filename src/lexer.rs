use std::collections::VecDeque;

#[derive(Debug)]
pub enum Lexeme {
    Word(String),
    Number(i64),
    Comment(String),
    CompileBegin(u64),
    CompileEnd(u64),
}

// TODO: Remove this entirely
#[derive(Debug)]
enum Word {
    CompileBegin(u64),
    CompileEnd(u64),
    CommentBegin,
    CommentEnd,
    Comment(String),
    Emit,
    Execute,
    Interpret,
    Number(i64),
}

#[derive(Debug)]
pub enum ParseError {
    UnknownWord(String),
    UnmatchedComment(String),
}

#[derive(Debug)]
pub struct Lexer {
    comment_depth: u64,
    compile_depth: u64,
    comment: String,
}

impl Lexer {
    pub fn lex(&self, input: String) -> Result<VecDeque<Word>, ParseError> {
        let mut words: VecDeque<Word> = VecDeque::new();

        let mut comment_depth = 0;
        let mut compile_depth = 0;
        for word in input.split_whitespace() {
            match word {
                // Handle comments
                "(" => {
                    comment_depth += 1;
                    words.push_back(Word::CommentBegin);
                }
                ")" => {
                    if comment_depth == 0 {
                        return Err(ParseError::UnmatchedComment(String::from(word)));
                    }
                    comment_depth -= 1;
                    words.push_back(Word::CommentEnd);
                }
                _word if comment_depth > 0 => {}

                // Handle compile mode
                ":" => {
                    words.push_back(Word::CompileBegin(compile_depth));
                    compile_depth += 1;
                }
                ";" => {
                    compile_depth -= 1;
                    words.push_back(Word::CompileEnd(compile_depth));
                }
                word if word.eq_ignore_ascii_case("emit") => {
                    words.push_back(Word::Emit);
                }
                _ => {
                    return Err(ParseError::UnknownWord(String::from(word)));
                }
            };
        }

        return Ok(words);
    }
}
