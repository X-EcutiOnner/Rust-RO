// Generated by lib/script-interpreter/generator
// Auto generated file do not edit manually

#[derive(Clone)]
pub enum ScopeSpecifier {
    At(crate::token::Token),
    Dollar(crate::token::Token),
    DollarAt(crate::token::Token),
    Dot(crate::token::Token),
    DotAt(crate::token::Token),
    Quote(crate::token::Token),
    Sharp(crate::token::Token),
    DoubleSharp(crate::token::Token),
    
}

impl ScopeSpecifier {
    pub fn build_from_at(at: crate::token::Token) -> Self {
        ScopeSpecifier::At(at)
    }
    pub fn build_from_dollar(dollar: crate::token::Token) -> Self {
        ScopeSpecifier::Dollar(dollar)
    }
    pub fn build_from_dollar_at(dollar_at: crate::token::Token) -> Self {
        ScopeSpecifier::DollarAt(dollar_at)
    }
    pub fn build_from_dot(dot: crate::token::Token) -> Self {
        ScopeSpecifier::Dot(dot)
    }
    pub fn build_from_dot_at(dot_at: crate::token::Token) -> Self {
        ScopeSpecifier::DotAt(dot_at)
    }
    pub fn build_from_quote(quote: crate::token::Token) -> Self {
        ScopeSpecifier::Quote(quote)
    }
    pub fn build_from_sharp(sharp: crate::token::Token) -> Self {
        ScopeSpecifier::Sharp(sharp)
    }
    pub fn build_from_double_sharp(double_sharp: crate::token::Token) -> Self {
        ScopeSpecifier::DoubleSharp(double_sharp)
    }
}

impl crate::ast::expression::Expression for ScopeSpecifier {
    fn accept(&self, visitor: Box<dyn crate::ast::visitor::Visitor>) {
        visitor.visit_scope_specifier(self)
    }

}

