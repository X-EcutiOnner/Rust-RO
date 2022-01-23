// Generated by lib/script-interpreter/generator
// Auto generated file do not edit manually

#[derive(Clone, Debug)]
pub enum PostfixExpression {
    PrimaryExpression(Box<crate::ast::primary_expression::PrimaryExpression>),
    PostfixExpression1{ postfix_expression: Box<crate::ast::postfix_expression::PostfixExpression>, left_bracket: crate::token::Token, any_expression: Box<crate::ast::any_expression::AnyExpression>, right_bracket: crate::token::Token },
    PostfixExpression2{ postfix_expression: Box<crate::ast::postfix_expression::PostfixExpression>, left_paren: crate::token::Token, right_paren: crate::token::Token },
    PostfixExpression3{ postfix_expression: Box<crate::ast::postfix_expression::PostfixExpression>, dot: crate::token::Token, identifier: crate::token::Token },
    PostfixExpression4{ postfix_expression: Box<crate::ast::postfix_expression::PostfixExpression>, increment_op: crate::token::Token },
    PostfixExpression5{ postfix_expression: Box<crate::ast::postfix_expression::PostfixExpression>, decrement_op: crate::token::Token },
    
}

impl PostfixExpression {
    pub fn build_from_primary_expression(primary_expression: Box<crate::ast::primary_expression::PrimaryExpression>) -> Self {
        PostfixExpression::PrimaryExpression(primary_expression)
    }
    pub fn build_from_postfix_expression1(postfix_expression: Box<crate::ast::postfix_expression::PostfixExpression>, left_bracket: crate::token::Token, any_expression: Box<crate::ast::any_expression::AnyExpression>, right_bracket: crate::token::Token) -> Self {
        PostfixExpression::PostfixExpression1{ postfix_expression, left_bracket, any_expression, right_bracket }
    }
    pub fn build_from_postfix_expression2(postfix_expression: Box<crate::ast::postfix_expression::PostfixExpression>, left_paren: crate::token::Token, right_paren: crate::token::Token) -> Self {
        PostfixExpression::PostfixExpression2{ postfix_expression, left_paren, right_paren }
    }
    pub fn build_from_postfix_expression3(postfix_expression: Box<crate::ast::postfix_expression::PostfixExpression>, dot: crate::token::Token, identifier: crate::token::Token) -> Self {
        PostfixExpression::PostfixExpression3{ postfix_expression, dot, identifier }
    }
    pub fn build_from_postfix_expression4(postfix_expression: Box<crate::ast::postfix_expression::PostfixExpression>, increment_op: crate::token::Token) -> Self {
        PostfixExpression::PostfixExpression4{ postfix_expression, increment_op }
    }
    pub fn build_from_postfix_expression5(postfix_expression: Box<crate::ast::postfix_expression::PostfixExpression>, decrement_op: crate::token::Token) -> Self {
        PostfixExpression::PostfixExpression5{ postfix_expression, decrement_op }
    }
}

impl crate::ast::expression::Expression for PostfixExpression {
    fn accept(&self, visitor: Box<dyn crate::ast::visitor::Visitor>) {
        visitor.visit_postfix_expression(self)
    }

}

