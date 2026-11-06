use std::{ 
    fmt::{Display, Formatter}, 
    hash::Hash, 
    ops::Deref, 
}; 

use function_compiler::{idx, Idx, IdxVec}; 
use printer::ASTPrinter;
use termion::{Fg, Reset}; 
use visitor::ASTVisitor;

use create::{ 
    ast::lexer::Token, 
    compilation_unit::{FunctionIdx, VariableIdx}, 
    text::span::TextSpan, 
    typings::Type,
}; 

pub mod lexer; 
pub mod 