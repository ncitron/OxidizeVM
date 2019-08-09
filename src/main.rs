mod machine;
mod opcodes;
mod frame;
use opcodes::Opcode;
use std::borrow::Borrow;

fn main() {

    //finds the max of two values
    let max_program = [
        Opcode::Push(2.0),
        Opcode::Store("var1".to_string()),
        Opcode::Push(3.0),
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
    let  whileloop= [
        Opcode::Push(1.0),
        Opcode::Dup,
        Opcode::Store("a".to_string()),
        Opcode::Store("b".to_string()),
        Opcode::Push(0.0),
        Opcode::Store("i".to_string()),

        Opcode::Push(10.0),
        Opcode::Store("end".to_string()),

        Opcode::Load("i".to_string()),

        //loop contents
        Opcode::Load("i".to_string()),
        Opcode::Print,
        //end loop contents

        Opcode::Push(1.0),
        Opcode::Add,
        Opcode::Store("i".to_string()),
        Opcode::Load("end".to_string()),
        Opcode::Load("i".to_string()),
        Opcode::Isgt,
        Opcode::Jif(8),

        Opcode::Halt
    ];

    //used to select which program to run
    let program= whileloop;

    let mut m = machine::build_machine(&program);
    m.run();
    m.print_stack();
}
