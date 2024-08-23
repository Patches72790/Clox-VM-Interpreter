use crate::frontend::{Locals, LOCALS_COUNT};
use crate::opcode::VariableOp;
use crate::{
    Chunk, ObjectType, OpCode, Precedence, RoxNumber, RoxObject, RoxString, Scanner, Token,
    TokenType, Value, DEBUG_MODE,
};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Compiler {
    tokens: Vec<Token>,
    token_idx: usize,
    chunk: Chunk,
    pub had_error: RefCell<bool>,
    pub panic_mode: RefCell<bool>,

    locals: Locals,
    scope_depth: usize,
}

enum ParseFn {
    And,
    Or,
    Binary,
    Unary,
    Literal,
    Grouping,
    Variable(Rc<RoxString>, usize),
    String(Rc<RoxString>, usize),
    Number(RoxNumber, usize),
}

struct ParseRule {
    precedence: Precedence,
    infix_fn: Option<ParseFn>,
    prefix_fn: Option<ParseFn>,
}

impl Compiler {
    pub fn new(tokens: Vec<Token>) -> Compiler {
        Compiler {
            chunk: Chunk::new(),
            tokens,
            token_idx: 0,
            had_error: RefCell::new(false),
            panic_mode: RefCell::new(false),
            scope_depth: 0,
            locals: Locals::new(),
        }
    }

    fn apply_parse_fn(&mut self, parse_fn: ParseFn, can_assign: bool) -> Result<(), String> {
        match parse_fn {
            ParseFn::And => self.and_(can_assign),
            ParseFn::Or => self.or(can_assign),
            ParseFn::Binary => self.binary(can_assign),
            ParseFn::Unary => self.unary(can_assign),
            ParseFn::Literal => self.literal(can_assign),
            ParseFn::Grouping => self.grouping(can_assign),
            ParseFn::Variable(str, line) => self.variable(&str, line, can_assign),
            ParseFn::String(str, line) => self.string(&str, line, can_assign),
            ParseFn::Number(num, line) => self.number(num, line, can_assign),
        }

        Ok(())
    }

    fn get_rule(token: &Token) -> ParseRule {
        let t_type = &token.token_type;
        let line = token.line;

        match t_type {
            TokenType::And => ParseRule {
                precedence: Precedence::PrecAnd,
                infix_fn: Some(ParseFn::And),
                prefix_fn: None,
            },
            TokenType::Or => ParseRule {
                precedence: Precedence::PrecOr,
                infix_fn: Some(ParseFn::Or),
                prefix_fn: None,
            },
            TokenType::Plus => ParseRule {
                precedence: Precedence::PrecTerm,
                infix_fn: Some(ParseFn::Binary),
                prefix_fn: None,
            },
            TokenType::Minus => ParseRule {
                precedence: Precedence::PrecTerm,
                infix_fn: Some(ParseFn::Binary),
                prefix_fn: Some(ParseFn::Unary),
            },
            TokenType::Star => ParseRule {
                precedence: Precedence::PrecFactor,
                prefix_fn: None,
                infix_fn: Some(ParseFn::Binary),
            },
            TokenType::Slash => ParseRule {
                precedence: Precedence::PrecFactor,
                prefix_fn: None,
                infix_fn: Some(ParseFn::Binary),
            },
            TokenType::Number(num) => ParseRule {
                precedence: Precedence::PrecNone,
                prefix_fn: Some(ParseFn::Number(*num, line)),
                infix_fn: None,
            },
            TokenType::True => ParseRule {
                precedence: Precedence::PrecNone,
                prefix_fn: Some(ParseFn::Literal),
                infix_fn: None,
            },
            TokenType::False => ParseRule {
                precedence: Precedence::PrecNone,
                prefix_fn: Some(ParseFn::Literal),
                infix_fn: None,
            },
            TokenType::Nil => ParseRule {
                precedence: Precedence::PrecNone,
                prefix_fn: Some(ParseFn::Literal),
                infix_fn: None,
            },
            TokenType::Bang => ParseRule {
                precedence: Precedence::PrecNone,
                prefix_fn: Some(ParseFn::Unary),
                infix_fn: None,
            },
            TokenType::BangEqual => ParseRule {
                precedence: Precedence::PrecEquality,
                prefix_fn: None,
                infix_fn: Some(ParseFn::Binary),
            },
            TokenType::EqualEqual => ParseRule {
                precedence: Precedence::PrecEquality,
                prefix_fn: None,
                infix_fn: Some(ParseFn::Binary),
            },
            TokenType::Greater => ParseRule {
                precedence: Precedence::PrecComparison,
                prefix_fn: None,
                infix_fn: Some(ParseFn::Binary),
            },
            TokenType::GreaterEqual => ParseRule {
                precedence: Precedence::PrecComparison,
                prefix_fn: None,
                infix_fn: Some(ParseFn::Binary),
            },
            TokenType::Less => ParseRule {
                precedence: Precedence::PrecComparison,
                prefix_fn: None,
                infix_fn: Some(ParseFn::Binary),
            },
            TokenType::LessEqual => ParseRule {
                precedence: Precedence::PrecComparison,
                prefix_fn: None,
                infix_fn: Some(ParseFn::Binary),
            },
            TokenType::LeftParen => ParseRule {
                precedence: Precedence::PrecNone,
                prefix_fn: Some(ParseFn::Grouping),
                infix_fn: None,
            },
            TokenType::RightParen => ParseRule {
                precedence: Precedence::PrecNone,
                prefix_fn: None,
                infix_fn: None,
            },
            TokenType::Semicolon => ParseRule {
                precedence: Precedence::PrecNone,
                prefix_fn: None,
                infix_fn: None,
            },

            /*
                        TokenType::Var => ParseRule {
                            precedence: Precedence::PrecNone,
                            prefix_fn: None,
                            infix_fn: None,
                        },
                        TokenType::RightBrace => ParseRule {
                            precedence: Precedence::PrecNone,
                            prefix_fn: None,
                            infix_fn: None,
                        },
            */
            TokenType::Identifier(id) => ParseRule {
                precedence: Precedence::PrecNone,
                prefix_fn: Some(ParseFn::Variable(id.clone(), line)),
                infix_fn: None,
            },
            TokenType::StringLiteral(str) => ParseRule {
                precedence: Precedence::PrecNone,
                prefix_fn: Some(ParseFn::String(str.clone(), line)),
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

    fn previous(&self) -> &Token {
        &self.tokens[self.token_idx.saturating_sub(1)]
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.token_idx]
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.token_idx += 1;
        }

        self.previous()
    }

    /// A conditional wrapper around advance that checks that
    /// the current token is of type t_type.
    fn match_token(&mut self, t_type: TokenType) -> bool {
        if self.check_token(t_type) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Helper function to check that the current token's
    /// type is equal to t_type.
    fn check_token(&self, t_type: TokenType) -> bool {
        self.peek().token_type == t_type
    }

    fn consume(&mut self, t_type: TokenType, message: &str) {
        let current_tok = self.peek();
        if current_tok.token_type == t_type {
            if DEBUG_MODE {
                println!("Consuming token {}", current_tok);
            }
            self.advance();
            return;
        }

        self.error_at_current_token(message);
    }

    fn error_at_current_token(&self, message: &str) {
        self.error_at(self.peek(), message);
    }

    fn error(&self, message: &str) {
        self.error_at(self.previous(), message);
    }

    fn error_at(&self, token: &Token, message: &str) {
        // if already in panic, stop parser
        if *self.panic_mode.borrow() {
            return;
        }

        (*self.panic_mode.borrow_mut()) = true;

        eprintln!(
            "Error at [{}, {}] with message: {}",
            token.line, token.column, message
        );
        (*self.had_error.borrow_mut()) = true;
    }

    fn synchronize(&mut self) {
        (*self.panic_mode.borrow_mut()) = false;
        let mut current_token_type = &self.peek().token_type;

        while *current_token_type != TokenType::EOF {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match current_token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => (),
            }

            // indiscriminately advance depending on token type until end of statement is found
            self.advance();
            current_token_type = &self.peek().token_type;
        }
    }

    fn expression(&mut self) {
        self.parse(&Precedence::PrecAssign);
    }

    fn declaration(&mut self) {
        if self.match_token(TokenType::Var) {
            self.var_declaration();
        } else {
            self.statement();
        }
        if *self.panic_mode.borrow() {
            self.synchronize();
        }
    }

    fn var_declaration(&mut self) {
        let index = self.parse_variable("Expect variable name.");

        if self.match_token(TokenType::Equal) {
            self.expression();
        } else {
            self.emit_byte(OpCode::OpNil);
        }

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        );

        self.define_variable(index);
    }

    fn declare_variable(&mut self) {
        // for globals
        if self.scope_depth == 0 {
            return;
        }

        let token = self.previous().clone();

        let is_doubly_declared = self
            .locals
            .local_is_doubly_declared(&token, self.scope_depth);

        if is_doubly_declared {
            self.error("Already a variable with this name in scope.");
            return;
        }

        self.add_local(&token);
    }

    fn add_local(&mut self, token: &Token) {
        let locals_count = self.locals.size();
        if locals_count == LOCALS_COUNT {
            self.error("Too many local variables in function.");
            return;
        }

        self.locals.add_local(token, self.scope_depth);
    }

    fn define_variable(&mut self, index: usize) {
        let scope_depth = self.scope_depth;
        if scope_depth > 0 {
            self.locals.initialize_variable(scope_depth);
            return;
        }

        self.emit_byte(OpCode::OpDefineGlobal(index));
    }

    fn statement(&mut self) {
        if self.match_token(TokenType::Print) {
            self.print_statement();
        } else if self.match_token(TokenType::For) {
            self.for_statement();
        } else if self.match_token(TokenType::If) {
            self.if_statement();
        } else if self.match_token(TokenType::While) {
            self.while_statement();
        } else if self.match_token(TokenType::LeftBrace) {
            self.begin_scope();
            self.block();
            self.end_scope();
        } else {
            self.expression_statement();
        }
    }

    fn print_statement(&mut self) {
        self.expression();
        self.consume(TokenType::Semicolon, "Expected ';' after value.");
        self.emit_byte(OpCode::OpPrint);
    }

    fn expression_statement(&mut self) {
        self.expression();
        self.consume(
            TokenType::Semicolon,
            "Expected ';' after expression statement.",
        );
        self.emit_byte(OpCode::OpPop);
    }

    fn for_statement(&mut self) {
        self.begin_scope();
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.");

        // compile initialization statement
        if self.match_token(TokenType::Semicolon) {
            // no initializer
        } else if self.match_token(TokenType::Var) {
            self.var_declaration();
        } else {
            self.expression_statement();
        }

        let mut loop_start = self.chunk.count();

        // compile conditional statement
        let mut exit_jump = None;
        if !self.match_token(TokenType::Semicolon) {
            self.expression();
            self.consume(TokenType::Semicolon, "Expect ';' after loop condition.");

            exit_jump = Some(self.emit_jump(OpCode::OpJumpIfFalse(None)));
            self.emit_byte(OpCode::OpPop);
        }

        // compile increment statement
        if !self.match_token(TokenType::RightParen) {
            let body_jump = self.emit_jump(OpCode::OpJump(None));
            let incr_start = self.chunk.count();

            self.expression();
            // TODO => Why does this fix the tests?
            //self.emit_byte(OpCode::OpPop);
            self.consume(TokenType::RightParen, "Expect ')' after for clauses.");

            self.emit_loop(loop_start);
            loop_start = incr_start;
            self.patch_jump(body_jump, OpCode::OpJump(None));
        }

        self.statement();
        self.emit_loop(loop_start);

        // compile code to quit for loop early when condition is false
        if let Some(exit_jump_offset) = exit_jump {
            self.patch_jump(exit_jump_offset, OpCode::OpJumpIfFalse(None));
            self.emit_byte(OpCode::OpPop);
        }

        self.end_scope();
    }

    fn while_statement(&mut self) {
        let loop_start = self.chunk.count();

        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.");
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after condition.");

        let exit_jump = self.emit_jump(OpCode::OpJumpIfFalse(None));
        self.emit_jump(OpCode::OpPop);
        self.statement();
        self.emit_loop(loop_start);

        self.patch_jump(exit_jump, OpCode::OpJumpIfFalse(None));
        self.emit_byte(OpCode::OpPop);
    }

    fn if_statement(&mut self) {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.");
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after condition.");

        let then_jump = self.emit_jump(OpCode::OpJumpIfFalse(None));
        self.emit_byte(OpCode::OpPop);
        self.statement();

        let else_jump = self.emit_jump(OpCode::OpJump(None));

        self.patch_jump(then_jump, OpCode::OpJumpIfFalse(None));
        self.emit_byte(OpCode::OpPop);

        if self.match_token(TokenType::Else) {
            self.statement();
        }
        self.patch_jump(else_jump, OpCode::OpJump(None));
    }

    fn emit_jump(&mut self, instruction: OpCode) -> usize {
        self.emit_byte(instruction);
        self.chunk.count() - 1
    }

    fn patch_jump(&mut self, offset: usize, opcode: OpCode) {
        let jump = self.chunk.count() - offset - 1;

        // patch in the jump offset from the jump opcode to past the then clause
        match opcode {
            OpCode::OpJumpIfFalse(_) => self.chunk.code[offset] = OpCode::OpJumpIfFalse(Some(jump)),
            OpCode::OpJump(_) => self.chunk.code[offset] = OpCode::OpJump(Some(jump)),
            _ => (),
        }
    }

    fn block(&mut self) {
        while !self.check_token(TokenType::RightBrace) && !self.check_token(TokenType::EOF) {
            self.declaration();
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.");
    }

    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    fn end_scope(&mut self) {
        self.scope_depth -= 1;
        let scope_depth = self.scope_depth;

        let num_removed = self.locals.remove_locals(scope_depth);

        for _ in 0..num_removed {
            self.emit_byte(OpCode::OpPop);
        }
    }

    fn and_(&mut self, _can_assign: bool) {
        let end_jump = self.emit_jump(OpCode::OpJumpIfFalse(None));

        self.emit_byte(OpCode::OpPop);
        self.parse(&Precedence::PrecAnd);

        self.patch_jump(end_jump, OpCode::OpJumpIfFalse(None));
    }

    fn or(&mut self, _can_assign: bool) {
        let else_jump = self.emit_jump(OpCode::OpJumpIfFalse(None));
        let end_jump = self.emit_jump(OpCode::OpJump(None));

        self.patch_jump(else_jump, OpCode::OpJumpIfFalse(None));
        self.emit_byte(OpCode::OpPop);

        self.parse(&Precedence::PrecOr);
        self.patch_jump(end_jump, OpCode::OpJump(None));
    }

    fn number(&mut self, num: RoxNumber, line: usize, _can_assign: bool) {
        self.emit_constant(Value::Number(num), line);
    }

    /// Writes a constant value to the chunk, bypassing
    /// emit_byte since the Chunk already has a convenience
    /// function for such a task.
    fn emit_constant(&mut self, value: Value, line: usize) {
        self.chunk.add_constant(value, line);
    }

    fn emit_identifier_constant(
        &mut self,
        string_value: &Rc<RoxString>,
        line: usize,
        variable_op: VariableOp,
    ) -> usize {
        // need to write string to constants array in chunk
        self.chunk
            .add_identifier_constant(string_value, line, variable_op)
    }

    fn grouping(&mut self, _can_assign: bool) {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    fn string(&mut self, string: &Rc<RoxString>, line: usize, _can_assign: bool) {
        let new_rox_object =
            RoxObject::new(ObjectType::ObjString(RoxString::new(&Rc::clone(string))));
        self.emit_constant(Value::Object(new_rox_object), line);
    }

    fn variable(&mut self, id: &Rc<RoxString>, line: usize, can_assign: bool) {
        let (is_initialized, is_local_id) = self.locals.resolve_local(id);

        if !is_initialized {
            self.error("Can't read local variable in its own initializer.");
        }

        // locals live on the stack at runtime
        if let Some(local_idx) = is_local_id {
            if can_assign && self.match_token(TokenType::Equal) {
                self.expression();
                self.emit_byte(OpCode::OpSetLocal(local_idx));
            } else {
                self.emit_byte(OpCode::OpGetLocal(local_idx));
            }
        } else {
            // globals live in globals list
            if can_assign && self.match_token(TokenType::Equal) {
                self.expression();
                self.chunk
                    .add_identifier_constant(id, line, VariableOp::SetGlobal);
            } else {
                self.chunk
                    .add_identifier_constant(id, line, VariableOp::GetGlobal);
            }
        }
    }

    fn literal(&mut self, _can_assign: bool) {
        match self.previous().token_type {
            TokenType::True => self.emit_byte(OpCode::OpTrue),
            TokenType::False => self.emit_byte(OpCode::OpFalse),
            TokenType::Nil => self.emit_byte(OpCode::OpNil),
            _ => (), // never will be here because literal only used for these three types
        }
    }

    fn unary(&mut self, _can_assign: bool) {
        // find type
        let operator_type = self.previous().clone();

        // compile operand
        self.parse(&Precedence::PrecUnary);

        // emit operator opcode
        match operator_type.token_type {
            TokenType::Minus => self.emit_byte(OpCode::OpNegate),
            TokenType::Bang => self.emit_byte(OpCode::OpNot),
            _ => panic!(
                "Error parsing unary expression. Unexpected token type: {}",
                operator_type
            ),
        }
    }

    fn binary(&mut self, _can_assign: bool) {
        let operator_type = self.previous().clone();

        // get parse rule
        let rule = Compiler::get_rule(&operator_type);

        // parse rule with next highest precedence (term -> factor, factor -> unary)
        self.parse(rule.precedence.get_next());

        // emit opcode for token type
        match operator_type.token_type {
            TokenType::Plus => self.emit_byte(OpCode::OpAdd),
            TokenType::Minus => self.emit_byte(OpCode::OpSubtract),
            TokenType::Star => self.emit_byte(OpCode::OpMultiply),
            TokenType::Slash => self.emit_byte(OpCode::OpDivide),
            TokenType::BangEqual => self.emit_bytes(OpCode::OpEqual, OpCode::OpNot),
            TokenType::EqualEqual => self.emit_byte(OpCode::OpEqual),
            TokenType::Greater => self.emit_byte(OpCode::OpGreater),
            TokenType::GreaterEqual => self.emit_bytes(OpCode::OpLess, OpCode::OpNot), // (a >= b) == !(a < b)
            TokenType::Less => self.emit_byte(OpCode::OpLess),
            TokenType::LessEqual => self.emit_bytes(OpCode::OpGreater, OpCode::OpNot), // (a <= b) == !(a > b)
            _ => panic!(
                "Error parsing binary expression. Unexpected token type: {}",
                operator_type
            ),
        }
    }

    fn emit_loop(&mut self, loop_start: usize) {
        let offset = self.chunk.count() - loop_start + 1;
        if offset > u16::MAX.into() {
            self.error("Loop body too large");
        }

        self.emit_byte(OpCode::OpLoop(offset));
    }

    fn emit_bytes(&mut self, byte1: OpCode, byte2: OpCode) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn emit_byte(&mut self, byte: OpCode) {
        let line = self.previous().line;
        self.chunk.write_chunk(byte, line);
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::OpReturn(0));
    }

    fn end_compiler(&mut self) {
        self.emit_return();
    }

    fn parse(&mut self, precedence: &Precedence) {
        // advance cursor
        self.advance();

        let ParseRule { prefix_fn, .. } = Compiler::get_rule(self.previous());

        let can_assign = precedence <= &Precedence::PrecAssign;

        // call prefix parsing function if present
        if let Some(parse_fn) = prefix_fn {
            self.apply_parse_fn(parse_fn, can_assign)
                .expect("Error applying parse func");
        } else if self.previous().token_type == TokenType::EOF {
            return;
        } else {
            self.error(&format!(
                "No prefix function parsed for precedence {}.",
                precedence
            ));
            return;
        }

        // check that current precedence is less than current_token's precedence
        while precedence <= &Compiler::get_rule(self.peek()).precedence {
            // advance cursor and execute infix parsing function
            self.advance();

            let ParseRule { infix_fn, .. } = Compiler::get_rule(self.previous());

            if let Some(parse_fn) = infix_fn {
                self.apply_parse_fn(parse_fn, can_assign)
                    .expect("Error applying parse func");
            } else if self.previous().token_type == TokenType::EOF {
                return;
            } else {
                self.error("No infix function parsed.");
                return;
            }

            if can_assign && self.match_token(TokenType::Equal) {
                self.error("Invalid assignment target.");
            }
        }
    }

    fn parse_variable(&mut self, msg: &str) -> usize {
        // TODO -- how to make parse variable work here without consuming blank ID?
        self.consume(TokenType::Identifier(Rc::new(RoxString::new(""))), msg);

        let previous = self.previous().clone();
        let previous_token_value = match &previous.token_type {
            TokenType::Identifier(str) => str,
            _ => panic!(
                "Error did not find identifier when parsing previous token for variable {}",
                previous
            ),
        };

        self.declare_variable();
        // don't add a local and a global below
        if self.scope_depth > 0 {
            return 0;
        }

        self.emit_identifier_constant(previous_token_value, previous.line, VariableOp::Define)
    }

    pub fn compile(source: &str) -> Result<Chunk, String> {
        let tokens = Scanner::new().scan_tokens(source);
        let mut compiler = Self::new(tokens.to_vec());

        // parse sequence of declarations and statements
        while !compiler.match_token(TokenType::EOF) {
            compiler.declaration();
        }

        // emit final byte code
        compiler.end_compiler();

        Ok(compiler.chunk.clone())
    }
}
