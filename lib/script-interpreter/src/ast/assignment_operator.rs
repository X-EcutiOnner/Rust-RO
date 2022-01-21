// Generated by lib/script-interpreter/generator
// Auto generated file do not edit manually

#[derive(Clone)]
pub enum AssignmentOperator {
    Equal(crate::token::Token),
    
}

impl AssignmentOperator {
    pub fn build_from_equal(equal: crate::token::Token) -> Self {
        AssignmentOperator::Equal(equal)
    }
}

impl crate::ast::expression::Expression for AssignmentOperator {
    fn accept(&self, visitor: Box<dyn crate::ast::visitor::Visitor>) {
        visitor.visit_assignment_operator(self)
    }

}

