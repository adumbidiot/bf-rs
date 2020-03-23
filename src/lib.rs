pub mod interpreter;
pub mod lexer;
pub mod optimize;
pub mod parser;
pub mod v1;

pub use crate::{
    interpreter::{
        Handler,
        Interpreter,
    },
    lexer::{
        Lexer,
        Token,
        TokenData,
    },
    optimize::{
        OptimizePass,
        Optimizer,
        SpecExecOptimizer,
        ZeroLoopOptimizer,
    },
    parser::{
        Expr,
        Parser,
    },
};

#[derive(Default)]
pub struct PythonCodeGen {
    pub output: String,
    tab_index: usize,
    newline: bool,
}

impl PythonCodeGen {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            tab_index: 0,
            newline: true,
        }
    }

    pub fn write(&mut self, s: &str) {
        for c in s.chars() {
            if self.newline {
                for _ in 0..self.tab_index {
                    self.output.push('\t');
                }
                self.newline = false;
            }

            match c {
                '\n' => {
                    self.newline = true;
                    self.output.push(c);
                }
                _ => {
                    self.output.push(c);
                }
            }
        }
    }

    pub fn write_preamble(&mut self) {
        self.write("cells = []\n");
        self.write("for i in range(0, 10000):\n");
        self.tab_index += 1;
        self.write("cells.append(0)\n");
        self.tab_index -= 1;

        self.write("cell_index = 0\n");
    }

    pub fn gen(&mut self, expr: &Expr) {
        if expr.uses_memory() {
            self.write_preamble();
        }

        self.gen_expr(expr);
    }

    fn gen_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Block { exprs } => {
                for expr in exprs {
                    self.gen_expr(expr);
                }
            }
            Expr::Increment { num } => {
                self.write(&format!("cells[cell_index] += {}\n", num));
            }
            Expr::Decrement { num } => {
                self.write(&format!("cells[cell_index] -= {}\n", num));
            }
            Expr::ShiftRight { num } => {
                self.write(&format!("cell_index += {}\n", num));
            }
            Expr::ShiftLeft { num } => {
                self.write(&format!("cell_index -= {}\n", num));
            }
            Expr::Loop { expr } => {
                self.write("while cells[cell_index] != 0:\n");
                self.tab_index += 1;
                self.gen_expr(expr);
                self.tab_index -= 1;
            }
            Expr::ReadChar => {
                self.write("cells[cell_index] = ord((input() + ' ')[0])\n");
            }
            Expr::PrintChar => {
                self.write("print(chr(cells[cell_index]), end='')\n");
            }
            Expr::Assign { index, value } => {
                self.write(&format!("cells[{}] = {}\n", index, value));
            }
            Expr::AssignCurrent { value } => {
                self.write(&format!("cells[cell_index] = {}\n", value));
            }
            Expr::SetCellPointer { value } => {
                self.write(&format!("cell_index = {}\n", value));
            }
            Expr::PrintString { value } => {
                self.write(&format!("print('{}', end='')\n", value));
            }
            Expr::ReadCharForget => {
                self.write("input()\n");
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestHandler {
        out: String,
    }

    impl TestHandler {
        fn new() -> Self {
            Self { out: String::new() }
        }
    }

    impl Handler for TestHandler {
        fn write_char(&mut self, c: u8) {
            self.out.push(char::from(c));
        }
    }

    fn test_output_optimized_py(data: &str, expected: &str) {
        let mut l = Lexer::new(data);
        l.lex().unwrap();

        let mut p = Parser::new(l.tokens);
        let exprs = p.parse().unwrap();

        let mut o = Optimizer::new(exprs);
        o.add_pass(ZeroLoopOptimizer);
        o.add_pass(SpecExecOptimizer);
        o.optimize();

        let exprs = o.expr;

        let mut codegen = PythonCodeGen::new();
        codegen.gen(&exprs);
        // std::fs::write("test.py", &codegen.output).unwrap();

        let mut vm = Interpreter::new(TestHandler::new());
        vm.run(&exprs).unwrap();

        dbg!(&exprs);

        assert_eq!(vm.handler.out.as_str(), expected);
    }

    #[test]
    fn aids_optimized_py() {
        test_output_optimized_py(
            include_str!("../test_data/aids.bf"),
            "How are you?I fucked a cheese burger",
        );
    }

    fn test_output_optimized(data: &str, expected: &str) {
        let mut l = Lexer::new(data);
        l.lex().unwrap();

        let mut p = Parser::new(l.tokens);
        let exprs = p.parse().unwrap();

        let mut o = Optimizer::new(exprs);
        o.add_pass(ZeroLoopOptimizer);
        o.add_pass(SpecExecOptimizer);
        o.optimize();

        let exprs = o.expr;

        let mut vm = Interpreter::new(TestHandler::new());
        vm.run(&exprs).unwrap();

        dbg!(&exprs);

        assert_eq!(vm.handler.out.as_str(), expected);
    }

    #[test]
    fn aids_optimized() {
        test_output_optimized(
            include_str!("../test_data/aids.bf"),
            "How are you?I fucked a cheese burger",
        );
    }
}
