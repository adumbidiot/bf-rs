use crate::parser::Expr;

pub trait Handler {
    fn read_char(&mut self) -> u8 {
        0
    }

    fn write_char(&mut self, _c: u8) {}

    fn mem_read(&mut self, _index: usize) {}
}

pub struct DefaultHandler;
impl Handler for DefaultHandler {}

#[derive(Debug)]
pub enum RuntimeError {
    GenericStr(&'static str),
}

pub struct Interpreter<T> {
    cells: Vec<u8>,
    current_cell_index: usize,

    pub handler: T,
}

impl<T: Handler> Interpreter<T> {
    pub fn new(handler: T) -> Self {
        Self {
            cells: Vec::new(),
            current_cell_index: 0,

            handler,
        }
    }

    pub fn cells(&self) -> &[u8] {
        &self.cells
    }

    pub fn current_cell_index(&self) -> usize {
        self.current_cell_index
    }

    fn cell(&mut self, index: usize) -> &mut u8 {
        if index >= self.cells.len() {
            self.cells.resize(index + 1, 0);
        }

        self.cells.get_mut(index).unwrap()
    }

    fn current_cell(&mut self) -> u8 {
        *self.cell(self.current_cell_index)
    }

    fn current_cell_mut(&mut self) -> &mut u8 {
        self.cell(self.current_cell_index)
    }

    pub fn run(&mut self, expr: &Expr) -> Result<(), RuntimeError> {
        match expr {
            Expr::Block { exprs } => {
                for expr in exprs {
                    self.run(expr)?;
                }
            }
            Expr::Increment { num } => {
                *self.current_cell_mut() = self.current_cell().overflowing_add(*num as u8).0;
            }
            Expr::Decrement { num } => {
                *self.current_cell_mut() = self.current_cell().overflowing_sub(*num as u8).0;
            }
            Expr::ShiftRight { num } => {
                self.current_cell_index += num;
            }
            Expr::ShiftLeft { num } => {
                self.current_cell_index -= num;
            }
            Expr::Loop { expr } => {
                self.handler.mem_read(self.current_cell_index);
                while self.current_cell() != 0 {
                    self.run(&expr)?;
                }
            }
            Expr::PrintChar => {
                self.handler.mem_read(self.current_cell_index);
                let cell = self.current_cell();
                self.handler.write_char(cell);
            }
            Expr::ReadChar => {
                *self.current_cell_mut() = self.handler.read_char();
            }
            Expr::Assign { index, value } => {
                *self.cell(*index) = *value;
            }
            Expr::AssignCurrent { value } => {
                *self.current_cell_mut() = *value;
            }
            Expr::PrintString { value } => {
                for b in value.bytes() {
                    self.handler.write_char(b);
                }
            }
            Expr::SetCellPointer { value } => {
                self.current_cell_index = *value;
            }
            Expr::ReadCharForget => {
                self.handler.read_char();
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::*;

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

    fn test_output(data: &str, expected: &str) {
        let mut l = Lexer::new(data);
        l.lex().unwrap();

        let mut p = Parser::new(l.tokens);
        let exprs = p.parse().unwrap();

        let mut vm = Interpreter::new(TestHandler::new());
        vm.run(&exprs).unwrap();

        assert_eq!(vm.handler.out.as_str(), expected);
    }

    #[test]
    fn factorial() {
        test_output(
            include_str!("../test_data/factorial.bf"),
            "0! = 1\n1! = 1\n2! = 2\n3! = 6\n4! = 24\n5! = 120\n6! = 28\n",
        ); // Requires 16 bit cells for proper answer
    }

    #[test]
    fn squares() {
        test_output(include_str!("../test_data/squares.bf"), "0\n1\n4\n9\n16\n25\n36\n49\n64\n81\n100\n121\n144\n169\n196\n225\n256\n289\n324\n361\n400\n441\n484\n529\n576\n625\n676\n729\n784\n841\n900\n961\n1024\n1089\n1156\n1225\n1296\n1369\n1444\n1521\n1600\n1681\n1764\n1849\n1936\n2025\n2116\n2209\n2304\n2401\n2500\n2601\n2704\n2809\n2916\n3025\n3136\n3249\n3364\n3481\n3600\n3721\n3844\n3969\n4096\n4225\n4356\n4489\n4624\n4761\n4900\n5041\n5184\n5329\n5476\n5625\n5776\n5929\n6084\n6241\n6400\n6561\n6724\n6889\n7056\n7225\n7396\n7569\n7744\n7921\n8100\n8281\n8464\n8649\n8836\n9025\n9216\n9409\n9604\n9801\n10000\n");
    }

    #[test]
    fn hello_world3() {
        test_output(
            include_str!("../test_data/hello_world3.bf"),
            "Hello, world!\n",
        );
    }

    #[test]
    fn hello_world2() {
        test_output(
            include_str!("../test_data/hello_world2.bf"),
            "Hello World!\n",
        );
    }

    #[test]
    fn hello_world1() {
        test_output(
            include_str!("../test_data/hello_world1.bf"),
            "Hello World!\n",
        );
    }

    #[test]
    fn count_down() {
        test_output(
            include_str!("../test_data/count_down.bf"),
            "9 8 7 6 5 4 3 2 1 0 ",
        );
    }

    #[test]
    fn aids() {
        test_output(
            include_str!("../test_data/aids.bf"),
            "How are you?I fucked a cheese burger",
        );
    }
}
