pub mod compiler {

    #[derive(Debug)]
    pub enum Code {
        PUSH(usize),
        STORE(String),
    }
}
