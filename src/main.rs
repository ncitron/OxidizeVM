mod machine;
mod opcodes;
mod frame;
mod stackobj;
use opcodes::Opcode;
use crate::machine::Machine;
use crate::stackobj::StackObj;

fn main() {

    //finds the max of two values
    let max_program = [
        Opcode::Push(StackObj::Num(2.0)),
        Opcode::Store("var1".to_string()),
        Opcode::Push(StackObj::Num(3.0)),
        Opcode::Store("var2".to_string()),

        Opcode::Load("var1".to_string()),
        Opcode::Load("var2".to_string()),

        Opcode::Isgt,
        Opcode::Jif(10),

        Opcode::Load("var2".to_string()),
        Opcode::Jmp(11),

        Opcode::Load("var1".to_string()),

        Opcode::Halt
    ];

    //whileloop
    let  while_loop= [
        Opcode::Push(StackObj::Num(1.0)),
        Opcode::Dup,
        Opcode::Store("a".to_string()),
        Opcode::Store("b".to_string()),
        Opcode::Push(StackObj::Num(0.0)),
        Opcode::Store("i".to_string()),

        Opcode::Push(StackObj::Num(10.0)),
        Opcode::Store("end".to_string()),

        //loop contents
        Opcode::Load("i".to_string()),
        Opcode::Print,
        //end loop contents

        Opcode::Load("i".to_string()),
        Opcode::Push(StackObj::Num(1.0)),
        Opcode::Add,
        Opcode::Store("i".to_string()),
        Opcode::Load("end".to_string()),
        Opcode::Load("i".to_string()),
        Opcode::Isgt,
        Opcode::Jif(8),

        Opcode::Halt
    ];

    //test function
    let test_function = [
        Opcode::Push(StackObj::Num(0.0)),
        Opcode::Store("local".to_string()),
        Opcode::Call(6),
        Opcode::Load("local".to_string()),
        Opcode::Print,
        Opcode::Halt,

        Opcode::Load("local".to_string()),
        Opcode::Print,
        Opcode::Ret,

    ];

    let stack_pusher = [
        Opcode::Push(StackObj::Num(1.0)),
        Opcode::Halt,
    ];

    //selects which program to run
    let program= stack_pusher;

    let mut m = Machine::new(&program);
    m.run();
    m.print_stack();

}
