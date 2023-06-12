pub struct Stack {
    variables: HashMap<Symbol, usize>,
    temporary_count: usize,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            variables: HashMap::with_capacity(GLOBAL_VARIABLE_CAPACITY),
            temporary_count: 0,
        }
    }

    pub fn insert_variable(&self) -> HashMap<Foo> {}
}
