// Generated by lib/script-interpreter/ast_generator.rs
// Auto generated file do not edit manually

pub enum VariableName {
    Identifier(crate::token::Token),
    VariableName1{ identifier: crate::token::Token, dollar: crate::token::Token },
    
}

impl VariableName {
    pub fn build_from_identifier(identifier: crate::token::Token) -> Self {
        VariableName::Identifier(identifier)
    }
    pub fn build_from_variable_name1(identifier: crate::token::Token, dollar: crate::token::Token) -> Self {
        VariableName::VariableName1{ identifier, dollar }
    }
}

impl crate::ast::expression::Expression for VariableName {
    fn accept(&self, visitor: Box<dyn crate::ast::visitor::Visitor>) {
        visitor.visit_variable_name(self)
    }

}

