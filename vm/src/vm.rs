use crate::{
    decode::{decode_bytes, decode_f64, decode_u16, decode_u32, decode_u8, decode_u8_option},
    frame::Frame,
    stack::Stack,
    Instruction,
};
use runtime::{Closure, NIL};
use std::str;

macro_rules! binary_operation {
    ($self:expr, $operator:tt) => {
        let value = (|| {
            let rhs = $self.stack.pop().into_float64()?.to_f64();
            let lhs = $self.stack.pop().into_float64()?.to_f64();

            Some((lhs $operator rhs).into())
        })()
        .unwrap_or(NIL);

        $self.stack.push(value);
    };
}

#[derive(Debug, Default)]
pub struct Vm {
    program_counter: usize,
    stack: Stack,
    frames: Vec<Frame>,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            program_counter: 0,
            stack: Stack::new(),
            frames: Default::default(),
        }
    }

    pub fn run(&mut self, codes: &[u8]) {
        while let Some(instruction) = decode_u8_option(codes, &mut self.program_counter) {
            match instruction {
                Instruction::NIL => self.nil(),
                Instruction::FLOAT64 => self.float64(codes),
                Instruction::INTEGER32 => self.integer32(codes),
                Instruction::SYMBOL => self.symbol(codes),
                Instruction::GET => self.get(),
                Instruction::SET => self.set(),
                Instruction::LENGTH => self.length(),
                Instruction::ADD => self.add(),
                Instruction::SUBTRACT => self.subtract(),
                Instruction::MULTIPLY => self.multiply(),
                Instruction::DIVIDE => self.divide(),
                Instruction::DROP => self.drop(),
                Instruction::DUMP => self.dump(),
                Instruction::CALL => self.call(codes),
                Instruction::TAIL_CALL => self.tail_call(codes),
                Instruction::CLOSE => self.close(codes),
                Instruction::ENVIRONMENT => self.environment(codes),
                Instruction::PEEK => self.peek(codes),
                Instruction::EQUAL => self.equal(),
                Instruction::LESS_THAN => self.less_than(),
                Instruction::NOT => self.not(),
                Instruction::AND => self.and(),
                Instruction::OR => self.or(),
                Instruction::JUMP => self.jump(codes),
                Instruction::BRANCH => self.branch(codes),
                Instruction::RETURN => self.r#return(),
                _ => panic!("invalid instruction"),
            }
        }
    }

    fn nil(&mut self) {
        self.stack.push(NIL)
    }

    fn float64(&mut self, codes: &[u8]) {
        let value = self.read_f64(codes);
        self.stack.push(value.into());
    }

    fn integer32(&mut self, codes: &[u8]) {
        let value = self.read_u32(codes);
        self.stack.push(value.into());
    }

    fn symbol(&mut self, codes: &[u8]) {
        let len = self.read_u8(codes);
        let value = str::from_utf8(self.read_bytes(codes, len as usize))
            .unwrap()
            .into();

        self.stack.push(value);
    }

    fn get(&mut self) {
        let value = (|| {
            let index = self.stack.pop();
            let array = self.stack.pop().into_array()?;

            Some(array.get(index).clone())
        })()
        .unwrap_or(NIL);

        self.stack.push(value);
    }

    fn set(&mut self) {
        let value = (|| {
            let value = self.stack.pop();
            let index = self.stack.pop();
            let array = self.stack.pop().into_array()?;

            Some(array.set(index, value).into())
        })()
        .unwrap_or(NIL);

        self.stack.push(value);
    }

    fn length(&mut self) {
        let value = (|| Some(self.stack.pop().into_array()?.len().into()))().unwrap_or(NIL);

        self.stack.push(value);
    }

    fn add(&mut self) {
        binary_operation!(self, +);
    }

    fn subtract(&mut self) {
        binary_operation!(self, -);
    }

    fn multiply(&mut self) {
        binary_operation!(self, *);
    }

    fn divide(&mut self) {
        binary_operation!(self, /);
    }

    fn drop(&mut self) {
        self.stack.pop();
    }

    fn dump(&mut self) {
        let value = self.stack.pop();

        println!("{}", value);

        self.stack.push(value);
    }

    fn call(&mut self, codes: &[u8]) {
        let arity = self.read_u8(codes) as usize;

        self.frames.push(Frame::new(
            (self.stack.len() - arity - 1) as u32,
            self.program_counter as u32,
        ));

        self.call_function(arity)
    }

    fn tail_call(&mut self, codes: &[u8]) {
        let arity = self.read_u8(codes) as usize;

        let frame = self.frames.last().expect("frame");
        self.stack
            .truncate(frame.pointer() as usize, self.stack.len() - arity - 1);

        self.call_function(arity)
    }

    fn close(&mut self, codes: &[u8]) {
        let id = self.read_u32(codes);
        let arity = self.read_u8(codes);
        let environment_size = self.read_u8(codes);
        let mut closure = Closure::new(id, arity, environment_size);

        for index in (0..environment_size).rev() {
            let value = self.stack.pop();

            closure.write_environment(index as usize, value);
        }

        self.stack.push(closure.into());
    }

    fn environment(&mut self, codes: &[u8]) {
        let pointer = self.frames.last().expect("frame").pointer();
        let index = self.read_u8(codes) as usize;

        self.stack.push(
            self.stack
                .peek(self.stack.len() - pointer as usize)
                .as_closure()
                .expect("closure")
                .get_environment(index)
                .clone(),
        );
    }

    fn peek(&mut self, codes: &[u8]) {
        // TODO Move local variables when possible.
        let index = self.read_u8(codes);

        self.stack.push(self.stack.peek(index as usize).clone());
    }

    fn equal(&mut self) {
        let rhs = self.stack.pop();
        let lhs = self.stack.pop();

        self.stack.push(((lhs == rhs) as usize as f64).into());
    }

    fn less_than(&mut self) {
        let rhs = self.stack.pop();
        let lhs = self.stack.pop();

        self.stack.push(((lhs < rhs) as usize as f64).into());
    }

    fn not(&mut self) {
        let value = self.stack.pop();

        self.stack
            .push(if value.is_nil() { 1.0.into() } else { NIL });
    }

    fn and(&mut self) {
        let rhs = self.stack.pop();
        let lhs = self.stack.pop();

        self.stack.push(if lhs.is_nil() { lhs } else { rhs });
    }

    fn or(&mut self) {
        let rhs = self.stack.pop();
        let lhs = self.stack.pop();

        self.stack.push(if lhs.is_nil() { rhs } else { lhs });
    }

    fn jump(&mut self, codes: &[u8]) {
        let address = self.read_u16(codes);

        self.program_counter = self
            .program_counter
            .wrapping_add(address as i16 as isize as usize);
    }

    fn branch(&mut self, codes: &[u8]) {
        let address = self.read_u16(codes);
        let value = self.stack.pop();

        if value != NIL {
            self.program_counter = self
                .program_counter
                .wrapping_add(address as i16 as isize as usize);
        }
    }

    fn r#return(&mut self) {
        let value = self.stack.pop();
        let frame = self.frames.pop().expect("frame");

        while self.stack.len() > frame.pointer() as usize {
            self.stack.pop();
        }

        self.program_counter = frame.return_address() as usize;

        self.stack.push(value);
    }

    #[inline(always)]
    fn call_function(&mut self, arity: usize) {
        if let Some(closure) = self.stack.peek(arity).as_closure() {
            let id = closure.id();
            let closure_arity = closure.arity() as usize;

            self.program_counter = id as usize;

            for _ in 0..arity.saturating_sub(closure_arity) {
                self.stack.pop();
            }

            for _ in 0..closure_arity.saturating_sub(arity) {
                self.stack.push(NIL);
            }
        } else {
            for _ in 0..arity + 1 {
                self.stack.pop();
            }

            self.stack.push(NIL);
        }
    }

    #[inline(always)]
    fn read_f64(&mut self, codes: &[u8]) -> f64 {
        decode_f64(codes, &mut self.program_counter)
    }

    #[inline(always)]
    fn read_u32(&mut self, codes: &[u8]) -> u32 {
        decode_u32(codes, &mut self.program_counter)
    }

    #[inline(always)]
    fn read_u16(&mut self, codes: &[u8]) -> u16 {
        decode_u16(codes, &mut self.program_counter)
    }

    #[inline(always)]
    fn read_u8(&mut self, codes: &[u8]) -> u8 {
        decode_u8(codes, &mut self.program_counter)
    }

    #[inline(always)]
    fn read_bytes<'a>(&mut self, codes: &'a [u8], len: usize) -> &'a [u8] {
        decode_bytes(codes, len, &mut self.program_counter)
    }
}
