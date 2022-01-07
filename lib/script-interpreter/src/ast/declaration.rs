// Generated by lib/script-interpreter/ast_generator.rs
// Auto generated file do not edit manually

pub enum Declaration {
    Declaration0{ declaration_specifiers: Box<crate::ast::declaration_specifiers::DeclarationSpecifiers>, semi_colon: crate::token::Token },
    
}

impl Declaration {
    pub fn build_from_declaration0(declaration_specifiers: Box<crate::ast::declaration_specifiers::DeclarationSpecifiers>, semi_colon: crate::token::Token) -> Self {
        Declaration::Declaration0{ declaration_specifiers, semi_colon }
    }
}

impl crate::ast::expression::Expression for Declaration {
    fn accept(&self, visitor: Box<dyn crate::ast::visitor::Visitor>) {
        visitor.visit_declaration(self)
    }

}

