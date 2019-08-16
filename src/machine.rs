use std::collections::{VecDeque, HashMap};
use super::opcodes::Opcode;
use super::frame::Frame;
use super::stackobj::StackObj;

//data for vm
pub struct Machine<'a> {
    program: &'a [Opcode],
    ip: i32,
    stack: VecDeque<StackObj>,
    halted: bool,
    frames: VecDeque<Frame<'a>>,
}

impl<'a> Machine<'a> {
    //prints contents of stack
    pub fn print_stack(&self) {
        println!("Stack contents: ");
        for stack in self.stack.iter() {
            println!("{}", stack);
        }
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
    pub fn to_bool(s: StackObj) -> bool {
        let n = match s {
            StackObj::Num(arg) => arg,
            _ => panic!("Cannot cast non number to boolean.")
        };
        if n  < 0.000001 {
            false
        } else {
            true
        }
    }

    pub fn to_float(n: bool) -> StackObj {
        if n {
            StackObj::Num(1.0)
        } else {
            StackObj::Num(0.0)
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
                let s2 = self.stack.pop_back().unwrap();
                let s1 = self.stack.pop_back().unwrap();
                self.stack.push_back(s1-s2);
            },
            Opcode::Mul => {
                self.check_stack(2);
                let res = self.stack.pop_back().unwrap() * self.stack.pop_back().unwrap();
                self.stack.push_back(res);
            },
            Opcode::Div => {
                self.check_stack(2);
                let s2 = self.stack.pop_back().unwrap();
                let s1 = self.stack.pop_back().unwrap();
                let res = s1/s2;
                self.stack.push_back(res);
            },

            //logical operators
            Opcode::And => {
                self.check_stack(2);
                let s1 = self.stack.pop_back().unwrap();
                let s2 = self.stack.pop_back().unwrap();

                let res = Machine::to_bool(s1) && Machine::to_bool(s2);
                self.stack.push_back(Machine::to_float(res));
            },
            Opcode::Or => {
                self.check_stack(2);
                let s1 = self.stack.pop_back().unwrap();
                let s2 = self.stack.pop_back().unwrap();

                let res = Machine::to_bool(s1) || Machine::to_bool(s2);
                self.stack.push_back(Machine::to_float(res));
            },
            Opcode::Not => {
                self.check_stack(1);
                let s1 = self.stack.pop_back().unwrap();
                let res = Machine::to_float(!Machine::to_bool(s1));
                self.stack.push_back(res);
            },

            //comparisons
            Opcode::Iseq => {
                self.check_stack(2);
                let s2 = self.stack.pop_back().unwrap();
                let s1 = self.stack.pop_back().unwrap();
                self.stack.push_back(Machine::to_float(s1 == s2))
            },
            Opcode::Isgt => {
                self.check_stack(2);
                let s2 = self.stack.pop_back().unwrap();
                let s1 = self.stack.pop_back().unwrap();
                self.stack.push_back(Machine::to_float(s1 > s2))
            },
            Opcode::Isge => {
                self.check_stack(2);
                let s2 = self.stack.pop_back().unwrap();
                let s1 = self.stack.pop_back().unwrap();
                self.stack.push_back(Machine::to_float(s1 >= s2));
            },

            //branching
            Opcode::Jmp(new_ip) => {
                self.ip = *new_ip;
            },
            Opcode::Jif(new_ip) => {
                let s1 = self.stack.pop_back().unwrap();
                if Machine::to_bool(s1) {
                    self.ip = *new_ip;
                }
            }

            //heap allocation
            //note: store will pop from stack and add to heap, load will copy heap and push onto stack
            Opcode::Store(key) => {
                self.check_stack(1);
                let s1 = self.stack.pop_back().unwrap();
                self.frames.back_mut().unwrap().set_var(key, s1);
            },
            Opcode::Load(key) => {
                self.stack.push_back(self.frames.back_mut().unwrap().get_var(key));
            },

            //io
            Opcode::Print => {
                println!("> {}", self.stack.pop_back().unwrap());
            },

            //call and ret
            Opcode::Call(address) => {
                self.frames.push_back(Frame {
                    vars: HashMap::new(),
                    return_address: self.ip,
                });
                self.ip = *address;
            },
            Opcode::Ret => {
                self.ip = self.frames.pop_back().unwrap().return_address;
            },

            _ => panic!("Error: unknown opcode.")
        }
    }

    //create a new machine and load "program" in it
    pub fn new(program: &[Opcode]) -> Machine {
        let top_frame = Frame {
            vars: HashMap::new(),
            return_address: 0,
        };

        let mut frame_stack = VecDeque::new();
        frame_stack.push_back(top_frame);

        Machine {
            program,
            ip: 0,
            stack: VecDeque::new(),
            halted: false,
            frames: frame_stack,
        }
    }
}
