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
                                } else if found_start == 1 && i.is_end_loop() {
                                    true
                                } else {
                                    false
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
