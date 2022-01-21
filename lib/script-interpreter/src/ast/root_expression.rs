// Generated by lib/script-interpreter/generator
// Auto generated file do not edit manually

#[derive(Clone)]
pub enum RootExpression {
    Declaration(Box<crate::ast::declaration::Declaration>),
    
}

impl RootExpression {
    pub fn build_from_declaration(declaration: Box<crate::ast::declaration::Declaration>) -> Self {
        RootExpression::Declaration(declaration)
    }
}

impl crate::ast::expression::Expression for RootExpression {
    fn accept(&self, visitor: Box<dyn crate::ast::visitor::Visitor>) {
        visitor.visit_root_expression(self)
    }

}

