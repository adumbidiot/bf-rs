#[derive(Debug)]
pub enum TokenData {
    ShiftLeft(usize),
    ShiftRight(usize),
    Increment(usize),
    Decrement(usize),
    StartLoop,
    EndLoop,
    Read,
    Print,

    Other(String),
}

#[derive(Debug)]
pub struct Token {
    pub data: TokenData,
}

fn is_bf_char(c: char) -> bool {
    c == '+' || c == '-' || c == '<' || c == '>' || c == '.' || c == ',' || c == '[' || c == ']'
}

#[derive(Debug)]
pub struct LexerError;

pub struct Lexer<'a> {
    pub tokens: Vec<Token>,

    iter: std::iter::Peekable<std::str::CharIndices<'a>>,
    data: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(data: &'a str) -> Self {
        Self {
            tokens: Vec::new(),
            iter: data.char_indices().peekable(),
            data,
        }
    }

    fn push_token(&mut self, data: TokenData) {
        self.tokens.push(Token { data });
    }

    fn count_char(&mut self, c: char) -> usize {
        let mut n = 0;
        while Some(c) == self.iter.peek().map(|(_, c)| *c) {
            n += 1;
            self.iter.next();
        }

        n
    }

    pub fn lex(&mut self) -> Result<(), LexerError> {
        loop {
            let next_char = self.iter.peek().copied();
            match next_char {
                Some((_, '+')) => {
                    let n = self.count_char('+');
                    self.push_token(TokenData::Increment(n));
                }
                Some((_, '-')) => {
                    let n = self.count_char('-');
                    self.push_token(TokenData::Decrement(n));
                }
                Some((_, '>')) => {
                    let n = self.count_char('>');
                    self.push_token(TokenData::ShiftRight(n));
                }
                Some((_, '<')) => {
                    let n = self.count_char('<');
                    self.push_token(TokenData::ShiftLeft(n));
                }
                Some((_, '.')) => {
                    self.iter.next();
                    self.push_token(TokenData::Print);
                }
                Some((_, ',')) => {
                    self.iter.next();
                    self.push_token(TokenData::Read);
                }
                Some((_, ']')) => {
                    self.iter.next();
                    self.push_token(TokenData::EndLoop);
                }
                Some((_, '[')) => {
                    self.iter.next();
                    self.push_token(TokenData::StartLoop);
                }
                Some((start, _)) => {
                    let mut end = 0;
                    while let Some((i, c)) = self.iter.peek() {
                        end = *i;
                        if is_bf_char(*c) {
                            break;
                        }
                        self.iter.next();
                    }

                    let s = self.data[start..end].to_string();
                    self.push_token(TokenData::Other(s));
                }
                None => {
                    break;
                }
            }
        }

        Ok(())
    }
}
