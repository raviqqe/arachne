use crate::block::Block;
use runtime::Symbol;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Frame<'a> {
    block: Block<'a>,
    free_variables: HashSet<usize>,
}

impl<'a> Frame<'a> {
    pub fn new() -> Self {
        Self {
            block: Block::new(),
            free_variables: Default::default(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            block: Block::with_capacity(capacity),
            free_variables: Default::default(),
        }
    }

    pub fn fork(&'a self) -> Block<'a> {
        self.block.fork()
    }

    pub fn get_variable(&self, name: Symbol) -> Option<usize> {
        self.block.get_variable(name)
    }

    pub fn insert_variable(&mut self, name: Symbol) {
        self.block.insert_variable(name)
    }

    pub fn temporary_count_mut(&mut self) -> &mut usize {
        self.block.temporary_count_mut()
    }

    pub fn free_variables(&self) -> &HashSet<usize> {
        &self.free_variables
    }
}
