use crate::{function::Function, variable::Variable};
use runtime::Symbol;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Block<'a> {
    function: &'a Function,
    parent: Option<&'a Block<'a>>,
    variables: HashMap<Symbol, usize>,
    temporary_count: usize,
}

impl<'a> Block<'a> {
    pub fn new(function: &'a Function) -> Self {
        Self::with_capacity(function, 0)
    }

    pub fn with_capacity(function: &'a Function, capacity: usize) -> Self {
        Self {
            function,
            parent: None,
            variables: HashMap::with_capacity(capacity),
            temporary_count: 0,
        }
    }

    pub fn fork(&'a self) -> Self {
        Self {
            function: self.function,
            parent: Some(self),
            variables: Default::default(),
            temporary_count: 0,
        }
    }

    pub fn get_variable(&self, name: Symbol) -> Variable {
        let offset = self.variables.len() + self.temporary_count;

        if let Some(index) = self.variables.get(&name) {
            Variable::Bound(offset - index - 1)
        } else if let Some(parent) = &self.parent {
            match parent.get_variable(name) {
                Variable::Bound(index) => Variable::Bound(index + offset),
                variable @ Variable::Free(_) => variable,
            }
        } else if let Some(index) = self
            .function
            .free_variables()
            .iter()
            .position(|other| other == &name)
        {
            Variable::Free(index)
        } else {
            let index = self.function.free_variables().len();

            self.function.free_variables_mut().push(name);

            Variable::Free(index)
        }
    }

    pub fn insert_variable(&mut self, name: Symbol) {
        if self.variables.contains_key(&name) {
            self.temporary_count += 1;
        }

        self.variables.insert(name, self.variables.len());
    }

    pub fn temporary_count_mut(&mut self) -> &mut usize {
        &mut self.temporary_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_bound_variable() {
        let function = Function::new();
        let mut block = Block::new(&function);

        block.insert_variable("x".into());

        assert_eq!(block.get_variable("x".into()), Variable::Bound(0));
    }

    #[test]
    fn get_free_variable() {
        let function = Function::new();
        let block = Block::new(&function);

        assert_eq!(block.get_variable("x".into()), Variable::Free(0));
        assert_eq!(block.get_variable("y".into()), Variable::Free(1));
        assert_eq!(block.get_variable("z".into()), Variable::Free(2));
    }

    #[test]
    fn get_two_variables() {
        let function = Function::new();
        let mut block = Block::new(&function);

        block.insert_variable("x".into());
        block.insert_variable("y".into());

        assert_eq!(block.get_variable("x".into()), Variable::Bound(1));
        assert_eq!(block.get_variable("y".into()), Variable::Bound(0));
    }

    #[test]
    fn get_three_variables() {
        let function = Function::new();
        let mut block = Block::new(&function);

        block.insert_variable("x".into());
        block.insert_variable("y".into());
        block.insert_variable("z".into());

        assert_eq!(block.get_variable("x".into()), Variable::Bound(2));
        assert_eq!(block.get_variable("y".into()), Variable::Bound(1));
        assert_eq!(block.get_variable("z".into()), Variable::Bound(0));
    }

    #[test]
    fn get_variable_in_parent() {
        let function = Function::new();
        let mut block = Block::new(&function);

        block.insert_variable("x".into());

        let block = block.fork();

        assert_eq!(block.get_variable("x".into()), Variable::Bound(0));
    }

    #[test]
    fn get_two_variables_with_parent() {
        let function = Function::new();
        let mut block = Block::new(&function);

        block.insert_variable("x".into());

        let mut block = block.fork();

        block.insert_variable("y".into());

        assert_eq!(block.get_variable("x".into()), Variable::Bound(1));
        assert_eq!(block.get_variable("y".into()), Variable::Bound(0));
    }

    #[test]
    fn get_three_variables_with_parent() {
        let function = Function::new();
        let mut block = Block::new(&function);

        block.insert_variable("x".into());
        block.insert_variable("y".into());

        let mut block = block.fork();

        block.insert_variable("z".into());

        assert_eq!(block.get_variable("x".into()), Variable::Bound(2));
        assert_eq!(block.get_variable("y".into()), Variable::Bound(1));
        assert_eq!(block.get_variable("z".into()), Variable::Bound(0));
    }

    #[test]
    fn get_four_variables_with_parent() {
        let function = Function::new();
        let mut block = Block::new(&function);

        block.insert_variable("x".into());
        block.insert_variable("y".into());

        let mut block = block.fork();

        block.insert_variable("z".into());
        block.insert_variable("v".into());

        assert_eq!(block.get_variable("x".into()), Variable::Bound(3));
        assert_eq!(block.get_variable("y".into()), Variable::Bound(2));
        assert_eq!(block.get_variable("z".into()), Variable::Bound(1));
        assert_eq!(block.get_variable("v".into()), Variable::Bound(0));
    }
}
