struct Statement<T> {
    range: Range,
    value: T,
}

type Byte = u8;
type Word = u16;
type LongWord = u32;

type RegisterIndex = Byte;

struct An {
    index: RegisterIndex,
}

struct Dn {
    index: RegisterIndex,
}

enum Xn {
    An(Statement<An>),
    Dn(Statement<Dn>),
}

enum EffectiveAddress {
    Dn(Statement<Dn>),
    An(Statement<An>),
    AnInd(Statement<An>),
    AnIndWithPostInc(Statement<An>),
    AnIndWithPreDec(Statement<An>),
    AnIndWithDisplacement(Statement<Word>, Statement<An>),
    AnIndWithIndex(Statement<Byte>, Statement<An>, Statement<Xn>),
    AbsoluteWord(Statement<Word>),
    AbsoluteLongWord(Statement<LongWord>),
    PcIndWithDisplacement(Statement<Word>),
    PcIndWithIndex(Statement<Byte>, Statement<Xn>),
}

impl std::string::ToString for Register {
    fn to_string(&self) -> String {
        match self {
            Register::PC => String::from("PC"),
            Register::SP => String::from("SP"),
            Register::An(n) => format!("A{}", n),
            Register::Dn(n) => format!("D{}", n),
        }
    }
}

type Operand = EffectiveAddress;

enum OperationType {
    Add,
    Adda,
    Addi,
    Addq,
    Addx, // ...
}

enum Size {
    Byte,
    Word,
    LongWord,
}

struct Operation {
    type: Statement<OperationType>,
    size: Statement<Size>,
    operands: Vec<Statement<Operand>>,
}

type Comment = String;

type Label = String;

type Program = Vec<Statement>;
