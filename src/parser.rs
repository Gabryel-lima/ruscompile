use crate::ast::*;
use crate::error::{CompilerError, CompilerResult};
use crate::lexer::{Token, TokenInfo};

pub struct Parser {
    tokens: Vec<TokenInfo>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenInfo>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> CompilerResult<Program> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            if let Some(stmt) = self.declaration()? {
                statements.push(stmt);
            }
        }

        Ok(Program { statements })
    }

    fn declaration(&mut self) -> CompilerResult<Option<Statement>> {
        if self.match_token(Token::Var) {
            self.var_declaration().map(Some)
        } else if self.match_token(Token::Func) {
            self.function_declaration().map(Some)
        } else {
            self.statement().map(Some)
        }
    }

    fn var_declaration(&mut self) -> CompilerResult<Statement> {
        let location = self.previous().location.clone();

        let name = if let Some(token_info) = self.advance() {
            if let Token::Identifier(name) = &token_info.token {
                name.clone()
            } else {
                return Err(CompilerError::syntax(
                    token_info.location.line,
                    token_info.location.column,
                    "Esperado nome de variável".to_string(),
                ));
            }
        } else {
            return Err(CompilerError::syntax(0, 0, "Esperado nome de variável".to_string()));
        };

        let var_type = if self.match_token(Token::Colon) {
            self.parse_type()?
        } else {
            Type::Int // Tipo padrão
        };

        let initializer = if self.match_token(Token::Assign) {
            Some(self.expression()?)
        } else {
            None
        };

        self.expect(Token::Semicolon)?;

        Ok(Statement::Declaration(DeclarationStatement {
            name,
            var_type,
            initializer,
            location,
        }))
    }

    fn function_declaration(&mut self) -> CompilerResult<Statement> {
        let location = self.previous().location.clone();

        let name = if let Some(token_info) = self.advance() {
            if let Token::Identifier(name) = &token_info.token {
                name.clone()
            } else {
                return Err(CompilerError::syntax(
                    token_info.location.line,
                    token_info.location.column,
                    "Esperado nome de função".to_string(),
                ));
            }
        } else {
            return Err(CompilerError::syntax(0, 0, "Esperado nome de função".to_string()));
        };

        self.expect(Token::LeftParen)?;

        let mut parameters = Vec::new();
        if !self.check(Token::RightParen) {
            loop {
                let param_name = if let Some(token_info) = self.advance() {
                    if let Token::Identifier(name) = &token_info.token {
                        name.clone()
                    } else {
                        return Err(CompilerError::syntax(
                            token_info.location.line,
                            token_info.location.column,
                            "Esperado nome de parâmetro".to_string(),
                        ));
                    }
                } else {
                    return Err(CompilerError::syntax(0, 0, "Esperado nome de parâmetro".to_string()));
                };

                self.expect(Token::Colon)?;
                let param_type = self.parse_type()?;

                parameters.push(Parameter {
                    name: param_name,
                    param_type,
                    location: self.previous().location.clone(),
                });

                if !self.match_token(Token::Comma) {
                    break;
                }
            }
        }

        self.expect(Token::RightParen)?;

        let return_type = if self.match_token(Token::Arrow) {
            self.parse_type()?
        } else {
            Type::Void
        };

        self.expect(Token::LeftBrace)?;
        let body = self.block_statement()?;

        Ok(Statement::Function(FunctionStatement {
            name,
            parameters,
            return_type,
            body,
            location,
        }))
    }

    fn statement(&mut self) -> CompilerResult<Statement> {
        if self.match_token(Token::If) {
            self.if_statement()
        } else if self.match_token(Token::While) {
            self.while_statement()
        } else if self.match_token(Token::Return) {
            self.return_statement()
        } else if self.match_token(Token::LeftBrace) {
            self.block_statement().map(Statement::Block)
        } else {
            self.expression_statement()
        }
    }

    fn if_statement(&mut self) -> CompilerResult<Statement> {
        let location = self.previous().location.clone();

        self.expect(Token::LeftParen)?;
        let condition = self.expression()?;
        self.expect(Token::RightParen)?;

        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.match_token(Token::Else) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Statement::If(IfStatement {
            condition,
            then_branch,
            else_branch,
            location,
        }))
    }

    fn while_statement(&mut self) -> CompilerResult<Statement> {
        let location = self.previous().location.clone();

        self.expect(Token::LeftParen)?;
        let condition = self.expression()?;
        self.expect(Token::RightParen)?;

        let body = Box::new(self.statement()?);

        Ok(Statement::While(WhileStatement {
            condition,
            body,
            location,
        }))
    }

    fn return_statement(&mut self) -> CompilerResult<Statement> {
        let location = self.previous().location.clone();

        let value = if !self.check(Token::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };

        self.expect(Token::Semicolon)?;

        Ok(Statement::Return(ReturnStatement { value, location }))
    }

    fn block_statement(&mut self) -> CompilerResult<BlockStatement> {
        let location = self.previous().location.clone();
        let mut statements = Vec::new();

        while !self.check(Token::RightBrace) && !self.is_at_end() {
            if let Some(stmt) = self.declaration()? {
                statements.push(stmt);
            }
        }

        if self.is_at_end() {
            return Err(CompilerError::syntax(
                self.previous().location.line,
                self.previous().location.column,
                "Esperado '}' antes do fim do arquivo".to_string(),
            ));
        }

        self.expect(Token::RightBrace)?;

        Ok(BlockStatement {
            statements,
            location,
        })
    }

    fn expression_statement(&mut self) -> CompilerResult<Statement> {
        let expression = self.expression()?;
        let location = self.previous().location.clone();

        self.expect(Token::Semicolon)?;

        Ok(Statement::Expression(ExpressionStatement {
            expression,
            location,
        }))
    }

    fn expression(&mut self) -> CompilerResult<Expression> {
        self.assignment()
    }

    fn assignment(&mut self) -> CompilerResult<Expression> {
        let expr = self.or()?;

        if self.match_token(Token::Assign) {
            let value = self.assignment()?;

            if let Expression::Identifier(identifier) = expr {
                return Ok(Expression::Assignment(AssignmentExpression {
                    target: identifier.name,
                    value: Box::new(value),
                    location: self.previous().location.clone(),
                }));
            }

            return Err(CompilerError::syntax(
                self.previous().location.line,
                self.previous().location.column,
                "Expressão inválida para atribuição".to_string(),
            ));
        }

        Ok(expr)
    }

    fn or(&mut self) -> CompilerResult<Expression> {
        let mut expr = self.and()?;

        while self.match_token(Token::Or) {
            let operator = BinaryOperator::Or;
            let right = Box::new(self.and()?);
            let location = self.previous().location.clone();

            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator,
                right,
                location,
            });
        }

        Ok(expr)
    }

    fn and(&mut self) -> CompilerResult<Expression> {
        let mut expr = self.equality()?;

        while self.match_token(Token::And) {
            let operator = BinaryOperator::And;
            let right = Box::new(self.equality()?);
            let location = self.previous().location.clone();

            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator,
                right,
                location,
            });
        }

        Ok(expr)
    }

    fn equality(&mut self) -> CompilerResult<Expression> {
        let mut expr = self.comparison()?;

        while self.match_token(Token::Equal) || self.match_token(Token::NotEqual) {
            let operator = if self.previous().token == Token::Equal {
                BinaryOperator::Equal
            } else {
                BinaryOperator::NotEqual
            };
            let right = Box::new(self.comparison()?);
            let location = self.previous().location.clone();

            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator,
                right,
                location,
            });
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> CompilerResult<Expression> {
        let mut expr = self.term()?;

        while self.match_token(Token::LessThan)
            || self.match_token(Token::LessThanEqual)
            || self.match_token(Token::GreaterThan)
            || self.match_token(Token::GreaterThanEqual)
        {
            let operator = match self.previous().token {
                Token::LessThan => BinaryOperator::LessThan,
                Token::LessThanEqual => BinaryOperator::LessThanEqual,
                Token::GreaterThan => BinaryOperator::GreaterThan,
                Token::GreaterThanEqual => BinaryOperator::GreaterThanEqual,
                _ => unreachable!(),
            };
            let right = Box::new(self.term()?);
            let location = self.previous().location.clone();

            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator,
                right,
                location,
            });
        }

        Ok(expr)
    }

    fn term(&mut self) -> CompilerResult<Expression> {
        let mut expr = self.factor()?;

        while self.match_token(Token::Plus) || self.match_token(Token::Minus) {
            let operator = if self.previous().token == Token::Plus {
                BinaryOperator::Add
            } else {
                BinaryOperator::Subtract
            };
            let right = Box::new(self.factor()?);
            let location = self.previous().location.clone();

            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator,
                right,
                location,
            });
        }

        Ok(expr)
    }

    fn factor(&mut self) -> CompilerResult<Expression> {
        let mut expr = self.unary()?;

        while self.match_token(Token::Star) || self.match_token(Token::Slash) || self.match_token(Token::Percent) {
            let operator = match self.previous().token {
                Token::Star => BinaryOperator::Multiply,
                Token::Slash => BinaryOperator::Divide,
                Token::Percent => BinaryOperator::Modulo,
                _ => unreachable!(),
            };
            let right = Box::new(self.unary()?);
            let location = self.previous().location.clone();

            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator,
                right,
                location,
            });
        }

        Ok(expr)
    }

    fn unary(&mut self) -> CompilerResult<Expression> {
        if self.match_token(Token::Not) || self.match_token(Token::Minus) {
            let operator = if self.previous().token == Token::Not {
                UnaryOperator::Not
            } else {
                UnaryOperator::Minus
            };
            let operand = Box::new(self.unary()?);
            let location = self.previous().location.clone();

            return Ok(Expression::Unary(UnaryExpression {
                operator,
                operand,
                location,
            }));
        }

        self.call()
    }

    fn call(&mut self) -> CompilerResult<Expression> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(Token::LeftParen) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expression) -> CompilerResult<Expression> {
        let mut arguments = Vec::new();

        if !self.check(Token::RightParen) {
            loop {
                arguments.push(self.expression()?);
                if !self.match_token(Token::Comma) {
                    break;
                }
            }
        }

        let location = self.expect(Token::RightParen)?.location.clone();

        let function_name = if let Expression::Identifier(identifier) = callee {
            identifier.name
        } else {
            return Err(CompilerError::syntax(
                location.line,
                location.column,
                "Esperado nome de função".to_string(),
            ));
        };

        Ok(Expression::Call(CallExpression {
            function: function_name,
            arguments,
            location,
        }))
    }

    fn primary(&mut self) -> CompilerResult<Expression> {
        if let Some(token_info) = self.advance() {
            let location = token_info.location.clone();

            match &token_info.token {
                Token::Integer(n) => Ok(Expression::Literal(LiteralExpression {
                    value: Literal::Integer(*n),
                    location,
                })),
                Token::Float(x) => Ok(Expression::Literal(LiteralExpression {
                    value: Literal::Float(*x),
                    location,
                })),
                Token::String(s) => Ok(Expression::Literal(LiteralExpression {
                    value: Literal::String(s.clone()),
                    location,
                })),
                Token::Boolean(b) => Ok(Expression::Literal(LiteralExpression {
                    value: Literal::Boolean(*b),
                    location,
                })),
                Token::Identifier(name) => Ok(Expression::Identifier(IdentifierExpression {
                    name: name.clone(),
                    location,
                })),
                Token::LeftParen => {
                    let expr = self.expression()?;
                    self.expect(Token::RightParen)?;
                    Ok(expr)
                }
                _ => Err(CompilerError::syntax(
                    location.line,
                    location.column,
                    format!("Expressão inesperada: {:?}", token_info.token),
                )),
            }
        } else {
            Err(CompilerError::syntax(0, 0, "Expressão inesperada no fim do arquivo".to_string()))
        }
    }

    fn parse_type(&mut self) -> CompilerResult<Type> {
        if let Some(token_info) = self.advance() {
            match &token_info.token {
                Token::Int => Ok(Type::Int),
                Token::FloatType => Ok(Type::Float),
                Token::Bool => Ok(Type::Bool),
                Token::StringType => Ok(Type::String),
                Token::Void => Ok(Type::Void),
                _ => Err(CompilerError::syntax(
                    token_info.location.line,
                    token_info.location.column,
                    "Tipo inválido".to_string(),
                )),
            }
        } else {
            Err(CompilerError::syntax(0, 0, "Tipo esperado".to_string()))
        }
    }

    // Métodos auxiliares
    fn match_token(&mut self, token: Token) -> bool {
        if self.check(token.clone()) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check(&self, token: Token) -> bool {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(&self.peek().token) == std::mem::discriminant(&token)
        }
    }

    fn advance(&mut self) -> Option<&TokenInfo> {
        if !self.is_at_end() {
            self.current += 1;
            Some(self.previous())
        } else {
            None
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek().token == Token::Eof
    }

    fn peek(&self) -> &TokenInfo {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &TokenInfo {
        &self.tokens[self.current - 1]
    }

    fn expect(&mut self, token: Token) -> CompilerResult<&TokenInfo> {
        if self.check(token.clone()) {
            Ok(self.advance().unwrap())
        } else {
            Err(CompilerError::syntax(
                self.peek().location.line,
                self.peek().location.column,
                format!("Esperado '{:?}'", token),
            ))
        }
    }
} 