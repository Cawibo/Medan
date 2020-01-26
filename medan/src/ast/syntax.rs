pub mod compiler {

    #[derive(Debug)]
    pub enum Code {
        NOOP,

        // data
        PUSH(usize),
        STORE(String),
        FETCH(String),

        // boolean constants
        TRUE,
        FALSE,

        // operators
        ADD,
        MULT,
        SUB,
        EQ,
        LE,
        NEG,
        AND,

        // branching
        BRANCH(Vec<Code>, Vec<Code>),
        LOOP(Vec<Code>, Vec<Code>),
    }
}
