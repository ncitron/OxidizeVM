//list of opcodes for the vm

pub enum Opcode {
    //ends program
    Halt,

    //stack manipulation
    Push(f32),
    Pop,
    Dup,

    //heap store/load
    Store(String),
    Load(String),

    //arithmatic operations
    Add,
    Sub,
    Mul,
    Div,

    //logical operations
    Not,
    And,
    Or,

    //function call/return
    Call,
    Ret,

    //unconditional and conditional jumps
    Jmp(i32),
    Jif(i32),

    //comparison operators
    Iseq,
    Isge,
    Isgt,

    //io
    Print,
}