use crate::{
    Chunk, OpCode, Parser, Precedence, RoxNumber, Token, TokenStream, TokenType, Value, DEBUG_MODE,
};
use std::cell::RefCell;
use std::iter::Peekable;
use std::rc::Rc;
use std::slice::Iter;

pub struct Compiler<'a> {
    //    func_table: RefCell<[Box<dyn FnMut()>; 2]>,
    chunk: Rc<RefCell<Chunk>>,
    tokens: RefCell<Peekable<Iter<'a, Token>>>,
    func_table: RefCell<[fn(); 2]>,
    pub had_error: bool,
    pub panic_mode: bool,
}

static FUNC_TABLE: [fn(); 2] = [some_func, other_func];

fn some_func() {
    println!("I'm a compiler function!");
}

fn other_func() {
    println!("I'm another compiler function!");
}

impl<'a> Compiler<'a> {
    pub fn new(chunk: Rc<RefCell<Chunk>>, tokens: RefCell<Peekable<Iter<'a, Token>>>) -> Compiler {
        Compiler {
            chunk,
            tokens,
            func_table: RefCell::new(FUNC_TABLE),
            had_error: false,
            panic_mode: false,
        }
    }

    fn expression(&self) {}

    fn number(&self, num: RoxNumber, line: usize) {
        self.emit_constant(Value::Number(num), line);
    }

    ///
    /// Writes a constant value to the chunk, bypassing
    /// emit_byte since the Chunk already has a convenience
    /// function for such a task.
    fn emit_constant(&self, value: Value, line: usize) {
        self.chunk.borrow_mut().add_constant(value, line);
    }

    fn emit_byte(&self, byte: OpCode) {
        self.chunk.borrow_mut().write_chunk(byte, 1);
    }

    fn emit_return(&self) {
        self.emit_byte(OpCode::OpReturn(0));
    }

    fn end_compiler(&self) {
        self.emit_return();
    }

    fn parse(&self, precedence: Precedence) {
        while let Some(token) = self.tokens.borrow_mut().next() {
            println!("Parsed token: {token} with precedence {:?}", *precedence);
            match token.token_type {
                TokenType::Number(num) => self.number(num, token.line),
                _ => (),
            }
        }
    }

    pub fn compile(&self) -> bool {
        // tokens already scanned
        // jump straight into parsing
        self.parse(Precedence::PrecPrimary);

        let first = Precedence::PrecAnd;
        let second = Precedence::PrecTerm;

        println!("{:?} has greater precedence than {:?}: {}", second, first, first < second);

        if DEBUG_MODE {
            for token in self.tokens.borrow_mut().next() {
                println!("Compiler has a token: {}", token);
            }
        }

        //        self.end_compiler();
        !self.had_error
    }
}
