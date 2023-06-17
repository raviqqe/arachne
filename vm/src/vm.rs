use crate::{
    decode::{decode_bytes, decode_f64, decode_u16, decode_u32, decode_u8},
    frame::Frame,
    stack::Stack,
    Instruction,
};
use num_traits::FromPrimitive;
use runtime::{Closure, NIL};
use std::str;

macro_rules! binary_operation {
    ($self:expr, $operator:tt) => {
        let value = (|| {
            let rhs = $self.stack.pop().to_float64()?;
            let lhs = $self.stack.pop().to_float64()?;

            Some((lhs.to_f64() $operator rhs.to_f64()).into())
        })()
        .unwrap_or(NIL);

        $self.stack.push(value);
    };
}

macro_rules! dispatch {
    ($self:expr, $codes:expr) => {
        if $self.program_counter >= $codes.len() {
            break;
        }

        $self.get_instruction($codes)($self, $codes);
    };
}

const INSTRUCTIONS: &[fn(&mut Vm, &[u8])] = &[
    Vm::add,
    Vm::branch,
    Vm::call,
    Vm::close,
    Vm::divide,
    Vm::drop,
    Vm::dump,
    Vm::environment,
    Vm::equal,
    Vm::float64,
    Vm::get,
    Vm::integer32,
    Vm::jump,
    Vm::length,
    Vm::multiply,
    Vm::nil,
    Vm::peek,
    Vm::r#return,
    Vm::set,
    Vm::subtract,
    Vm::symbol,
    Vm::tail_call,
];

pub struct Vm {
    program_counter: usize,
    stack: Stack,
    frames: Vec<Frame>,
}

impl Vm {
    pub fn new(stack_size: usize) -> Self {
        Self {
            program_counter: 0,
            stack: Stack::new(stack_size),
            frames: Default::default(),
        }
    }

    pub fn run(&mut self, codes: &[u8]) {
        loop {
            dispatch!(self, codes);
            dispatch!(self, codes);
            dispatch!(self, codes);
            dispatch!(self, codes);
            dispatch!(self, codes);
            dispatch!(self, codes);
            dispatch!(self, codes);
            dispatch!(self, codes);
        }
    }

    #[inline(always)]
    fn get_instruction(&mut self, codes: &[u8]) -> fn(&mut Vm, &[u8]) {
        INSTRUCTIONS[Instruction::from_u8(self.read_u8(codes)).expect("valid instruction") as usize]
    }

    #[inline(always)]
    fn nil(&mut self, _codes: &[u8]) {
        self.stack.push(NIL)
    }

    #[inline(always)]
    fn float64(&mut self, codes: &[u8]) {
        let value = self.read_f64(codes);
        self.stack.push(value.into());
    }

    #[inline(always)]
    fn integer32(&mut self, codes: &[u8]) {
        let value = self.read_u32(codes);
        self.stack.push(value.into());
    }

    #[inline(always)]
    fn symbol(&mut self, codes: &[u8]) {
        let len = self.read_u8(codes);
        let value = str::from_utf8(self.read_bytes(codes, len as usize))
            .unwrap()
            .into();

        self.stack.push(value);
    }

    #[inline(always)]
    fn get(&mut self, _codes: &[u8]) {
        let value = (|| {
            let index = self.stack.pop();
            let array = self.stack.pop().into_array()?;

            Some(array.get(index).clone())
        })()
        .unwrap_or(NIL);

        self.stack.push(value);
    }

    #[inline(always)]
    fn set(&mut self, _codes: &[u8]) {
        let value = (|| {
            let value = self.stack.pop();
            let index = self.stack.pop();
            let array = self.stack.pop().into_array()?;

            Some(array.set(index, value).into())
        })()
        .unwrap_or(NIL);

        self.stack.push(value);
    }

    #[inline(always)]
    fn length(&mut self, _codes: &[u8]) {
        let value = (|| Some(self.stack.pop().into_array()?.len().into()))().unwrap_or(NIL);

        self.stack.push(value);
    }

    #[inline(always)]
    fn add(&mut self, _codes: &[u8]) {
        binary_operation!(self, +);
    }

    #[inline(always)]
    fn subtract(&mut self, _codes: &[u8]) {
        binary_operation!(self, -);
    }

    #[inline(always)]
    fn multiply(&mut self, _codes: &[u8]) {
        binary_operation!(self, *);
    }

    #[inline(always)]
    fn divide(&mut self, _codes: &[u8]) {
        binary_operation!(self, /);
    }

    #[inline(always)]
    fn drop(&mut self, _codes: &[u8]) {
        self.stack.pop();
    }

    #[inline(always)]
    fn dump(&mut self, _codes: &[u8]) {
        let value = self.stack.pop();

        println!("{}", value);

        self.stack.push(value);
    }

    #[inline(always)]
    fn call(&mut self, codes: &[u8]) {
        let arity = self.read_u8(codes) as usize;

        self.frames.push(Frame::new(
            self.program_counter as u32,
            (self.stack.len() - arity - 1) as u32,
        ));

        self.call_function(arity)
    }

    #[inline(always)]
    fn tail_call(&mut self, codes: &[u8]) {
        let arity = self.read_u8(codes) as usize;

        let frame = self.frames.last().expect("frame");
        self.stack
            .truncate(frame.pointer() as usize, self.stack.len() - arity - 1);

        self.call_function(arity)
    }

    #[inline(always)]
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

    #[inline(always)]
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

    #[inline(always)]
    fn peek(&mut self, codes: &[u8]) {
        // TODO Move local variables when possible.
        let index = self.read_u8(codes);

        self.stack.push(self.stack.peek(index as usize).clone());
    }

    #[inline(always)]
    fn equal(&mut self, _codes: &[u8]) {
        let rhs = self.stack.pop();
        let lhs = self.stack.pop();

        self.stack.push(((lhs == rhs) as usize as f64).into());
    }

    #[inline(always)]
    fn jump(&mut self, codes: &[u8]) {
        let address = self.read_u16(codes);

        self.program_counter = self
            .program_counter
            .wrapping_add(address as i16 as isize as usize);
    }

    #[inline(always)]
    fn branch(&mut self, codes: &[u8]) {
        let address = self.read_u16(codes);
        let value = self.stack.pop();

        if value != NIL {
            self.program_counter = self
                .program_counter
                .wrapping_add(address as i16 as isize as usize);
        }
    }

    #[inline(always)]
    fn r#return(&mut self, _codes: &[u8]) {
        let value = self.stack.pop();
        let frame = self.frames.pop().expect("frame");

        while self.stack.len() > frame.pointer() as usize {
            self.stack.pop();
        }

        self.program_counter = frame.return_address() as usize;

        self.stack.push(value);
    }

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

    fn read_f64(&mut self, codes: &[u8]) -> f64 {
        decode_f64(codes, &mut self.program_counter)
    }

    fn read_u32(&mut self, codes: &[u8]) -> u32 {
        decode_u32(codes, &mut self.program_counter)
    }

    fn read_u16(&mut self, codes: &[u8]) -> u16 {
        decode_u16(codes, &mut self.program_counter)
    }

    fn read_u8(&mut self, codes: &[u8]) -> u8 {
        decode_u8(codes, &mut self.program_counter)
    }

    fn read_bytes<'a>(&mut self, codes: &'a [u8], len: usize) -> &'a [u8] {
        decode_bytes(codes, len, &mut self.program_counter)
    }
}
