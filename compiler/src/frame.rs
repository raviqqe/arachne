use runtime::Symbol;
use std::collections::HashMap;

pub struct Frame<'a> {
    // TODO Support blocks.
    #[allow(dead_code)]
    parent: Option<&'a Frame<'a>>,
    variables: HashMap<Symbol, usize>,
    temporary_count: usize,
}

impl<'a> Frame<'a> {
    pub fn new() -> Self {
        Self {
            parent: None,
            variables: Default::default(),
            temporary_count: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            parent: None,
            variables: HashMap::with_capacity(capacity),
            temporary_count: 0,
        }
    }

    // TODO Support blocks.
    #[allow(dead_code)]
    pub fn fork(&'a self) -> Self {
        Self {
            parent: Some(self),
            variables: Default::default(),
            temporary_count: 0,
        }
    }

    pub fn get_variable(&mut self, name: Symbol) -> Option<usize> {
        self.variables.get(&name).copied()
    }

    pub fn insert_variable(&mut self, name: Symbol) {
        self.variables.insert(name, self.variables.len());
    }

    pub fn temporary_count_mut(&mut self) -> &mut usize {
        &mut self.temporary_count
    }
}
