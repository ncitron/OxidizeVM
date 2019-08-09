mod machine;
mod opcodes;
mod frame;
use opcodes::Opcode;

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

    //used to select which program to run
    let program= max_program;

    let mut m = machine::build_machine(&program);
    m.run();
    m.print_stack();
}
