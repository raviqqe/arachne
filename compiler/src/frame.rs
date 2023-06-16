use runtime::Symbol;
use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
};

#[derive(Clone, Copy, Debug)]
pub enum Variable {
    Bound(usize),
    Free(usize),
}

#[derive(Debug)]
pub struct Frame<'a> {
    parent: Option<&'a Frame<'a>>,
    variables: HashMap<Symbol, usize>,
    temporary_count: usize,
    // Only for function
    free_variables: Option<RefCell<Vec<Symbol>>>,
}

impl<'a> Frame<'a> {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            parent: None,
            variables: HashMap::with_capacity(capacity),
            temporary_count: 0,
            free_variables: Some(Default::default()),
        }
    }

    pub fn block(&'a self) -> Self {
        Self {
            parent: Some(self),
            variables: Default::default(),
            temporary_count: 0,
            free_variables: None,
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
        } else {
            let index = self.free_variables().len();

            self.free_variables_mut().push(name);

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

    pub fn free_variables(&self) -> Ref<Vec<Symbol>> {
        self.free_variables.as_ref().unwrap().borrow()
    }

    fn free_variables_mut(&self) -> RefMut<Vec<Symbol>> {
        if let Some(variables) = self.free_variables.as_ref() {
            variables.borrow_mut()
        } else {
            self.parent.unwrap().free_variables_mut()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_variable() {
        let mut frame = Frame::new();

        frame.insert_variable("x".into());

        assert_eq!(frame.get_variable("x".into()), Some(0));
    }

    #[test]
    fn get_two_variables() {
        let mut frame = Frame::new();

        frame.insert_variable("x".into());
        frame.insert_variable("y".into());

        assert_eq!(frame.get_variable("x".into()), Some(1));
        assert_eq!(frame.get_variable("y".into()), Some(0));
    }

    #[test]
    fn get_three_variables() {
        let mut frame = Frame::new();

        frame.insert_variable("x".into());
        frame.insert_variable("y".into());
        frame.insert_variable("z".into());

        assert_eq!(frame.get_variable("x".into()), Some(2));
        assert_eq!(frame.get_variable("y".into()), Some(1));
        assert_eq!(frame.get_variable("z".into()), Some(0));
    }

    #[test]
    fn get_variable_in_parent() {
        let mut frame = Frame::new();

        frame.insert_variable("x".into());

        let frame = frame.block();

        assert_eq!(frame.get_variable("x".into()), Some(0));
    }

    #[test]
    fn get_two_variables_with_parent() {
        let mut frame = Frame::new();

        frame.insert_variable("x".into());

        let mut frame = frame.block();

        frame.insert_variable("y".into());

        assert_eq!(frame.get_variable("x".into()), Some(1));
        assert_eq!(frame.get_variable("y".into()), Some(0));
    }

    #[test]
    fn get_three_variables_with_parent() {
        let mut frame = Frame::new();

        frame.insert_variable("x".into());
        frame.insert_variable("y".into());

        let mut frame = frame.block();

        frame.insert_variable("z".into());

        assert_eq!(frame.get_variable("x".into()), Some(2));
        assert_eq!(frame.get_variable("y".into()), Some(1));
        assert_eq!(frame.get_variable("z".into()), Some(0));
    }

    #[test]
    fn get_four_variables_with_parent() {
        let mut frame = Frame::new();

        frame.insert_variable("x".into());
        frame.insert_variable("y".into());

        let mut frame = frame.block();

        frame.insert_variable("z".into());
        frame.insert_variable("v".into());

        assert_eq!(frame.get_variable("x".into()), Some(3));
        assert_eq!(frame.get_variable("y".into()), Some(2));
        assert_eq!(frame.get_variable("z".into()), Some(1));
        assert_eq!(frame.get_variable("v".into()), Some(0));
    }
}
