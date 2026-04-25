//! Effect checking for the LM type system.
//!
//! Verifies that `pure` functions do not call `io` functions, and that
//! functions performing IO are properly annotated.

use crate::env::TypeEnv;
use lm_diagnostics::{Diagnostic, Label};
use lm_parser::ast::*;
use std::collections::HashMap;

/// Check effect annotations for all functions in a program.
///
/// Returns diagnostics for any effect violations found.
pub fn check_effects(program: &Program, env: &TypeEnv) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let aliases = top_level_effect_aliases(program, env);

    for decl in &program.decls {
        if let DeclKind::FnDef {
            name, effect, body, ..
        } = &decl.kind
        {
            if *effect == Effect::Pure {
                check_expr_effects(body, name, env, &aliases, &mut diagnostics);
            }
        }
    }

    diagnostics
}

/// Collect simple top-level aliases such as `let p = print;`.
fn top_level_effect_aliases(program: &Program, env: &TypeEnv) -> HashMap<String, Effect> {
    let mut aliases = HashMap::new();

    for decl in &program.decls {
        if let DeclKind::LetDef { name, value, .. } = &decl.kind {
            if let Some(effect) = effect_of_expr(value, env, &aliases) {
                aliases.insert(name.clone(), effect);
            }
        }
    }

    aliases
}

fn effect_of_name(name: &str, env: &TypeEnv, aliases: &HashMap<String, Effect>) -> Option<Effect> {
    aliases
        .get(name)
        .copied()
        .or_else(|| env.fn_effects.get(name).map(|info| info.effect))
}

fn effect_of_expr(expr: &Expr, env: &TypeEnv, aliases: &HashMap<String, Effect>) -> Option<Effect> {
    if let ExprKind::Ident { name } = &expr.kind {
        effect_of_name(name, env, aliases)
    } else {
        None
    }
}

/// Recursively check an expression for IO calls within a pure function.
fn check_expr_effects(
    expr: &Expr,
    fn_name: &str,
    env: &TypeEnv,
    aliases: &HashMap<String, Effect>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    match &expr.kind {
        ExprKind::FnCall { callee, args } => {
            // Check if the callee is a known IO function
            if let ExprKind::Ident { name } = &callee.kind {
                if effect_of_name(name, env, aliases) == Some(Effect::Io) {
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

            // Check arguments recursively
            check_expr_effects(callee, fn_name, env, aliases, diagnostics);
            for arg in args {
                check_expr_effects(arg, fn_name, env, aliases, diagnostics);
            }
        }

        ExprKind::BinaryOp { lhs, rhs, .. } => {
            check_expr_effects(lhs, fn_name, env, aliases, diagnostics);
            check_expr_effects(rhs, fn_name, env, aliases, diagnostics);
        }

        ExprKind::UnaryOp { operand, .. } => {
            check_expr_effects(operand, fn_name, env, aliases, diagnostics);
        }

        ExprKind::LetExpr {
            name, value, body, ..
        } => {
            check_expr_effects(value, fn_name, env, aliases, diagnostics);
            let mut body_aliases = aliases.clone();
            if let Some(effect) = effect_of_expr(value, env, aliases) {
                body_aliases.insert(name.clone(), effect);
            } else {
                body_aliases.remove(name);
            }
            check_expr_effects(body, fn_name, env, &body_aliases, diagnostics);
        }

        ExprKind::IfElse {
            condition,
            then_branch,
            else_branch,
        } => {
            check_expr_effects(condition, fn_name, env, aliases, diagnostics);
            check_expr_effects(then_branch, fn_name, env, aliases, diagnostics);
            check_expr_effects(else_branch, fn_name, env, aliases, diagnostics);
        }

        ExprKind::Match { scrutinee, arms } => {
            check_expr_effects(scrutinee, fn_name, env, aliases, diagnostics);
            for arm in arms {
                check_expr_effects(&arm.body, fn_name, env, aliases, diagnostics);
            }
        }

        ExprKind::Block { exprs } => {
            for e in exprs {
                check_expr_effects(e, fn_name, env, aliases, diagnostics);
            }
        }

        ExprKind::VariantConstruct { args, .. } => {
            for arg in args {
                check_expr_effects(arg, fn_name, env, aliases, diagnostics);
            }
        }

        ExprKind::ListLiteral { elements } => {
            for elem in elements {
                check_expr_effects(elem, fn_name, env, aliases, diagnostics);
            }
        }

        // These don't perform effects
        ExprKind::Literal { .. } | ExprKind::Ident { .. } | ExprKind::Error => {}
    }
}
