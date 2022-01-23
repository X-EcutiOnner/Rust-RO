// Generated by lib/script-interpreter/generator
// Auto generated file do not edit manually

#[derive(Clone, Debug)]
pub enum Variable {
    Variable0{ scope_specifier: Box<crate::ast::scope_specifier::ScopeSpecifier>, identifier: crate::token::Token },
    Variable1{ scope_specifier: Box<crate::ast::scope_specifier::ScopeSpecifier>, identifier: crate::token::Token, dollar: crate::token::Token },
    
}

impl Variable {
    pub fn build_from_variable0(scope_specifier: Box<crate::ast::scope_specifier::ScopeSpecifier>, identifier: crate::token::Token) -> Self {
        Variable::Variable0{ scope_specifier, identifier }
    }
    pub fn build_from_variable1(scope_specifier: Box<crate::ast::scope_specifier::ScopeSpecifier>, identifier: crate::token::Token, dollar: crate::token::Token) -> Self {
        Variable::Variable1{ scope_specifier, identifier, dollar }
    }
}

impl crate::ast::expression::Expression for Variable {
    fn accept(&self, visitor: Box<dyn crate::ast::visitor::Visitor>) {
        visitor.visit_variable(self)
    }

}

