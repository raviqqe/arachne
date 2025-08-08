use runtime::Symbol;
use std::cell::{Ref, RefCell, RefMut};

#[derive(Debug, Default)]
pub struct Function {
    free_variables: RefCell<Vec<Symbol>>,
}

impl Function {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn free_variables(&self) -> Ref<'_, Vec<Symbol>> {
        self.free_variables.borrow()
    }

    pub fn free_variables_mut(&self) -> RefMut<'_, Vec<Symbol>> {
        self.free_variables.borrow_mut()
    }
}
