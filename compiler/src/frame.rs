use runtime::Symbol;
use std::collections::HashMap;

pub struct Frame<'a> {
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

    pub fn insert_variable(&mut self, name: Symbol, index: usize) {
        self.variables.insert(name, index);
    }

    pub fn temporary_count_mut(&mut self) -> &mut usize {
        &mut self.temporary_count
    }
}
