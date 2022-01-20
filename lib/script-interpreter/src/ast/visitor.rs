// Generated by lib/script-interpreter/generator
// Auto generated file do not edit manually

pub trait Visitor {
  fn visit_root_expression(&self, expression: &dyn crate::ast::expression::Expression);
  fn visit_primary_expression(&self, expression: &dyn crate::ast::expression::Expression);
  fn visit_postfix_expression(&self, expression: &dyn crate::ast::expression::Expression);
  fn visit_assignment_expression(&self, expression: &dyn crate::ast::expression::Expression);
  fn visit_logical_or_expression(&self, expression: &dyn crate::ast::expression::Expression);
  fn visit_logical_and_expression(&self, expression: &dyn crate::ast::expression::Expression);
  fn visit_conditional_expression(&self, expression: &dyn crate::ast::expression::Expression);
  fn visit_assignment_operator(&self, expression: &dyn crate::ast::expression::Expression);
  fn visit_unary_expression(&self, expression: &dyn crate::ast::expression::Expression);
  fn visit_any_expression(&self, expression: &dyn crate::ast::expression::Expression);
  fn visit_declaration(&self, expression: &dyn crate::ast::expression::Expression);
  fn visit_declaration_specifiers(&self, expression: &dyn crate::ast::expression::Expression);
  fn visit_variable(&self, expression: &dyn crate::ast::expression::Expression);
  fn visit_scope_specifier(&self, expression: &dyn crate::ast::expression::Expression);
  fn visit_variable_name(&self, expression: &dyn crate::ast::expression::Expression);
}
