use crate::{
    interpreter::{
        Handler,
        Interpreter,
    },
    parser::Expr,
};
// use std::collections::HashSet;

pub trait OptimizePass {
    fn optimize(&mut self, expr: &mut Expr);
}

pub struct ZeroLoopOptimizer;

impl OptimizePass for ZeroLoopOptimizer {
    fn optimize(&mut self, top_expr: &mut Expr) {
        match top_expr {
            Expr::Block { exprs } => {
                for expr in exprs.iter_mut() {
                    self.optimize(expr);
                }
            }
            Expr::Loop { expr } => match &**expr {
                Expr::Block { exprs } if exprs.as_slice() == [Expr::Decrement { num: 1 }] => {
                    *top_expr = Expr::AssignCurrent { value: 0 };
                }
                _ => {}
            },
            _ => {}
        }
    }
}

pub struct SpecExecHandler {
    out: Vec<String>,
    // dirty_cells: HashSet<usize>,
}

impl SpecExecHandler {
    fn new() -> Self {
        Self {
            out: Vec::new(),
            // dirty_cells: HashSet::new(),
        }
    }

    fn print(&mut self, c: char) {
        if self.out.is_empty() {
            self.out.push(String::new());
        }

        self.out.last_mut().unwrap().push(c);
    }
}

impl Handler for SpecExecHandler {
    fn write_char(&mut self, c: u8) {
        self.print(char::from(c));
    }
}

pub struct SpecExecOptimizer;

impl OptimizePass for SpecExecOptimizer {
    fn optimize(&mut self, top_expr: &mut Expr) {
        if let Expr::Block { exprs } = top_expr {
            let mut vm = Interpreter::new(SpecExecHandler::new());
            let mut read_pos = None;

            for (i, expr) in exprs.iter().enumerate() {
                if expr.contains_read() {
                    match exprs.get(i + 1) {
                        Some(Expr::AssignCurrent { .. }) => {
                            vm.handler.out.push(String::new());
                        }
                        _ => {
                            if exprs.iter().skip(i + 1).all(|expr| !expr.uses_memory()) {
                                vm.handler.out.push(String::new());
                            } else {
                                read_pos = Some(i);
                                break;
                            }
                        }
                    }
                }

                match vm.run(&expr) {
                    Ok(_) => {}
                    Err(_e) => {
                        return;
                    }
                }
            }

            match read_pos {
                Some(pos) => {
                    let mut new_exprs = Vec::new();
                    for (i, cell) in vm.cells().iter().enumerate() {
                        new_exprs.push(Expr::Assign {
                            index: i,
                            value: *cell,
                        });
                    }

                    for value in vm.handler.out.iter().take(vm.handler.out.len() - 1) {
                        new_exprs.push(Expr::PrintString {
                            value: value.clone(),
                        });

                        new_exprs.push(Expr::ReadCharForget);
                    }

                    new_exprs.push(Expr::PrintString {
                        value: vm.handler.out.last().unwrap().clone(),
                    });

                    new_exprs.push(Expr::SetCellPointer {
                        value: vm.current_cell_index(),
                    });

                    for expr in exprs.iter().skip(pos).cloned() {
                        new_exprs.push(expr);
                    }

                    *top_expr = Expr::Block { exprs: new_exprs };
                }
                None => {
                    let mut new_exprs = Vec::new();
                    for value in vm.handler.out.iter().take(vm.handler.out.len() - 1) {
                        new_exprs.push(Expr::PrintString {
                            value: value.clone(),
                        });

                        new_exprs.push(Expr::ReadCharForget);
                    }

                    new_exprs.push(Expr::PrintString {
                        value: vm.handler.out.last().unwrap().clone(),
                    });

                    *top_expr = Expr::Block { exprs: new_exprs };
                }
            }
        }
    }
}

/*
fn overwrites_current_cell(expr: &Expr) -> bool {
    match expr {
        Expr::AssignCurrent { .. } => true,
        //  Expr::ReadChar => true,
        _ => false,
    }
}
*/
/*
pub struct WriteMergeOptimizer;

impl OptimizePass for WriteMergeOptimizer {
    fn optimize(&mut self, top_expr: &mut Expr) {
        match top_expr {
            Expr::Block { exprs } => {
                let mut new_exprs = Vec::with_capacity(exprs.len());

                for i in 0..exprs.len() {
                    let expr1 = exprs.get(i);
                    let expr2 = exprs.get(i + 1);

                    match (expr1, expr2) {
                        (Some(expr1), _) => {
                            new_exprs.push(expr1.clone());
                        }
                        _ => {}
                    }
                }

                if &mut new_exprs != exprs {
                    std::mem::swap(&mut new_exprs, exprs);
                }
            }
            Expr::Loop { expr } => self.optimize(expr),
            _ => {}
        }
    }
}
*/
pub struct Optimizer {
    pub expr: Expr,

    passes: Vec<Box<dyn OptimizePass>>,
}

impl Optimizer {
    pub fn new(mut expr: Expr) -> Self {
        if !expr.is_block() {
            expr = Expr::Block { exprs: vec![expr] };
        }

        Optimizer {
            expr,
            passes: Vec::new(),
        }
    }

    pub fn add_pass<P: OptimizePass + 'static>(&mut self, pass: P) {
        self.passes.push(Box::new(pass));
    }

    pub fn optimize(&mut self) {
        let limit = 3;
        let mut old_expr = self.expr.clone();

        for _ in 0..limit {
            for pass in self.passes.iter_mut() {
                pass.optimize(&mut self.expr);
            }

            if self.expr == old_expr {
                break;
            } else {
                old_expr = self.expr.clone();
            }
        }
    }
}
