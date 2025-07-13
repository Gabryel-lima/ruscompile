use std::fmt;
use std::io;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("Erro ao ler arquivo {0}: {1}")]
    FileReadError(PathBuf, io::Error),

    #[error("Erro ao escrever arquivo {0}: {1}")]
    FileWriteError(PathBuf, io::Error),

    #[error("Erro léxico na linha {line}, coluna {column}: {message}")]
    LexicalError {
        line: usize,
        column: usize,
        message: String,
    },

    #[error("Erro de sintaxe na linha {line}, coluna {column}: {message}")]
    SyntaxError {
        line: usize,
        column: usize,
        message: String,
    },

    #[error("Erro semântico: {message}")]
    SemanticError {
        message: String,
        line: Option<usize>,
        column: Option<usize>,
    },

    #[error("Erro de tipo: {message}")]
    TypeError {
        message: String,
        line: Option<usize>,
        column: Option<usize>,
    },

    #[error("Erro de geração de código: {message}")]
    CodeGenError {
        message: String,
    },

    #[error("Erro interno do compilador: {message}")]
    InternalError {
        message: String,
    },
}

impl CompilerError {
    pub fn lexical(line: usize, column: usize, message: impl Into<String>) -> Self {
        Self::LexicalError {
            line,
            column,
            message: message.into(),
        }
    }

    pub fn syntax(line: usize, column: usize, message: impl Into<String>) -> Self {
        Self::SyntaxError {
            line,
            column,
            message: message.into(),
        }
    }

    pub fn semantic(message: impl Into<String>) -> Self {
        Self::SemanticError {
            message: message.into(),
            line: None,
            column: None,
        }
    }

    pub fn semantic_with_location(
        message: impl Into<String>,
        line: usize,
        column: usize,
    ) -> Self {
        Self::SemanticError {
            message: message.into(),
            line: Some(line),
            column: Some(column),
        }
    }

    #[allow(dead_code)]
    pub fn type_error(message: impl Into<String>) -> Self {
        Self::TypeError {
            message: message.into(),
            line: None,
            column: None,
        }
    }

    pub fn type_error_with_location(
        message: impl Into<String>,
        line: usize,
        column: usize,
    ) -> Self {
        Self::TypeError {
            message: message.into(),
            line: Some(line),
            column: Some(column),
        }
    }

    pub fn codegen(message: impl Into<String>) -> Self {
        Self::CodeGenError {
            message: message.into(),
        }
    }

    #[allow(dead_code)]
    pub fn internal(message: impl Into<String>) -> Self {
        Self::InternalError {
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ErrorLocation {
    pub line: usize,
    pub column: usize,
    pub _length: usize,
}

impl fmt::Display for ErrorLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "linha {}, coluna {}", self.line, self.column)
    }
}

pub type CompilerResult<T> = Result<T, CompilerError>;

impl From<String> for CompilerError {
    fn from(message: String) -> Self {
        CompilerError::InternalError { message }
    }
} 