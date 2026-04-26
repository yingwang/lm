//! Pattern match exhaustiveness checking.
//!
//! Verifies that match expressions cover all possible cases, and warns
//! about unreachable patterns after wildcards.

use crate::env::TypeEnv;
use crate::unify::UnificationTable;
use lm_diagnostics::{Diagnostic, Label};
use lm_parser::ast::*;

/// Check all match expressions in a program for exhaustiveness.
///
/// Returns diagnostics for non-exhaustive matches and unreachable patterns.
pub fn check_exhaustiveness(
    program: &Program,
    env: &TypeEnv,
    table: &UnificationTable,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    for decl in &program.decls {
        match &decl.kind {
            DeclKind::FnDef { body, .. } => {
                check_expr_exhaustiveness(body, env, table, &mut diagnostics);
            }
            DeclKind::LetDef { value, .. } => {
                check_expr_exhaustiveness(value, env, table, &mut diagnostics);
            }
            DeclKind::TypeDef { .. } => {}
        }
    }

    diagnostics
}

/// Recursively check expressions for match exhaustiveness.
fn check_expr_exhaustiveness(
    expr: &Expr,
    env: &TypeEnv,
    table: &UnificationTable,
    diagnostics: &mut Vec<Diagnostic>,
) {
    match &expr.kind {
        ExprKind::Match { scrutinee, arms } => {
            // Recursively check sub-expressions
            check_expr_exhaustiveness(scrutinee, env, table, diagnostics);
            for arm in arms {
                check_expr_exhaustiveness(&arm.body, env, table, diagnostics);
            }

            // Check exhaustiveness of the match itself
            check_match_exhaustiveness(scrutinee, arms, env, table, diagnostics);
        }

        ExprKind::BinaryOp { lhs, rhs, .. } => {
            check_expr_exhaustiveness(lhs, env, table, diagnostics);
            check_expr_exhaustiveness(rhs, env, table, diagnostics);
        }

        ExprKind::UnaryOp { operand, .. } => {
            check_expr_exhaustiveness(operand, env, table, diagnostics);
        }

        ExprKind::FnCall { callee, args } => {
            check_expr_exhaustiveness(callee, env, table, diagnostics);
            for arg in args {
                check_expr_exhaustiveness(arg, env, table, diagnostics);
            }
        }

        ExprKind::LetExpr { value, body, .. } => {
            check_expr_exhaustiveness(value, env, table, diagnostics);
            check_expr_exhaustiveness(body, env, table, diagnostics);
        }

        ExprKind::IfElse {
            condition,
            then_branch,
            else_branch,
        } => {
            check_expr_exhaustiveness(condition, env, table, diagnostics);
            check_expr_exhaustiveness(then_branch, env, table, diagnostics);
            check_expr_exhaustiveness(else_branch, env, table, diagnostics);
        }

        ExprKind::Block { exprs } => {
            for e in exprs {
                check_expr_exhaustiveness(e, env, table, diagnostics);
            }
        }

        ExprKind::VariantConstruct { args, .. } => {
            for arg in args {
                check_expr_exhaustiveness(arg, env, table, diagnostics);
            }
        }

        ExprKind::ListLiteral { elements } => {
            for elem in elements {
                check_expr_exhaustiveness(elem, env, table, diagnostics);
            }
        }

        ExprKind::Literal { .. } | ExprKind::Ident { .. } | ExprKind::Error => {}
    }
}

/// Check a single match expression for exhaustiveness.
fn check_match_exhaustiveness(
    scrutinee: &Expr,
    arms: &[MatchArm],
    env: &TypeEnv,
    table: &UnificationTable,
    diagnostics: &mut Vec<Diagnostic>,
) {
    // Check for unreachable patterns (anything after a wildcard/ident pattern)
    let mut has_wildcard = false;
    for (i, arm) in arms.iter().enumerate() {
        if has_wildcard {
            diagnostics.push(
                Diagnostic::warning(
                    "E0401",
                    "unreachable pattern",
                    arm.pattern.span,
                )
                .with_label(Label::new(arm.pattern.span, "this pattern is unreachable"))
                .with_help("a previous pattern already matches all values"),
            );
        }
        if is_catch_all_pattern(&arm.pattern) {
            has_wildcard = true;
            // If it's not the last arm, remaining arms are unreachable
            if i < arms.len() - 1 {
                // Will be reported on the next iteration
            }
        }
    }

    // If we have a catch-all, the match is exhaustive
    if has_wildcard {
        return;
    }

    // Determine what type is being matched and check coverage
    // We need to figure out the scrutinee type from patterns
    let scrutinee_type = infer_scrutinee_type_from_patterns(arms, env, table);

    match scrutinee_type {
        ScrutineeType::Bool => {
            check_bool_exhaustiveness(arms, scrutinee, diagnostics);
        }
        ScrutineeType::ADT(adt_name) => {
            check_adt_exhaustiveness(&adt_name, arms, scrutinee, env, diagnostics);
        }
        ScrutineeType::Option => {
            check_option_exhaustiveness(arms, scrutinee, diagnostics);
        }
        ScrutineeType::Result => {
            check_result_exhaustiveness(arms, scrutinee, diagnostics);
        }
        ScrutineeType::Int | ScrutineeType::Float | ScrutineeType::String => {
            // For infinite types, must have a wildcard
            diagnostics.push(
                Diagnostic::error(
                    "E0400",
                    "non-exhaustive match: pattern matching on a type with infinite values requires a wildcard `_` pattern",
                    scrutinee.span,
                )
                .with_label(Label::new(scrutinee.span, "match is not exhaustive"))
                .with_help("add a `_ -> ...` arm to handle remaining cases"),
            );
        }
        ScrutineeType::Unknown => {
            // Can't determine type; skip exhaustiveness check
        }
    }
}

/// Determine the type being matched from the patterns.
#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
enum ScrutineeType {
    Bool,
    Int,
    Float,
    String,
    ADT(String),
    Option,
    Result,
    Unknown,
}

fn infer_scrutinee_type_from_patterns(
    arms: &[MatchArm],
    env: &TypeEnv,
    _table: &UnificationTable,
) -> ScrutineeType {
    for arm in arms {
        match &arm.pattern.kind {
            PatternKind::Literal { value } => match value {
                LitValue::Bool(_) => return ScrutineeType::Bool,
                LitValue::Int(_) => return ScrutineeType::Int,
                LitValue::Float(_) => return ScrutineeType::Float,
                LitValue::String(_) => return ScrutineeType::String,
                LitValue::Unit => return ScrutineeType::String, // Unit is trivially exhaustive
            },
            PatternKind::Variant { name, .. } => {
                // Check built-in variants
                match name.as_str() {
                    "Some" | "None" => return ScrutineeType::Option,
                    "Ok" | "Err" => return ScrutineeType::Result,
                    _ => {
                        // Look up in ADT defs
                        if let Some(info) = env.variant_defs.get(name.as_str()) {
                            return ScrutineeType::ADT(info.adt_name.clone());
                        }
                    }
                }
            }
            PatternKind::Ident { .. } | PatternKind::Wildcard => {
                // Can't determine type from catch-all
                continue;
            }
        }
    }
    ScrutineeType::Unknown
}

/// Check Bool match exhaustiveness.
fn check_bool_exhaustiveness(
    arms: &[MatchArm],
    scrutinee: &Expr,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut has_true = false;
    let mut has_false = false;

    for arm in arms {
        match &arm.pattern.kind {
            PatternKind::Literal {
                value: LitValue::Bool(true),
            } => has_true = true,
            PatternKind::Literal {
                value: LitValue::Bool(false),
            } => has_false = true,
            PatternKind::Wildcard | PatternKind::Ident { .. } => {
                has_true = true;
                has_false = true;
            }
            _ => {}
        }
    }

    let mut missing = Vec::new();
    if !has_true {
        missing.push("true");
    }
    if !has_false {
        missing.push("false");
    }

    if !missing.is_empty() {
        diagnostics.push(
            Diagnostic::error(
                "E0400",
                format!(
                    "non-exhaustive match: missing pattern(s): {}",
                    missing.join(", ")
                ),
                scrutinee.span,
            )
            .with_label(Label::new(scrutinee.span, "match is not exhaustive"))
            .with_help(format!("add arm(s) for: {}", missing.join(", "))),
        );
    }
}

/// Check ADT match exhaustiveness.
fn check_adt_exhaustiveness(
    adt_name: &str,
    arms: &[MatchArm],
    scrutinee: &Expr,
    env: &TypeEnv,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let adt_info = if let Some(info) = env.adt_defs.get(adt_name) {
        info
    } else {
        return;
    };

    let all_variants: Vec<&str> = adt_info.variants.iter().map(|v| v.name.as_str()).collect();

    let mut covered: Vec<String> = Vec::new();

    for arm in arms {
        match &arm.pattern.kind {
            PatternKind::Variant { name, .. }
                if !covered.contains(name) => {
                    covered.push(name.clone());
                }
            PatternKind::Wildcard | PatternKind::Ident { .. } => {
                // Covers everything
                return;
            }
            _ => {}
        }
    }

    let missing: Vec<&&str> = all_variants
        .iter()
        .filter(|v| !covered.iter().any(|c| c == **v))
        .collect();

    if !missing.is_empty() {
        let missing_strs: Vec<String> = missing.iter().map(|v| v.to_string()).collect();
        diagnostics.push(
            Diagnostic::error(
                "E0400",
                format!(
                    "non-exhaustive match: missing pattern(s): {}",
                    missing_strs.join(", ")
                ),
                scrutinee.span,
            )
            .with_label(Label::new(scrutinee.span, "match is not exhaustive"))
            .with_help(format!("add arm(s) for: {}", missing_strs.join(", "))),
        );
    }
}

/// Check Option match exhaustiveness.
fn check_option_exhaustiveness(
    arms: &[MatchArm],
    scrutinee: &Expr,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut has_some = false;
    let mut has_none = false;

    for arm in arms {
        match &arm.pattern.kind {
            PatternKind::Variant { name, .. } => match name.as_str() {
                "Some" => has_some = true,
                "None" => has_none = true,
                _ => {}
            },
            PatternKind::Wildcard | PatternKind::Ident { .. } => {
                has_some = true;
                has_none = true;
            }
            _ => {}
        }
    }

    let mut missing = Vec::new();
    if !has_some {
        missing.push("Some(_)");
    }
    if !has_none {
        missing.push("None");
    }

    if !missing.is_empty() {
        diagnostics.push(
            Diagnostic::error(
                "E0400",
                format!(
                    "non-exhaustive match: missing pattern(s): {}",
                    missing.join(", ")
                ),
                scrutinee.span,
            )
            .with_label(Label::new(scrutinee.span, "match is not exhaustive"))
            .with_help(format!("add arm(s) for: {}", missing.join(", "))),
        );
    }
}

/// Check Result match exhaustiveness.
fn check_result_exhaustiveness(
    arms: &[MatchArm],
    scrutinee: &Expr,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut has_ok = false;
    let mut has_err = false;

    for arm in arms {
        match &arm.pattern.kind {
            PatternKind::Variant { name, .. } => match name.as_str() {
                "Ok" => has_ok = true,
                "Err" => has_err = true,
                _ => {}
            },
            PatternKind::Wildcard | PatternKind::Ident { .. } => {
                has_ok = true;
                has_err = true;
            }
            _ => {}
        }
    }

    let mut missing = Vec::new();
    if !has_ok {
        missing.push("Ok(_)");
    }
    if !has_err {
        missing.push("Err(_)");
    }

    if !missing.is_empty() {
        diagnostics.push(
            Diagnostic::error(
                "E0400",
                format!(
                    "non-exhaustive match: missing pattern(s): {}",
                    missing.join(", ")
                ),
                scrutinee.span,
            )
            .with_label(Label::new(scrutinee.span, "match is not exhaustive"))
            .with_help(format!("add arm(s) for: {}", missing.join(", "))),
        );
    }
}

/// Check if a pattern is a catch-all (matches everything).
fn is_catch_all_pattern(pattern: &Pattern) -> bool {
    matches!(
        pattern.kind,
        PatternKind::Wildcard | PatternKind::Ident { .. }
    )
}
