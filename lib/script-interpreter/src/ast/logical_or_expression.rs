// Generated by lib/script-interpreter/generator
// Auto generated file do not edit manually

#[derive(Clone, Debug)]
pub enum LogicalOrExpression {
    LogicalAndExpression(Box<crate::ast::logical_and_expression::LogicalAndExpression>),
    LogicalOrExpression1{ logical_or_expression: Box<crate::ast::logical_or_expression::LogicalOrExpression>, or_op: crate::token::Token, logical_and_expression: Box<crate::ast::logical_and_expression::LogicalAndExpression> },
    
}

impl LogicalOrExpression {
    pub fn build_from_logical_and_expression(logical_and_expression: Box<crate::ast::logical_and_expression::LogicalAndExpression>) -> Self {
        LogicalOrExpression::LogicalAndExpression(logical_and_expression)
    }
    pub fn build_from_logical_or_expression1(logical_or_expression: Box<crate::ast::logical_or_expression::LogicalOrExpression>, or_op: crate::token::Token, logical_and_expression: Box<crate::ast::logical_and_expression::LogicalAndExpression>) -> Self {
        LogicalOrExpression::LogicalOrExpression1{ logical_or_expression, or_op, logical_and_expression }
    }
}

impl crate::ast::expression::Expression for LogicalOrExpression {
    fn accept(&self, visitor: Box<dyn crate::ast::visitor::Visitor>) {
        visitor.visit_logical_or_expression(self)
    }

}

