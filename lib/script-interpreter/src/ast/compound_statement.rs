// Generated by lib/script-interpreter/generator
// Auto generated file do not edit manually

#[derive(Clone, Debug)]
pub enum CompoundStatement {
    CompoundStatement0{ left_bracket: crate::token::Token, statement_list: Box<crate::ast::statement_list::StatementList>, right_bracket: crate::token::Token },
    
}

impl CompoundStatement {
    pub fn build_from_compound_statement0(left_bracket: crate::token::Token, statement_list: Box<crate::ast::statement_list::StatementList>, right_bracket: crate::token::Token) -> Self {
        CompoundStatement::CompoundStatement0{ left_bracket, statement_list, right_bracket }
    }
}

impl crate::ast::expression::Expression for CompoundStatement {
    fn accept(&self, visitor: Box<dyn crate::ast::visitor::Visitor>) {
        visitor.visit_compound_statement(self)
    }

}

