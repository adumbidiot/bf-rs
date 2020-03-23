use crate::{
    Token,
    TokenData,
};

#[derive(Debug)]
pub enum ParseError {}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Block { exprs: Vec<Expr> },
    Increment { num: usize },
    Decrement { num: usize },
    PrintChar,
    ReadChar,
    ShiftLeft { num: usize },
    ShiftRight { num: usize },
    Loop { expr: Box<Expr> },

    Assign { index: usize, value: u8 },
    AssignCurrent { value: u8 },
    PrintString { value: String },
    SetCellPointer { value: usize },
    ReadCharForget,
}

impl Expr {
    pub fn is_read(&self) -> bool {
        match self {
            Self::ReadChar { .. } => true,
            _ => false,
        }
    }

    pub fn contains_read(&self) -> bool {
        match self {
            Self::ReadChar { .. } => true,
            Self::ReadCharForget { .. } => true,
            Self::Block { exprs } => exprs.iter().any(|expr| expr.contains_read()),
            Self::Loop { expr } => expr.contains_read(),
            _ => false,
        }
    }

    pub fn is_block(&self) -> bool {
        match self {
            Self::Block { .. } => true,
            _ => false,
        }
    }

    pub fn is_loop(&self) -> bool {
        match self {
            Self::Loop { .. } => true,
            _ => false,
        }
    }

    pub fn uses_memory(&self) -> bool {
        match self {
            Self::Block { exprs } => exprs.iter().any(|expr| expr.uses_memory()),
            Self::Loop { expr } => expr.uses_memory(),
            Self::PrintString { .. } => false,
            Self::ReadCharForget => false,
            _ => true,
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,

    loop_count: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            index: 0,
            loop_count: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        let mut exprs = Vec::new();

        while self.index < self.tokens.len() {
            match self.tokens[self.index].data {
                TokenData::Increment(num) => {
                    exprs.push(Expr::Increment { num });
                    self.index += 1;
                }
                TokenData::Decrement(num) => {
                    exprs.push(Expr::Decrement { num });
                    self.index += 1;
                }
                TokenData::ShiftLeft(num) => {
                    exprs.push(Expr::ShiftLeft { num });
                    self.index += 1;
                }
                TokenData::ShiftRight(num) => {
                    exprs.push(Expr::ShiftRight { num });
                    self.index += 1;
                }
                TokenData::Print => {
                    exprs.push(Expr::PrintChar);
                    self.index += 1;
                }
                TokenData::Read => {
                    exprs.push(Expr::ReadChar);
                    self.index += 1;
                }
                TokenData::StartLoop => {
                    self.loop_count += 1;
                    self.index += 1;

                    let expr = self.parse()?;
                    exprs.push(Expr::Loop { expr: expr.into() });
                }
                TokenData::EndLoop => {
                    self.index += 1;
                    if self.loop_count > 0 {
                        self.loop_count -= 1;
                        break;
                    }
                }
                TokenData::Other(_) => {
                    self.index += 1;
                }
            }
        }

        Ok(Expr::Block { exprs })
    }
}
