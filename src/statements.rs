use m68k_reloaded_common::Range;

pub struct Stmt<T> {
    pub range: Range,
    pub value: T,
}

pub type Byte = u8;
pub type Word = u16;
pub type LongWord = u32;

pub type RegisterIndex = Byte;

pub struct An {
    pub index: RegisterIndex,
}

pub struct Dn {
    pub index: RegisterIndex,
}

pub enum Xn {
    An(Stmt<An>),
    Dn(Stmt<Dn>),
}

pub enum EffectiveAddress {
    Dn(Stmt<Dn>),
    An(Stmt<An>),
    AnInd(Stmt<An>),
    AnIndWithPostInc(Stmt<An>),
    AnIndWithPreDec(Stmt<An>),
    AnIndWithDisplacement(Stmt<Word>, Stmt<An>),
    AnIndWithIndex(Stmt<Byte>, Stmt<An>, Stmt<Xn>),
    AbsoluteWord(Stmt<Word>),
    AbsoluteLongWord(Stmt<LongWord>),
    PcIndWithDisplacement(Stmt<Word>),
    PcIndWithIndex(Stmt<Byte>, Stmt<Xn>),
}

// impl std::string::ToString for Register {
//     fn to_string(&self) -> String {
//         match self {
//             Register::PC => String::from("PC"),
//             Register::SP => String::from("SP"),
//             Register::An(n) => format!("A{}", n),
//             Register::Dn(n) => format!("D{}", n),
//         }
//     }
// }

pub type Operand = EffectiveAddress;

pub enum OperationType {
    Add,
    Adda,
    Addi,
    Addq,
    Addx, // ...
}

pub enum Size {
    Byte,
    Word,
    LongWord,
}

pub struct Operation {
    pub operation_type: Stmt<OperationType>,
    pub size: Stmt<Size>,
    pub operands: Vec<Stmt<Operand>>,
}

pub type Comment = String;

pub type Label = String;

pub enum Statement {
    Label(Label),
    Operation(Operation),
    Comment(Comment),
}

pub type Program = Vec<Stmt<Statement>>;
