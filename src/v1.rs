use std::fmt;

#[derive(Debug)]
pub enum Instruction {
    ShiftLeft,
    ShiftRight,

    Increment,
    Decrement,

    StartLoop,
    EndLoop,

    Read,
    Print,
}

impl Instruction {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '>' => Some(Instruction::ShiftRight),
            '<' => Some(Instruction::ShiftLeft),
            '+' => Some(Instruction::Increment),
            '-' => Some(Instruction::Decrement),
            '[' => Some(Instruction::StartLoop),
            ']' => Some(Instruction::EndLoop),
            ',' => Some(Instruction::Read),
            '.' => Some(Instruction::Print),
            _ => None,
        }
    }

    pub fn is_end_loop(&self) -> bool {
        match self {
            Instruction::EndLoop => true,
            _ => false,
        }
    }

    pub fn is_start_loop(&self) -> bool {
        match self {
            Instruction::StartLoop => true,
            _ => false,
        }
    }
}

fn default_output_func(_c: u8) {}
fn default_input_func() -> u8 {
    0
}

#[derive(Clone)]
pub struct Interpreter<'i, 'o> {
    mem: Vec<u8>,
    ptr: usize,
    output_func: &'o dyn Fn(u8),
    input_func: &'i dyn Fn() -> u8,
}

impl<'i, 'o> Interpreter<'i, 'o> {
    pub fn new() -> Self {
        Interpreter {
            mem: Vec::new(),
            ptr: 0,
            output_func: &default_output_func,
            input_func: &default_input_func,
        }
    }

    pub fn set_output_func(&mut self, output_func: &'o dyn Fn(u8)) {
        self.output_func = output_func;
    }

    pub fn set_input_func(&mut self, input_func: &'i dyn Fn() -> u8) {
        self.input_func = input_func;
    }

    fn get(&mut self, i: usize) -> u8 {
        if i >= self.mem.len() {
            self.mem.resize(i + 1, 0);
        }

        *self.mem.get(i).unwrap()
    }

    fn get_mut(&mut self, i: usize) -> &mut u8 {
        if i >= self.mem.len() {
            self.mem.resize(i + 1, 0);
        }

        self.mem.get_mut(i).unwrap()
    }

    pub fn exec(&mut self, instructions: &[Instruction]) {
        let mut loop_stack = Vec::new();

        let mut i = 0;
        while i < instructions.len() {
            let ins = instructions.get(i).expect("Instruction");
            match ins {
                Instruction::ShiftRight => {
                    self.ptr += 1;
                }
                Instruction::ShiftLeft => {
                    self.ptr -= 1;
                }
                Instruction::Increment => {
                    let (v, _) = self.get(self.ptr).overflowing_add(1);
                    *self.get_mut(self.ptr) = v;
                }
                Instruction::Decrement => {
                    *self.get_mut(self.ptr) -= 1;
                }
                Instruction::StartLoop => {
                    if self.get(self.ptr) == 0 {
                        let mut found_start = 0;
                        i += instructions[i..]
                            .iter()
                            .position(|i| {
                                if found_start > 1 && i.is_end_loop() {
                                    found_start -= 1;
                                    false
                                } else if i.is_start_loop() {
                                    found_start += 1;
                                    false
                                } else {
                                    found_start == 1 && i.is_end_loop()
                                }
                            })
                            .expect("Valid end loop");
                    } else {
                        loop_stack.push(i);
                    }
                }
                Instruction::EndLoop => {
                    if self.get(self.ptr) != 0 {
                        i = *loop_stack.last().expect("Valid startloop");
                    } else {
                        loop_stack.pop().expect("In Loop");
                    }
                }
                Instruction::Read => {
                    *self.get_mut(self.ptr) = (self.input_func)();
                }
                Instruction::Print => {
                    let out = self.get(self.ptr);
                    (self.output_func)(out);
                }
            }
            i += 1;
        }
    }
}

impl<'i, 'o> fmt::Debug for Interpreter<'i, 'o> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Interpreter")
            .field("mem", &self.mem)
            .field("ptr", &self.ptr)
            .finish()
    }
}

impl<'i, 'o> Default for Interpreter<'i, 'o> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::cell::RefCell;

    fn test_output(data: &str, expected: &str) {
        let data = data
            .chars()
            .filter_map(Instruction::from_char)
            .collect::<Vec<_>>();

        let s = RefCell::new(String::new());
        let mut vm = Interpreter::new();
        let output_func = |c| {
            //print!("{}", c as char);
            s.borrow_mut().push(char::from(c));
        };
        vm.set_output_func(&output_func);
        vm.exec(&data);

        assert_eq!(s.borrow().as_str(), expected);
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
