#[derive(Debug)]
pub struct Stack<T, const N: usize> {
    values: Vec<T>,
}

impl<T, const N: usize> Stack<T, N> {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            values: Default::default(),
        }
    }

    #[inline(always)]
    pub fn push(&mut self, value: T) {
        self.values.push(value)
    }

    #[inline(always)]
    pub fn pop(&mut self) -> T {
        self.values.pop().expect("stack value")
    }

    #[inline(always)]
    pub fn peek(&self, index: usize) -> &T {
        &self.values[self.values.len() - 1 - index]
    }

    #[inline(always)]
    pub fn top(&self) -> &T {
        self.peek(0)
    }

    #[inline(always)]
    pub fn truncate(&mut self, start: usize, end: usize) {
        self.values.splice(start..end, []);
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.values.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_stack<T>() -> Stack<T, 256> {
        Stack::new()
    }

    #[test]
    fn push_and_pop_scalars() {
        let mut stack = create_stack();
        stack.push(1);
        stack.push(2);

        assert_eq!(stack.pop(), 2);
        assert_eq!(stack.pop(), 1);
    }

    #[test]
    fn push_and_pop_containers() {
        let mut stack = create_stack();
        stack.push(Box::new(1));
        stack.push(Box::new(2));

        assert_eq!(stack.pop(), Box::new(2));
        assert_eq!(stack.pop(), Box::new(1));
    }

    #[test]
    fn peek() {
        let mut stack = create_stack();
        stack.push(1);
        stack.push(2);

        assert_eq!(stack.peek(0), &2);
        assert_eq!(stack.peek(1), &1);
    }

    #[test]
    fn truncate() {
        let mut stack = create_stack();
        stack.push(1);
        stack.push(2);

        stack.truncate(0, 1);

        assert_eq!(stack.pop(), 2);
    }

    #[test]
    fn truncate_overlapping() {
        let mut stack = create_stack();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        stack.truncate(0, 1);

        assert_eq!(stack.pop(), 3);
        assert_eq!(stack.pop(), 2);
    }

    #[test]
    #[should_panic]
    fn overflow() {
        let mut stack = create_stack();

        loop {
            stack.push(0);
        }
    }
}
