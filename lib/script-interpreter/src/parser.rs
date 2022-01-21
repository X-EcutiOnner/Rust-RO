// Generated by lib/script-interpreter/generator
// Auto generated file do not edit manually

use crate::token::*;
use crate::ast_node::AstNode;
use crate::parser_state::ParserState;

use crate::ast::expression::*;

use crate::ast::root_expression::*;
use crate::ast::primary_expression::*;
use crate::ast::postfix_expression::*;
use crate::ast::assignment_expression::*;
use crate::ast::logical_or_expression::*;
use crate::ast::logical_and_expression::*;
use crate::ast::conditional_expression::*;
use crate::ast::assignment_operator::*;
use crate::ast::unary_expression::*;
use crate::ast::any_expression::*;
use crate::ast::declaration::*;
use crate::ast::declaration_specifiers::*;
use crate::ast::variable::*;
use crate::ast::scope_specifier::*;
use crate::ast::variable_name::*;

pub fn parse(tokens: &'static Vec<Token>) {
    let mut parser_state = ParserState::new(tokens);
}
pub fn parse_token(parser_state: &mut ParserState, token_type: TokenType) -> Result<Token, String> {
    parser_state.consume(token_type)
}

// <declaration>
pub fn parse_root_expression(parser_state: &mut ParserState) -> Result<AstNode<RootExpression>, String> {
    let mut result: Result<AstNode<RootExpression>, String>;
    result = Err("Haven't match (todo)".to_string());
    if result.is_err() {
        result = (|| {
        let children_node_0_result = parse_declaration(parser_state);
        let children_node_0 = children_node_0_result?;

        let expression = RootExpression::build_from_declaration(Box::new(*children_node_0.expression().clone()));
            let mut node = AstNode::new(Box::new(expression));
            node.append_child(children_node_0.as_any());
        return Ok(node);
        })();
    }
    Err("Haven't match (todo)".to_string())
}
// "Identifier" | "Number" | "String" | "LeftParen" <any_expression> "RightParen"
pub fn parse_primary_expression(parser_state: &mut ParserState) -> Result<AstNode<PrimaryExpression>, String> {
    let mut result: Result<AstNode<PrimaryExpression>, String>;
    result = Err("Haven't match (todo)".to_string());
    if result.is_err() {
        result = (|| {
        let token_parse_0_result = parse_token(parser_state, TokenType::Identifier(Default::default()));
        let token_parse_0 = token_parse_0_result?;

        let expression = PrimaryExpression::build_from_identifier(token_parse_0);
        return Ok(AstNode::new(Box::new(expression)));
        })();
    }
    if result.is_err() {
        result = (|| {
        let token_parse_0_result = parse_token(parser_state, TokenType::Number(Default::default()));
        let token_parse_0 = token_parse_0_result?;

        let expression = PrimaryExpression::build_from_number(token_parse_0);
        return Ok(AstNode::new(Box::new(expression)));
        })();
    }
    if result.is_err() {
        result = (|| {
        let token_parse_0_result = parse_token(parser_state, TokenType::String(Default::default()));
        let token_parse_0 = token_parse_0_result?;

        let expression = PrimaryExpression::build_from_string(token_parse_0);
        return Ok(AstNode::new(Box::new(expression)));
        })();
    }
    if result.is_err() {
        result = (|| {
        let token_parse_0_result = parse_token(parser_state, TokenType::LeftParen);
        let token_parse_0 = token_parse_0_result?;

        let children_node_1_result = parse_any_expression(parser_state);
        let children_node_1 = children_node_1_result?;

        let token_parse_2_result = parse_token(parser_state, TokenType::RightParen);
        let token_parse_2 = token_parse_2_result?;

        let expression = PrimaryExpression::build_from_primary_expression3(token_parse_0, Box::new(*children_node_1.expression().clone()), token_parse_2);
        return Ok(AstNode::new(Box::new(expression)));
        })();
    }
    Err("Haven't match (todo)".to_string())
}
// <primary_expression> | <postfix_expression> "LeftBracket" <any_expression> "RightBracket" | <postfix_expression> "LeftParen" "RightParen" | <postfix_expression> "Dot" "Identifier" | <postfix_expression> "IncrementOp" | <postfix_expression> "DecrementOp"
pub fn parse_postfix_expression(parser_state: &mut ParserState) -> Result<AstNode<PostfixExpression>, String> {
    let mut result: Result<AstNode<PostfixExpression>, String>;
    result = Err("Haven't match (todo)".to_string());
    if result.is_err() {
        result = (|| {
        let children_node_0_result = parse_primary_expression(parser_state);
        let children_node_0 = children_node_0_result?;

        let expression = PostfixExpression::build_from_primary_expression(Box::new(*children_node_0.expression().clone()));
        return Ok(AstNode::new(Box::new(expression)));
        })();
    }
    if result.is_err() {
        result = (|| {
        let children_node_0_result = parse_postfix_expression(parser_state);
        let children_node_0 = children_node_0_result?;

        let token_parse_1_result = parse_token(parser_state, TokenType::LeftBracket);
        let token_parse_1 = token_parse_1_result?;

        let children_node_2_result = parse_any_expression(parser_state);
        let children_node_2 = children_node_2_result?;

        let token_parse_3_result = parse_token(parser_state, TokenType::RightBracket);
        let token_parse_3 = token_parse_3_result?;

        let expression = PostfixExpression::build_from_postfix_expression1(Box::new(*children_node_0.expression().clone()), token_parse_1, Box::new(*children_node_2.expression().clone()), token_parse_3);
        return Ok(AstNode::new(Box::new(expression)));
        })();
    }
    if result.is_err() {
        result = (|| {
        let children_node_0_result = parse_postfix_expression(parser_state);
        let children_node_0 = children_node_0_result?;

        let token_parse_1_result = parse_token(parser_state, TokenType::LeftParen);
        let token_parse_1 = token_parse_1_result?;

        let token_parse_2_result = parse_token(parser_state, TokenType::RightParen);
        let token_parse_2 = token_parse_2_result?;

        let expression = PostfixExpression::build_from_postfix_expression2(Box::new(*children_node_0.expression().clone()), token_parse_1, token_parse_2);
        return Ok(AstNode::new(Box::new(expression)));
        })();
    }
    if result.is_err() {
        result = (|| {
        let children_node_0_result = parse_postfix_expression(parser_state);
        let children_node_0 = children_node_0_result?;

        let token_parse_1_result = parse_token(parser_state, TokenType::Dot);
        let token_parse_1 = token_parse_1_result?;

        let token_parse_2_result = parse_token(parser_state, TokenType::Identifier(Default::default()));
        let token_parse_2 = token_parse_2_result?;

        let expression = PostfixExpression::build_from_postfix_expression3(Box::new(*children_node_0.expression().clone()), token_parse_1, token_parse_2);
        return Ok(AstNode::new(Box::new(expression)));
        })();
    }
    if result.is_err() {
        result = (|| {
        let children_node_0_result = parse_postfix_expression(parser_state);
        let children_node_0 = children_node_0_result?;

        let token_parse_1_result = parse_token(parser_state, TokenType::IncrementOp);
        let token_parse_1 = token_parse_1_result?;

        let expression = PostfixExpression::build_from_postfix_expression4(Box::new(*children_node_0.expression().clone()), token_parse_1);
        return Ok(AstNode::new(Box::new(expression)));
        })();
    }
    if result.is_err() {
        result = (|| {
        let children_node_0_result = parse_postfix_expression(parser_state);
        let children_node_0 = children_node_0_result?;

        let token_parse_1_result = parse_token(parser_state, TokenType::DecrementOp);
        let token_parse_1 = token_parse_1_result?;

        let expression = PostfixExpression::build_from_postfix_expression5(Box::new(*children_node_0.expression().clone()), token_parse_1);
        return Ok(AstNode::new(Box::new(expression)));
        })();
    }
    Err("Haven't match (todo)".to_string())
}
// <conditional_expression> | <unary_expression> <assignment_operator> <assignment_expression>
pub fn parse_assignment_expression(parser_state: &mut ParserState) -> Result<AstNode<AssignmentExpression>, String> {
    let mut result: Result<AstNode<AssignmentExpression>, String>;
    result = Err("Haven't match (todo)".to_string());
    if result.is_err() {
        result = (|| {
        let children_node_0_result = parse_conditional_expression(parser_state);
        let children_node_0 = children_node_0_result?;

        let expression = AssignmentExpression::build_from_conditional_expression(Box::new(*children_node_0.expression().clone()));
        return Ok(AstNode::new(Box::new(expression)));
        })();
    }
    if result.is_err() {
        result = (|| {
        let children_node_0_result = parse_unary_expression(parser_state);
        let children_node_0 = children_node_0_result?;

        let children_node_1_result = parse_assignment_operator(parser_state);
        let children_node_1 = children_node_1_result?;

        let children_node_2_result = parse_assignment_expression(parser_state);
        let children_node_2 = children_node_2_result?;

        let expression = AssignmentExpression::build_from_assignment_expression1(Box::new(*children_node_0.expression().clone()), Box::new(*children_node_1.expression().clone()), Box::new(*children_node_2.expression().clone()));
        return Ok(AstNode::new(Box::new(expression)));
        })();
    }
    Err("Haven't match (todo)".to_string())
}
// <logical_and_expression> | <logical_or_expression> "OrOp" <logical_and_expression>
pub fn parse_logical_or_expression(parser_state: &mut ParserState) -> Result<AstNode<LogicalOrExpression>, String> {
    let mut result: Result<AstNode<LogicalOrExpression>, String>;
    result = Err("Haven't match (todo)".to_string());
    if result.is_err() {
        result = (|| {
        let children_node_0_result = parse_logical_and_expression(parser_state);
        let children_node_0 = children_node_0_result?;

        let expression = LogicalOrExpression::build_from_logical_and_expression(Box::new(*children_node_0.expression().clone()));
        return Ok(AstNode::new(Box::new(expression)));
        })();
    }
    if result.is_err() {
        result = (|| {
        let children_node_0_result = parse_logical_or_expression(parser_state);
        let children_node_0 = children_node_0_result?;

        let token_parse_1_result = parse_token(parser_state, TokenType::OrOp);
        let token_parse_1 = token_parse_1_result?;

        let children_node_2_result = parse_logical_and_expression(parser_state);
        let children_node_2 = children_node_2_result?;

        let expression = LogicalOrExpression::build_from_logical_or_expression1(Box::new(*children_node_0.expression().clone()), token_parse_1, Box::new(*children_node_2.expression().clone()));
        return Ok(AstNode::new(Box::new(expression)));
        })();
    }
    Err("Haven't match (todo)".to_string())
}
// "Equal"
pub fn parse_logical_and_expression(parser_state: &mut ParserState) -> Result<AstNode<LogicalAndExpression>, String> {
    let mut result: Result<AstNode<LogicalAndExpression>, String>;
    result = Err("Haven't match (todo)".to_string());
    if result.is_err() {
        result = (|| {
        let token_parse_0_result = parse_token(parser_state, TokenType::Equal);
        let token_parse_0 = token_parse_0_result?;

        let expression = LogicalAndExpression::build_from_equal(token_parse_0);
        return Ok(AstNode::new(Box::new(expression)));
        })();
    }
    Err("Haven't match (todo)".to_string())
}
// <logical_or_expression> | <logical_or_expression> "QuestionMark" <any_expression> "Colon" <conditional_expression>
pub fn parse_conditional_expression(parser_state: &mut ParserState) -> Result<AstNode<ConditionalExpression>, String> {
    let mut result: Result<AstNode<ConditionalExpression>, String>;
    result = Err("Haven't match (todo)".to_string());
    if result.is_err() {
        result = (|| {
        let children_node_0_result = parse_logical_or_expression(parser_state);
        let children_node_0 = children_node_0_result?;

        let expression = ConditionalExpression::build_from_logical_or_expression(Box::new(*children_node_0.expression().clone()));
        return Ok(AstNode::new(Box::new(expression)));
        })();
    }
    if result.is_err() {
        result = (|| {
        let children_node_0_result = parse_logical_or_expression(parser_state);
        let children_node_0 = children_node_0_result?;

        let token_parse_1_result = parse_token(parser_state, TokenType::QuestionMark);
        let token_parse_1 = token_parse_1_result?;

        let children_node_2_result = parse_any_expression(parser_state);
        let children_node_2 = children_node_2_result?;

        let token_parse_3_result = parse_token(parser_state, TokenType::Colon);
        let token_parse_3 = token_parse_3_result?;

        let children_node_4_result = parse_conditional_expression(parser_state);
        let children_node_4 = children_node_4_result?;

        let expression = ConditionalExpression::build_from_conditional_expression1(Box::new(*children_node_0.expression().clone()), token_parse_1, Box::new(*children_node_2.expression().clone()), token_parse_3, Box::new(*children_node_4.expression().clone()));
        return Ok(AstNode::new(Box::new(expression)));
        })();
    }
    Err("Haven't match (todo)".to_string())
}
// "Equal"
pub fn parse_assignment_operator(parser_state: &mut ParserState) -> Result<AstNode<AssignmentOperator>, String> {
    let mut result: Result<AstNode<AssignmentOperator>, String>;
    result = Err("Haven't match (todo)".to_string());
    if result.is_err() {
        result = (|| {
        let token_parse_0_result = parse_token(parser_state, TokenType::Equal);
        let token_parse_0 = token_parse_0_result?;

        let expression = AssignmentOperator::build_from_equal(token_parse_0);
        return Ok(AstNode::new(Box::new(expression)));
        })();
    }
    Err("Haven't match (todo)".to_string())
}
// <postfix_expression> | "IncrementOp" <unary_expression> | "DecrementOp" <unary_expression>
pub fn parse_unary_expression(parser_state: &mut ParserState) -> Result<AstNode<UnaryExpression>, String> {
    let mut result: Result<AstNode<UnaryExpression>, String>;
    result = Err("Haven't match (todo)".to_string());
    if result.is_err() {
        result = (|| {
        let children_node_0_result = parse_postfix_expression(parser_state);
        let children_node_0 = children_node_0_result?;

        let expression = UnaryExpression::build_from_postfix_expression(Box::new(*children_node_0.expression().clone()));
        return Ok(AstNode::new(Box::new(expression)));
        })();
    }
    if result.is_err() {
        result = (|| {
        let token_parse_0_result = parse_token(parser_state, TokenType::IncrementOp);
        let token_parse_0 = token_parse_0_result?;

        let children_node_1_result = parse_unary_expression(parser_state);
        let children_node_1 = children_node_1_result?;

        let expression = UnaryExpression::build_from_unary_expression1(token_parse_0, Box::new(*children_node_1.expression().clone()));
        return Ok(AstNode::new(Box::new(expression)));
        })();
    }
    if result.is_err() {
        result = (|| {
        let token_parse_0_result = parse_token(parser_state, TokenType::DecrementOp);
        let token_parse_0 = token_parse_0_result?;

        let children_node_1_result = parse_unary_expression(parser_state);
        let children_node_1 = children_node_1_result?;

        let expression = UnaryExpression::build_from_unary_expression2(token_parse_0, Box::new(*children_node_1.expression().clone()));
        return Ok(AstNode::new(Box::new(expression)));
        })();
    }
    Err("Haven't match (todo)".to_string())
}
// <assignment_expression>
pub fn parse_any_expression(parser_state: &mut ParserState) -> Result<AstNode<AnyExpression>, String> {
    let mut result: Result<AstNode<AnyExpression>, String>;
    result = Err("Haven't match (todo)".to_string());
    if result.is_err() {
        result = (|| {
        let children_node_0_result = parse_assignment_expression(parser_state);
        let children_node_0 = children_node_0_result?;

        let expression = AnyExpression::build_from_assignment_expression(Box::new(*children_node_0.expression().clone()));
        return Ok(AstNode::new(Box::new(expression)));
        })();
    }
    Err("Haven't match (todo)".to_string())
}
// <declaration_specifiers> "SemiColon"
pub fn parse_declaration(parser_state: &mut ParserState) -> Result<AstNode<Declaration>, String> {
    let mut result: Result<AstNode<Declaration>, String>;
    result = Err("Haven't match (todo)".to_string());
    if result.is_err() {
        result = (|| {
        let children_node_0_result = parse_declaration_specifiers(parser_state);
        let children_node_0 = children_node_0_result?;

        let token_parse_1_result = parse_token(parser_state, TokenType::SemiColon);
        let token_parse_1 = token_parse_1_result?;

        let expression = Declaration::build_from_declaration0(Box::new(*children_node_0.expression().clone()), token_parse_1);
        return Ok(AstNode::new(Box::new(expression)));
        })();
    }
    Err("Haven't match (todo)".to_string())
}
// <variable> | <variable> "Equal" <assignment_expression>
pub fn parse_declaration_specifiers(parser_state: &mut ParserState) -> Result<AstNode<DeclarationSpecifiers>, String> {
    let mut result: Result<AstNode<DeclarationSpecifiers>, String>;
    result = Err("Haven't match (todo)".to_string());
    if result.is_err() {
        result = (|| {
        let children_node_0_result = parse_variable(parser_state);
        let children_node_0 = children_node_0_result?;

        let expression = DeclarationSpecifiers::build_from_variable(Box::new(*children_node_0.expression().clone()));
        return Ok(AstNode::new(Box::new(expression)));
        })();
    }
    if result.is_err() {
        result = (|| {
        let children_node_0_result = parse_variable(parser_state);
        let children_node_0 = children_node_0_result?;

        let token_parse_1_result = parse_token(parser_state, TokenType::Equal);
        let token_parse_1 = token_parse_1_result?;

        let children_node_2_result = parse_assignment_expression(parser_state);
        let children_node_2 = children_node_2_result?;

        let expression = DeclarationSpecifiers::build_from_declaration_specifiers1(Box::new(*children_node_0.expression().clone()), token_parse_1, Box::new(*children_node_2.expression().clone()));
        return Ok(AstNode::new(Box::new(expression)));
        })();
    }
    Err("Haven't match (todo)".to_string())
}
// <scope_specifier> "Identifier" | <scope_specifier> "Identifier" "Dollar"
pub fn parse_variable(parser_state: &mut ParserState) -> Result<AstNode<Variable>, String> {
    let mut result: Result<AstNode<Variable>, String>;
    result = Err("Haven't match (todo)".to_string());
    if result.is_err() {
        result = (|| {
        let children_node_0_result = parse_scope_specifier(parser_state);
        let children_node_0 = children_node_0_result?;

        let token_parse_1_result = parse_token(parser_state, TokenType::Identifier(Default::default()));
        let token_parse_1 = token_parse_1_result?;

        let expression = Variable::build_from_variable0(Box::new(*children_node_0.expression().clone()), token_parse_1);
        return Ok(AstNode::new(Box::new(expression)));
        })();
    }
    if result.is_err() {
        result = (|| {
        let children_node_0_result = parse_scope_specifier(parser_state);
        let children_node_0 = children_node_0_result?;

        let token_parse_1_result = parse_token(parser_state, TokenType::Identifier(Default::default()));
        let token_parse_1 = token_parse_1_result?;

        let token_parse_2_result = parse_token(parser_state, TokenType::Dollar);
        let token_parse_2 = token_parse_2_result?;

        let expression = Variable::build_from_variable1(Box::new(*children_node_0.expression().clone()), token_parse_1, token_parse_2);
        return Ok(AstNode::new(Box::new(expression)));
        })();
    }
    Err("Haven't match (todo)".to_string())
}
// "At" | "Dollar" | "DollarAt" | "Dot" | "DotAt" | "Quote" | "Sharp" | "DoubleSharp"
pub fn parse_scope_specifier(parser_state: &mut ParserState) -> Result<AstNode<ScopeSpecifier>, String> {
    let mut result: Result<AstNode<ScopeSpecifier>, String>;
    result = Err("Haven't match (todo)".to_string());
    if result.is_err() {
        result = (|| {
        let token_parse_0_result = parse_token(parser_state, TokenType::At);
        let token_parse_0 = token_parse_0_result?;

        let expression = ScopeSpecifier::build_from_at(token_parse_0);
        return Ok(AstNode::new(Box::new(expression)));
        })();
    }
    if result.is_err() {
        result = (|| {
        let token_parse_0_result = parse_token(parser_state, TokenType::Dollar);
        let token_parse_0 = token_parse_0_result?;

        let expression = ScopeSpecifier::build_from_dollar(token_parse_0);
        return Ok(AstNode::new(Box::new(expression)));
        })();
    }
    if result.is_err() {
        result = (|| {
        let token_parse_0_result = parse_token(parser_state, TokenType::DollarAt);
        let token_parse_0 = token_parse_0_result?;

        let expression = ScopeSpecifier::build_from_dollar_at(token_parse_0);
        return Ok(AstNode::new(Box::new(expression)));
        })();
    }
    if result.is_err() {
        result = (|| {
        let token_parse_0_result = parse_token(parser_state, TokenType::Dot);
        let token_parse_0 = token_parse_0_result?;

        let expression = ScopeSpecifier::build_from_dot(token_parse_0);
        return Ok(AstNode::new(Box::new(expression)));
        })();
    }
    if result.is_err() {
        result = (|| {
        let token_parse_0_result = parse_token(parser_state, TokenType::DotAt);
        let token_parse_0 = token_parse_0_result?;

        let expression = ScopeSpecifier::build_from_dot_at(token_parse_0);
        return Ok(AstNode::new(Box::new(expression)));
        })();
    }
    if result.is_err() {
        result = (|| {
        let token_parse_0_result = parse_token(parser_state, TokenType::Quote);
        let token_parse_0 = token_parse_0_result?;

        let expression = ScopeSpecifier::build_from_quote(token_parse_0);
        return Ok(AstNode::new(Box::new(expression)));
        })();
    }
    if result.is_err() {
        result = (|| {
        let token_parse_0_result = parse_token(parser_state, TokenType::Sharp);
        let token_parse_0 = token_parse_0_result?;

        let expression = ScopeSpecifier::build_from_sharp(token_parse_0);
        return Ok(AstNode::new(Box::new(expression)));
        })();
    }
    if result.is_err() {
        result = (|| {
        let token_parse_0_result = parse_token(parser_state, TokenType::DoubleSharp);
        let token_parse_0 = token_parse_0_result?;

        let expression = ScopeSpecifier::build_from_double_sharp(token_parse_0);
        return Ok(AstNode::new(Box::new(expression)));
        })();
    }
    Err("Haven't match (todo)".to_string())
}
// "Identifier" | "Identifier" "Dollar"
pub fn parse_variable_name(parser_state: &mut ParserState) -> Result<AstNode<VariableName>, String> {
    let mut result: Result<AstNode<VariableName>, String>;
    result = Err("Haven't match (todo)".to_string());
    if result.is_err() {
        result = (|| {
        let token_parse_0_result = parse_token(parser_state, TokenType::Identifier(Default::default()));
        let token_parse_0 = token_parse_0_result?;

        let expression = VariableName::build_from_identifier(token_parse_0);
        return Ok(AstNode::new(Box::new(expression)));
        })();
    }
    if result.is_err() {
        result = (|| {
        let token_parse_0_result = parse_token(parser_state, TokenType::Identifier(Default::default()));
        let token_parse_0 = token_parse_0_result?;

        let token_parse_1_result = parse_token(parser_state, TokenType::Dollar);
        let token_parse_1 = token_parse_1_result?;

        let expression = VariableName::build_from_variable_name1(token_parse_0, token_parse_1);
        return Ok(AstNode::new(Box::new(expression)));
        })();
    }
    Err("Haven't match (todo)".to_string())
}
