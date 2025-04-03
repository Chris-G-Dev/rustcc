pub struct Program {
    func_defs: Vec<Function>,
}

pub struct Function {
    name: String, //
    body: Statement,
}
