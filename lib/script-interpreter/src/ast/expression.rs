// Generated by lib/script-interpreter/generator
// Auto generated file do not edit manually

pub trait Expression {
    fn accept(&self, visitor: Box<dyn crate::ast::visitor::Visitor>);
}