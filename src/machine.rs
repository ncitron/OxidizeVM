use std::collections::{VecDeque, HashMap};
use super::opcodes::Opcode;
use super::frame::Frame;
use std::env::set_var;

//data for vm
pub struct Machine<'a> {
    program: &'a [Opcode],
    ip: i32,
    stack: VecDeque<f32>,
    halted: bool,
    frame: Frame<'a>,
}

impl<'a> Machine<'a> {
    //prints contents of stack
    pub fn print_stack(&self) {
        println!("{:?}", self.stack);
    }

    //runs program
    pub fn run(&mut self) {
        while !self.halted {
            self.step();
        }
    }

    fn step(&mut self) {
        if self.ip < self.program.len() as i32 {
            self.ip+=1;
            self.decode(&self.program[(self.ip-1) as usize])

        }
    }

    //checks that stack has at least "size" number of elements on it
    fn check_stack(&self, size: usize) {
        if self.stack.len() < size {
            panic!("Must have at least {} elements on the stack for this operation.", size);
        }
    }

    //helpers to convert between floats (from the stack) and booleans
    fn to_bool(&self, n: f32) -> bool {
        if n  < 0.000001 {
            false
        } else {
            true
        }
    }

    fn to_float(&self, n: bool) -> f32 {
        if n {
            1.0
        } else {
            0.0
        }
    }

    //decoding and executing all instructions
    fn decode(&mut self, instruction: &'a Opcode) {
        match instruction {
            //halt
            Opcode::Halt => self.halted = true,

            //stack management
            Opcode::Push(arg1) => {
                self.stack.push_back(*arg1);
            },
            Opcode::Pop => {
                self.check_stack(1);
                self.stack.pop_back();
            },
            Opcode::Dup => {
                self.stack.push_back(*self.stack.get(self.stack.len()-1).unwrap());
            }

            //arithmetic operators
            Opcode::Add => {
                self.check_stack(2);
                let res = self.stack.pop_back().unwrap() + self.stack.pop_back().unwrap();
                self.stack.push_back(res);
            },
            Opcode::Sub => {
                self.check_stack(2);
                let res = -self.stack.pop_back().unwrap() + self.stack.pop_back().unwrap();
                self.stack.push_back(res);
            },
            Opcode::Mul => {
                self.check_stack(2);
                let res = self.stack.pop_back().unwrap() * self.stack.pop_back().unwrap();
                self.stack.push_back(res);
            },
            Opcode::Div => {
                self.check_stack(2);
                let res = 1.0/self.stack.pop_back().unwrap() * self.stack.pop_back().unwrap();
                self.stack.push_back(res);
            },

            //logical operators
            Opcode::And => {
                self.check_stack(2);
                let s1 = self.stack.pop_back().unwrap();
                let s2 = self.stack.pop_back().unwrap();

                let res = self.to_bool(s1) && self.to_bool(s2);
                self.stack.push_back(self.to_float(res));
            },
            Opcode::Or => {
                self.check_stack(2);
                let s1 = self.stack.pop_back().unwrap();
                let s2 = self.stack.pop_back().unwrap();

                let res = self.to_bool(s1) || self.to_bool(s2);
                self.stack.push_back(self.to_float(res));
            },
            Opcode::Not => {
                self.check_stack(1);
                let s1 = self.stack.pop_back().unwrap();
                let res = self.to_float(!self.to_bool(s1));
                self.stack.push_back(res);
            },

            //comparisons
            Opcode::Iseq => {
                self.check_stack(2);
                let s2 = self.stack.pop_back().unwrap();
                let s1 = self.stack.pop_back().unwrap();
                self.stack.push_back(self.to_float(s1 == s2))
            },
            Opcode::Isgt => {
                self.check_stack(2);
                let s2 = self.stack.pop_back().unwrap();
                let s1 = self.stack.pop_back().unwrap();
                self.stack.push_back(self.to_float(s1 > s2))
            },
            Opcode::Isge => {
                self.check_stack(2);
                let s2 = self.stack.pop_back().unwrap();
                let s1 = self.stack.pop_back().unwrap();
                self.stack.push_back(self.to_float(s1 >= s2));
            },

            //branching
            Opcode::Jmp(new_ip) => {
                self.ip = *new_ip;
            },
            Opcode::Jif(new_ip) => {
                let s1 = self.stack.pop_back().unwrap();
                if self.to_bool(s1) {
                    self.ip = *new_ip;
                }
            }

            //heap allocation
            //note: store will pop from stack and add to heap, load will copy heap and push onto stack
            Opcode::Store(key) => {
                self.check_stack(1);
                let s1 = self.stack.pop_back().unwrap();
                self.frame.set_var(key, s1);
            },
            Opcode::Load(key) => {
                self.stack.push_back(self.frame.get_var(key));
            },

            //io
            Opcode::Print => {
                println!("> {}", self.stack.pop_back().unwrap());
            }

            _ => panic!("Error: unknown opcode.")
        }
    }
}

//helper function to instantiate a new vm with a program loaded onto it
pub fn build_machine(program: &[Opcode]) -> Machine {
    let top_frame = Frame {
        vars: HashMap::new()
    };

    Machine {
        program,
        ip: 0,
        stack: VecDeque::new(),
        halted: false,
        frame: top_frame,
    }
}