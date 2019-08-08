mod machine;
mod opcodes;
mod frame;
use opcodes::Opcode;

fn main() {

    let program = [
        Opcode::Push(1.0),
        Opcode::Jif(3),
        Opcode::Push(2.0),
        Opcode::Push(5.0),
        Opcode::Halt,
    ];

    let mut m = machine::build_machine(&program);
    m.run();
    m.print_stack();
}
