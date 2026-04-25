//! Effect checking for the LM type system.
//!
//! Verifies that `pure` functions do not call `io` functions, and that
//! functions performing IO are properly annotated.

use crate::env::TypeEnv;
use lm_diagnostics::{Diagnostic, Label};
use lm_parser::ast::*;

/// Check effect annotations for all functions in a program.
///
/// Returns diagnostics for any effect violations found.
pub fn check_effects(program: &Program, env: &TypeEnv) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    for decl in &program.decls {
        if let DeclKind::FnDef {
            name,
            effect,
            body,
            ..
        } = &decl.kind
        {
            if *effect == Effect::Pure {
                check_expr_effects(body, name, env, &mut diagnostics);
            }
        }
    }

    diagnostics
}

/// Recursively check an expression for IO calls within a pure function.
fn check_expr_effects(
    expr: &Expr,
    fn_name: &str,
    env: &TypeEnv,
    diagnostics: &mut Vec<Diagnostic>,
) {
    match &expr.kind {
        ExprKind::FnCall { callee, args } => {
            // Check if the callee is a known IO function
            if let ExprKind::Ident { name } = &callee.kind {
                if let Some(info) = env.fn_effects.get(name.as_str()) {
                    if info.effect == Effect::Io {
                        diagnostics.push(
                            Diagnostic::error(
                                "E0300",
                                format!(
                                    "pure function `{}` cannot call io function `{}`",
                                    fn_name, name
                                ),
                                expr.span,
                            )
                            .with_label(Label::new(
                                callee.span,
                                format!("`{}` is an io function", name),
                            ))
                            .with_help(format!(
                                "either mark `{}` as `io fn` or remove the call to `{}`",
                                fn_name, name
                            )),
                        );
                    }
                }
            }

            // Check arguments recursively
            check_expr_effects(callee, fn_name, env, diagnostics);
            for arg in args {
                check_expr_effects(arg, fn_name, env, diagnostics);
            }
        }

        ExprKind::BinaryOp { lhs, rhs, .. } => {
            check_expr_effects(lhs, fn_name, env, diagnostics);
            check_expr_effects(rhs, fn_name, env, diagnostics);
        }

        ExprKind::UnaryOp { operand, .. } => {
            check_expr_effects(operand, fn_name, env, diagnostics);
        }

        ExprKind::LetExpr { value, body, .. } => {
            check_expr_effects(value, fn_name, env, diagnostics);
            check_expr_effects(body, fn_name, env, diagnostics);
        }

        ExprKind::IfElse {
            condition,
            then_branch,
            else_branch,
        } => {
            check_expr_effects(condition, fn_name, env, diagnostics);
            check_expr_effects(then_branch, fn_name, env, diagnostics);
            check_expr_effects(else_branch, fn_name, env, diagnostics);
        }

        ExprKind::Match { scrutinee, arms } => {
            check_expr_effects(scrutinee, fn_name, env, diagnostics);
            for arm in arms {
                check_expr_effects(&arm.body, fn_name, env, diagnostics);
            }
        }

        ExprKind::Block { exprs } => {
            for e in exprs {
                check_expr_effects(e, fn_name, env, diagnostics);
            }
        }

        ExprKind::VariantConstruct { args, .. } => {
            for arg in args {
                check_expr_effects(arg, fn_name, env, diagnostics);
            }
        }

        ExprKind::ListLiteral { elements } => {
            for elem in elements {
                check_expr_effects(elem, fn_name, env, diagnostics);
            }
        }

        // These don't perform effects
        ExprKind::Literal { .. } | ExprKind::Ident { .. } | ExprKind::Error => {}
    }
}
