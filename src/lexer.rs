use logos::Logos;
use crate::ast::{Location, Literal};
use crate::error::{CompilerError, CompilerResult};

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    // Literais
    #[regex(r"[0-9]+", |lex| lex.slice().parse().unwrap_or(0))]
    Integer(i64),

    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse().unwrap_or(0.0))]
    Float(f64),

    #[regex(r#""([^"]|\\")*""#, |lex| {
        let s = lex.slice();
        s[1..s.len()-1].to_string()
    })]
    String(String),

    #[regex(r"true|false", |lex| lex.slice().parse().unwrap_or(false))]
    Boolean(bool),

    // Identificadores
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    // Operadores
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("==")]
    Equal,
    #[token("!=")]
    NotEqual,
    #[token("<")]
    LessThan,
    #[token("<=")]
    LessThanEqual,
    #[token(">")]
    GreaterThan,
    #[token(">=")]
    GreaterThanEqual,
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("!")]
    Not,
    #[token("=")]
    Assign,

    // Delimitadores
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token(";")]
    Semicolon,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,

    // Palavras-chave
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("while")]
    While,
    #[token("for")]
    For,
    #[token("return")]
    Return,
    #[token("var")]
    Var,
    #[token("func")]
    Func,
    #[token("int")]
    Int,
    #[token("float")]
    FloatType,
    #[token("bool")]
    Bool,
    #[token("string")]
    StringType,
    #[token("void")]
    Void,
    #[token(":")]
    Colon,
    #[token("->")]
    Arrow,

    // Comentários e espaços em branco
    #[regex(r"//[^\n]*", logos::skip)]
    #[regex(r"/\*([^*]|\*+[^*/])*\*+/", logos::skip)]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
    
    // Token especial para fim de arquivo
    Eof,
}

impl Token {
    #[allow(dead_code)]
    pub fn to_literal(&self) -> Option<Literal> {
        match self {
            Token::Integer(n) => Some(Literal::Integer(*n)),
            Token::Float(x) => Some(Literal::Float(*x)),
            Token::String(s) => Some(Literal::String(s.clone())),
            Token::Boolean(b) => Some(Literal::Boolean(*b)),
            _ => None,
        }
    }

    #[allow(dead_code)]
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Token::If | Token::Else | Token::While | Token::For | Token::Return |
            Token::Var | Token::Func | Token::Int | Token::FloatType | Token::Bool |
            Token::StringType | Token::Void
        )
    }

    #[allow(dead_code)]
    pub fn is_type(&self) -> bool {
        matches!(
            self,
            Token::Int | Token::FloatType | Token::Bool | Token::StringType | Token::Void
        )
    }

    // Token EOF será adicionado manualmente no lexer
}

#[derive(Debug, Clone)]
pub struct TokenInfo {
    pub token: Token,
    pub location: Location,
}

pub struct Lexer {
    source: String,
    tokens: Vec<TokenInfo>,
    _current_pos: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: Vec::new(),
            _current_pos: 0,
        }
    }

    pub fn tokenize(&mut self) -> CompilerResult<Vec<TokenInfo>> {
        let mut lexer = Token::lexer(&self.source);
        let mut tokens = Vec::new();
        let source = &self.source;

        while let Some(token) = lexer.next() {
            match token {
                Ok(Token::Error) => {
                    let span = lexer.span();
                    let slice = &source[span.start..span.end];
                    // Calcular linha e coluna do início do token
                    let (line, column) = {
                        let before = &source[..span.start];
                        let line = before.chars().filter(|&c| c == '\n').count() + 1;
                        let last_newline = before.rfind('\n');
                        let column = match last_newline {
                            Some(idx) => before.len() - idx,
                            None => before.len() + 1,
                        };
                        (line, column)
                    };
                    return Err(CompilerError::lexical(
                        line,
                        column,
                        format!("Token inválido: '{}'", slice),
                    ));
                }
                Ok(token) => {
                    let span = lexer.span();
                    let slice = &source[span.start..span.end];
                    // Calcular linha e coluna do início do token
                    let (line, column) = {
                        let before = &source[..span.start];
                        let line = before.chars().filter(|&c| c == '\n').count() + 1;
                        let last_newline = before.rfind('\n');
                        let column = match last_newline {
                            Some(idx) => before.len() - idx,
                            None => before.len() + 1,
                        };
                        (line, column)
                    };
                    let length = slice.len();
                    let location = Location {
                        line,
                        column,
                        length,
                    };

                    tokens.push(TokenInfo {
                        token,
                        location,
                    });
                }
                Err(_) => {
                    let span = lexer.span();
                    let slice = &source[span.start..span.end];
                    let (line, column) = {
                        let before = &source[..span.start];
                        let line = before.chars().filter(|&c| c == '\n').count() + 1;
                        let last_newline = before.rfind('\n');
                        let column = match last_newline {
                            Some(idx) => before.len() - idx,
                            None => before.len() + 1,
                        };
                        (line, column)
                    };
                    return Err(CompilerError::lexical(
                        line,
                        column,
                        format!("Token inválido: '{}'", slice),
                    ));
                }
            }
        }

        // Adicionar token EOF ao final
        // Calcular linha e coluna do final do arquivo
        let (line, column) = {
            let before = &self.source;
            let line = before.chars().filter(|&c| c == '\n').count() + 1;
            let last_newline = before.rfind('\n');
            let column = match last_newline {
                Some(idx) => before.len() - idx,
                None => before.len() + 1,
            };
            (line, column)
        };
        tokens.push(TokenInfo {
            token: Token::EOF,
            location: Location {
                line,
                column,
                length: 0,
            },
        });

        self.tokens = tokens.clone();
        Ok(tokens)
    }

    #[allow(dead_code)]
    pub fn peek(&self, offset: usize) -> Option<&TokenInfo> {
        self.tokens.get(self._current_pos + offset)
    }

    #[allow(dead_code)]
    pub fn current(&self) -> Option<&TokenInfo> {
        self.tokens.get(self._current_pos)
    }

    #[allow(dead_code)]
    pub fn advance(&mut self) -> Option<&TokenInfo> {
        let token = self.tokens.get(self._current_pos);
        if token.is_some() {
            self._current_pos += 1;
        }
        token
    }

    #[allow(dead_code)]
    pub fn expect(&mut self, expected: Token) -> CompilerResult<&TokenInfo> {
        if let Some(token_info) = self.current() {
            let token_discriminant = std::mem::discriminant(&token_info.token);
            let expected_discriminant = std::mem::discriminant(&expected);
            
            if token_discriminant == expected_discriminant {
                let _token_info = token_info.clone();
                self.advance();
                Ok(&self.tokens[self._current_pos - 1])
            } else {
                Err(CompilerError::syntax(
                    token_info.location.line,
                    token_info.location.column,
                    format!("Esperado '{:?}', encontrado '{:?}'", expected, token_info.token),
                ))
            }
        } else {
            Err(CompilerError::syntax(
                0,
                0,
                format!("Esperado '{:?}', mas chegou ao fim do arquivo", expected),
            ))
        }
    }

    #[allow(dead_code)]
    pub fn check(&self, token: Token) -> bool {
        if let Some(token_info) = self.current() {
            std::mem::discriminant(&token_info.token) == std::mem::discriminant(&token)
        } else {
            false
        }
    }

    #[allow(dead_code)]
    pub fn match_token(&mut self, token: Token) -> bool {
        if self.check(token) {
            self.advance();
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let source = "123 45.67 true false \"hello\"";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 5);
        assert!(matches!(tokens[0].token, Token::Integer(123)));
        assert!(matches!(tokens[1].token, Token::Float(45.67)));
        assert!(matches!(tokens[2].token, Token::Boolean(true)));
        assert!(matches!(tokens[3].token, Token::Boolean(false)));
        assert!(matches!(tokens[4].token, Token::String(ref s) if s == "hello"));
    }

    #[test]
    fn test_operators() {
        let source = "+ - * / % == != < <= > >=";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 12);
        assert!(matches!(tokens[0].token, Token::Plus));
        assert!(matches!(tokens[1].token, Token::Minus));
        assert!(matches!(tokens[2].token, Token::Star));
        assert!(matches!(tokens[3].token, Token::Slash));
        assert!(matches!(tokens[4].token, Token::Percent));
        assert!(matches!(tokens[5].token, Token::Equal));
        assert!(matches!(tokens[6].token, Token::NotEqual));
        assert!(matches!(tokens[7].token, Token::LessThan));
        assert!(matches!(tokens[8].token, Token::LessThanEqual));
        assert!(matches!(tokens[9].token, Token::GreaterThan));
        assert!(matches!(tokens[10].token, Token::GreaterThanEqual));
    }

    #[test]
    fn test_keywords() {
        let source = "if else while for return var func";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 7);
        assert!(matches!(tokens[0].token, Token::If));
        assert!(matches!(tokens[1].token, Token::Else));
        assert!(matches!(tokens[2].token, Token::While));
        assert!(matches!(tokens[3].token, Token::For));
        assert!(matches!(tokens[4].token, Token::Return));
        assert!(matches!(tokens[5].token, Token::Var));
        assert!(matches!(tokens[6].token, Token::Func));
    }
} 