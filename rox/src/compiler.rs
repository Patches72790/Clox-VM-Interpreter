use crate::{Chunk, OpCode, Precedence, RoxNumber, Token, TokenType, Value, DEBUG_MODE};
use std::cell::RefCell;
use std::iter::Peekable;
use std::rc::Rc;
use std::slice::Iter;

pub struct Compiler<'a> {
    chunk: Rc<RefCell<Chunk>>,
    tokens: RefCell<Peekable<Iter<'a, Token>>>,
    previous: RefCell<Option<&'a Token>>,
    current: RefCell<Option<&'a Token>>,
    pub had_error: RefCell<bool>,
    pub panic_mode: RefCell<bool>,
}

type ParseFn<'a> = Box<dyn FnOnce() + 'a>;

struct ParseRule<'a> {
    precedence: Precedence,
    infix_fn: Option<ParseFn<'a>>,
    prefix_fn: Option<ParseFn<'a>>,
}

impl<'a> ParseRule<'a> {
    pub fn new(
        precedence: Precedence,
        infix_fn: Option<ParseFn<'a>>,
        prefix_fn: Option<ParseFn<'a>>,
    ) -> ParseRule<'a> {
        ParseRule {
            precedence,
            infix_fn,
            prefix_fn,
        }
    }
}

impl<'a> Compiler<'a> {
    pub fn new(chunk: Rc<RefCell<Chunk>>, tokens: RefCell<Peekable<Iter<'a, Token>>>) -> Compiler {
        Compiler {
            chunk,
            tokens,
            had_error: RefCell::new(false),
            panic_mode: RefCell::new(false),
            previous: RefCell::new(None),
            current: RefCell::new(None),
        }
    }

    fn get_rule(&'a self, t_type: &'a TokenType) -> ParseRule {
        match t_type {
            TokenType::Plus => ParseRule {
                precedence: Precedence::PrecTerm,
                infix_fn: Some(Box::new(|| self.binary())),
                prefix_fn: None,
            },
            TokenType::Minus => ParseRule {
                precedence: Precedence::PrecTerm,
                infix_fn: Some(Box::new(|| self.binary())),
                prefix_fn: Some(Box::new(|| self.unary())),
            },
            TokenType::Star => ParseRule {
                precedence: Precedence::PrecFactor,
                prefix_fn: None,
                infix_fn: Some(Box::new(|| self.binary())),
            },
            TokenType::Slash => ParseRule {
                precedence: Precedence::PrecFactor,
                prefix_fn: None,
                infix_fn: Some(Box::new(|| self.binary())),
            },
            TokenType::Number(num) => ParseRule {
                precedence: Precedence::PrecNone,
                prefix_fn: Some(Box::new(|| self.number(*num, 1))),
                infix_fn: None,
            },
            TokenType::Semicolon => ParseRule {
                precedence: Precedence::PrecNone,
                prefix_fn: None,
                infix_fn: None,
            },
            TokenType::EOF => ParseRule {
                precedence: Precedence::PrecNone,
                prefix_fn: None,
                infix_fn: None,
            },
            _ => todo!("Unimplemented token type: {:?}", t_type),
        }
    }

    fn advance(&self) {
        // set previous to current token
        let current_tok = *self.current.borrow();
        *(self.previous.borrow_mut()) = current_tok;

        // set current to the next token in scanner tokens
        // until no error token is found
        loop {
            let next_token = match self.tokens.borrow_mut().next() {
                Some(tok) => tok,
                None => panic!("Error getting next token in advance!"),
            };

            if DEBUG_MODE {
                println!("Advanced to Token: {}", next_token);
            }

            *(self.current.borrow_mut()) = Some(next_token);

            if let TokenType::Error(msg) = &next_token.token_type {
                self.error_at_current_token(msg);
            } else {
                break;
            }
        }
    }

    fn consume(&self, t_type: TokenType, message: &str) {
        let current_tok = self
            .current
            .borrow()
            .expect("Error consuming current token!");
        if current_tok.token_type == t_type {
            self.advance();
            return;
        }

        self.error_at_current_token(message);
    }

    fn error_at_current_token(&self, message: &str) {
        self.error_at(
            self.current
                .borrow()
                .expect("Error borrowing current token"),
            message,
        );
    }

    fn error(&self, message: &str) {
        self.error_at(
            self.previous
                .borrow()
                .expect("Error borrowing previous token"),
            message,
        );
    }

    fn error_at(&self, token: &Token, message: &str) {
        // if already in panic, stop parser
        if *self.panic_mode.borrow() {
            return;
        }

        *self.panic_mode.borrow_mut() = true;

        eprintln!(
            "Error at [{}, {}] with message: {}",
            token.line, token.column, message
        );
        *self.had_error.borrow_mut() = true;
    }

    fn expression(&'a self) {
        self.parse(&Precedence::PrecAssign);
    }

    fn number(&'a self, num: RoxNumber, line: usize) {
        self.emit_constant(Value::Number(num), line);
    }

    ///
    /// Writes a constant value to the chunk, bypassing
    /// emit_byte since the Chunk already has a convenience
    /// function for such a task.
    fn emit_constant(&self, value: Value, line: usize) {
        self.chunk.borrow_mut().add_constant(value, line);
    }

    fn grouping(&'a self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    fn unary(&'a self) {
        // find type
        let operator_type = self
            .previous
            .borrow()
            .expect("Error borrowing previous token in unary");

        // compile operand
        self.parse(&Precedence::PrecUnary);

        // emit operator opcode
        match operator_type.token_type {
            TokenType::Minus => self.emit_byte(OpCode::OpNegate),
            _ => panic!(
                "Error parsing unary expression. Unexpected token type: {}",
                operator_type
            ),
        }
    }

    fn binary(&'a self) {
        let operator_type = self
            .previous
            .borrow()
            .expect("Error borrowing previous token in binary");

        // get parse rule
        let rule = self.get_rule(&operator_type.token_type);

        // parse rule with next highest precedence (term -> factor, factor -> unary)
        self.parse(rule.precedence.get_next());

        // emit opcode for token type
        match operator_type.token_type {
            TokenType::Plus => self.emit_byte(OpCode::OpAdd),
            TokenType::Minus => self.emit_byte(OpCode::OpSubtract),
            TokenType::Star => self.emit_byte(OpCode::OpMultiply),
            TokenType::Slash => self.emit_byte(OpCode::OpDivide),
            _ => panic!(
                "Error parsing binary expression. Unexpected token type: {}",
                operator_type
            ),
        }
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

    fn parse(&'a self, precedence: &Precedence) {
        self.advance();
        let prefix_fn = self
            .get_rule(
                &self
                    .previous
                    .borrow()
                    .expect("Error borrowing previous token in parser")
                    .token_type,
            )
            .prefix_fn;

        if let Some(p_fn) = prefix_fn {
            p_fn();
        } else {
            self.error("Expect expression.");
            return;
        }

        while precedence
            <= &self
                .get_rule(&self.current.borrow().unwrap().token_type)
                .precedence
        {
            self.advance();
            let infix_fn = self
                .get_rule(
                    &self
                        .previous
                        .borrow()
                        .expect("Error borrowing previous token in parser")
                        .token_type,
                )
                .infix_fn;

            if let Some(in_fn) = infix_fn {
                in_fn();
            } else {
                self.error("No infix function parsed.");
                return;
            }
        }
    }

    pub fn compile(&'a self) -> bool {
        // prime pump with token to parse
        self.advance();

        // parse expression first with lowest precedence
        self.expression();

        // emit final byte code
        self.end_compiler();

        !*self.had_error.borrow()
    }
}
