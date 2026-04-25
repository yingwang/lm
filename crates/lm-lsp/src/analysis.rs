//! Source analysis: run the full LM pipeline and collect results.
//!
//! This module provides [`AnalysisResult`] which bundles diagnostics,
//! hover information (type at position), definition locations, and
//! document symbols extracted from a single source file.

use crate::convert::LineIndex;
use lm_diagnostics::{Diagnostic, DiagnosticBag, Span};
use lm_lexer::Lexer;
use lm_parser::ast::*;
use tower_lsp::lsp_types::{DocumentSymbol, Position, Range, SymbolKind};

/// The result of analyzing a single LM source file.
#[derive(Debug)]
pub struct AnalysisResult {
    /// All diagnostics (lex + parse + type check).
    pub diagnostics: Vec<Diagnostic>,
    /// Top-level symbol information for the document outline.
    pub symbols: Vec<DocumentSymbol>,
    /// Hover information: maps (line, column) ranges to type description strings.
    pub hover_map: Vec<HoverEntry>,
    /// Definition locations: maps usage spans to definition spans.
    pub def_map: Vec<DefEntry>,
}

/// A hover entry mapping a source range to a type description.
#[derive(Debug, Clone)]
pub struct HoverEntry {
    /// The range in the source that this hover applies to.
    pub range: Range,
    /// The type description to show on hover.
    pub description: String,
}

/// A definition entry mapping a usage range to a definition range.
#[derive(Debug, Clone)]
pub struct DefEntry {
    /// The range of the usage site.
    pub usage_range: Range,
    /// The range of the definition site.
    pub def_range: Range,
}

/// Analyze a single LM source file.
///
/// Runs the full pipeline: lex -> parse -> type check, and extracts
/// hover, definition, and symbol information from the AST.
pub fn analyze(source: &str) -> AnalysisResult {
    let line_index = LineIndex::new(source);

    // Lex
    let (tokens, lex_diagnostics) = Lexer::new(source, 0).tokenize();

    let mut bag = DiagnosticBag::new();
    for d in lex_diagnostics {
        bag.add(d);
    }

    // Parse
    let (program, parse_diagnostics) = lm_parser::Parser::new(tokens).parse();
    for d in parse_diagnostics {
        bag.add(d);
    }

    // Type check (only if no parse errors)
    if !bag.has_errors() {
        let checker = lm_types::TypeChecker::new();
        let type_diagnostics = checker.check(&program);
        for d in type_diagnostics {
            bag.add(d);
        }
    }

    // Extract symbols, hover info, and definitions from the AST
    let symbols = extract_symbols(&program, &line_index);
    let (hover_map, def_map) = extract_navigation(&program, &line_index);

    AnalysisResult {
        diagnostics: bag.into_vec(),
        symbols,
        hover_map,
        def_map,
    }
}

/// Extract document symbols from the AST for the outline view.
#[allow(deprecated)]
fn extract_symbols(program: &Program, index: &LineIndex) -> Vec<DocumentSymbol> {
    let mut symbols = Vec::new();

    for decl in &program.decls {
        let range = index.span_to_range(decl.span);
        match &decl.kind {
            DeclKind::FnDef {
                name,
                effect,
                params,
                return_type,
                ..
            } => {
                let detail = build_fn_detail(*effect, params, return_type.as_ref());
                symbols.push(DocumentSymbol {
                    name: name.clone(),
                    detail: Some(detail),
                    kind: SymbolKind::FUNCTION,
                    tags: None,
                    deprecated: None,
                    range,
                    selection_range: range,
                    children: None,
                });
            }
            DeclKind::TypeDef {
                name,
                type_params,
                variants,
            } => {
                let detail = if type_params.is_empty() {
                    None
                } else {
                    Some(format!("<{}>", type_params.join(", ")))
                };
                let children: Vec<DocumentSymbol> = variants
                    .iter()
                    .map(|v| {
                        let vrange = index.span_to_range(v.span);
                        DocumentSymbol {
                            name: v.name.clone(),
                            detail: None,
                            kind: SymbolKind::ENUM_MEMBER,
                            tags: None,
                            deprecated: None,
                            range: vrange,
                            selection_range: vrange,
                            children: None,
                        }
                    })
                    .collect();
                symbols.push(DocumentSymbol {
                    name: name.clone(),
                    detail,
                    kind: SymbolKind::ENUM,
                    tags: None,
                    deprecated: None,
                    range,
                    selection_range: range,
                    children: if children.is_empty() {
                        None
                    } else {
                        Some(children)
                    },
                });
            }
            DeclKind::LetDef { name, .. } => {
                symbols.push(DocumentSymbol {
                    name: name.clone(),
                    detail: None,
                    kind: SymbolKind::VARIABLE,
                    tags: None,
                    deprecated: None,
                    range,
                    selection_range: range,
                    children: None,
                });
            }
        }
    }

    symbols
}

/// Build a detail string for a function signature.
fn build_fn_detail(
    effect: Effect,
    params: &[Param],
    return_type: Option<&TypeAnnotation>,
) -> String {
    let prefix = match effect {
        Effect::Pure => "fn",
        Effect::Io => "io fn",
    };
    let param_strs: Vec<String> = params
        .iter()
        .map(|p| {
            if let Some(ann) = &p.type_annotation {
                format!("{}: {}", p.name, type_annotation_to_string(ann))
            } else {
                p.name.clone()
            }
        })
        .collect();
    let ret = return_type
        .map(|r| format!(" -> {}", type_annotation_to_string(r)))
        .unwrap_or_default();
    format!("{}({}){}", prefix, param_strs.join(", "), ret)
}

/// Convert a type annotation AST node to a display string.
fn type_annotation_to_string(ann: &TypeAnnotation) -> String {
    match &ann.kind {
        TypeKind::Name { name } => name.clone(),
        TypeKind::App { name, args } => {
            let arg_strs: Vec<String> = args.iter().map(type_annotation_to_string).collect();
            format!("{}<{}>", name, arg_strs.join(", "))
        }
        TypeKind::Fn { params, ret } => {
            let param_strs: Vec<String> = params.iter().map(type_annotation_to_string).collect();
            format!("({}) -> {}", param_strs.join(", "), type_annotation_to_string(ret))
        }
    }
}

/// Extract hover and go-to-definition information from the AST.
///
/// This builds simplified mappings based on the AST structure. A full
/// implementation would integrate with the type checker to get inferred
/// types, but for now we extract what we can from the AST alone:
///
/// - **Hover:** function names show their declared signature; let bindings
///   show their type annotation if present.
/// - **Go-to-definition:** identifiers in expressions map to the enclosing
///   let binding or function parameter declaration.
fn extract_navigation(program: &Program, index: &LineIndex) -> (Vec<HoverEntry>, Vec<DefEntry>) {
    let mut hover_map = Vec::new();
    let mut def_map = Vec::new();
    let mut scope: Vec<(String, Span)> = Vec::new();

    for decl in &program.decls {
        match &decl.kind {
            DeclKind::FnDef {
                name,
                effect,
                params,
                return_type,
                body,
            } => {
                // Hover on the function name itself shows the signature
                let detail = build_fn_detail(*effect, params, return_type.as_ref());
                // The function name is at the start of the declaration span.
                // We approximate: the name span is hard to extract without
                // storing it separately, so we use the whole decl span as a
                // fallback for the symbol. Hover will show on any part of
                // the function decl line.
                hover_map.push(HoverEntry {
                    range: index.span_to_range(decl.span),
                    description: detail,
                });

                // Record function name in scope for references
                scope.push((name.clone(), decl.span));

                // Record parameters in scope
                let mut fn_scope = scope.clone();
                for param in params {
                    fn_scope.push((param.name.clone(), param.span));
                    if let Some(ann) = &param.type_annotation {
                        hover_map.push(HoverEntry {
                            range: index.span_to_range(param.span),
                            description: format!(
                                "{}: {}",
                                param.name,
                                type_annotation_to_string(ann)
                            ),
                        });
                    }
                }

                // Walk body for identifier references
                walk_expr_for_defs(body, &fn_scope, &mut def_map, index);
            }
            DeclKind::LetDef {
                name,
                type_annotation,
                value,
            } => {
                if let Some(ann) = type_annotation {
                    hover_map.push(HoverEntry {
                        range: index.span_to_range(decl.span),
                        description: format!(
                            "let {}: {}",
                            name,
                            type_annotation_to_string(ann)
                        ),
                    });
                } else {
                    hover_map.push(HoverEntry {
                        range: index.span_to_range(decl.span),
                        description: format!("let {}", name),
                    });
                }
                scope.push((name.clone(), decl.span));

                walk_expr_for_defs(value, &scope, &mut def_map, index);
            }
            DeclKind::TypeDef { .. } => {
                // Type definitions don't introduce expression-level bindings
            }
        }
    }

    (hover_map, def_map)
}

/// Walk an expression tree looking for identifier references and recording
/// go-to-definition links.
fn walk_expr_for_defs(
    expr: &Expr,
    scope: &[(String, Span)],
    def_map: &mut Vec<DefEntry>,
    index: &LineIndex,
) {
    match &expr.kind {
        ExprKind::Ident { name } => {
            // Find the definition in scope (last binding wins)
            if let Some((_, def_span)) = scope.iter().rev().find(|(n, _)| n == name) {
                def_map.push(DefEntry {
                    usage_range: index.span_to_range(expr.span),
                    def_range: index.span_to_range(*def_span),
                });
            }
        }
        ExprKind::BinaryOp { lhs, rhs, .. } => {
            walk_expr_for_defs(lhs, scope, def_map, index);
            walk_expr_for_defs(rhs, scope, def_map, index);
        }
        ExprKind::UnaryOp { operand, .. } => {
            walk_expr_for_defs(operand, scope, def_map, index);
        }
        ExprKind::FnCall { callee, args } => {
            walk_expr_for_defs(callee, scope, def_map, index);
            for arg in args {
                walk_expr_for_defs(arg, scope, def_map, index);
            }
        }
        ExprKind::LetExpr {
            name, value, body, ..
        } => {
            walk_expr_for_defs(value, scope, def_map, index);
            let mut inner_scope = scope.to_vec();
            inner_scope.push((name.clone(), expr.span));
            walk_expr_for_defs(body, &inner_scope, def_map, index);
        }
        ExprKind::IfElse {
            condition,
            then_branch,
            else_branch,
        } => {
            walk_expr_for_defs(condition, scope, def_map, index);
            walk_expr_for_defs(then_branch, scope, def_map, index);
            walk_expr_for_defs(else_branch, scope, def_map, index);
        }
        ExprKind::Match { scrutinee, arms } => {
            walk_expr_for_defs(scrutinee, scope, def_map, index);
            for arm in arms {
                let mut arm_scope = scope.to_vec();
                collect_pattern_bindings(&arm.pattern, &mut arm_scope);
                walk_expr_for_defs(&arm.body, &arm_scope, def_map, index);
            }
        }
        ExprKind::Block { exprs } => {
            let mut block_scope = scope.to_vec();
            for e in exprs {
                walk_expr_for_defs(e, &block_scope, def_map, index);
                // If it's a let, add binding to scope for subsequent exprs
                if let ExprKind::LetExpr { name, .. } = &e.kind {
                    block_scope.push((name.clone(), e.span));
                }
            }
        }
        ExprKind::VariantConstruct { args, .. } => {
            for arg in args {
                walk_expr_for_defs(arg, scope, def_map, index);
            }
        }
        ExprKind::ListLiteral { elements } => {
            for elem in elements {
                walk_expr_for_defs(elem, scope, def_map, index);
            }
        }
        ExprKind::Literal { .. } | ExprKind::Error => {}
    }
}

/// Collect variable bindings from a pattern into the scope.
fn collect_pattern_bindings(pattern: &Pattern, scope: &mut Vec<(String, Span)>) {
    match &pattern.kind {
        PatternKind::Ident { name } => {
            scope.push((name.clone(), pattern.span));
        }
        PatternKind::Variant { fields, .. } => {
            for field in fields {
                collect_pattern_bindings(field, scope);
            }
        }
        PatternKind::Literal { .. } | PatternKind::Wildcard => {}
    }
}

/// Find a hover entry that contains the given position.
pub fn find_hover(entries: &[HoverEntry], pos: Position) -> Option<&HoverEntry> {
    entries.iter().rev().find(|e| position_in_range(pos, e.range))
}

/// Find a definition entry that contains the given position.
pub fn find_definition(entries: &[DefEntry], pos: Position) -> Option<&DefEntry> {
    entries.iter().find(|e| position_in_range(pos, e.usage_range))
}

/// Check whether a position falls within a range (inclusive start, exclusive end).
fn position_in_range(pos: Position, range: Range) -> bool {
    if pos.line < range.start.line || pos.line > range.end.line {
        return false;
    }
    if pos.line == range.start.line && pos.character < range.start.character {
        return false;
    }
    if pos.line == range.end.line && pos.character >= range.end.character {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_valid_program() {
        let source = "fn add(x: Int, y: Int) -> Int {\n  x + y\n}\n";
        let result = analyze(source);
        assert!(result.diagnostics.is_empty());
        assert_eq!(result.symbols.len(), 1);
        assert_eq!(result.symbols[0].name, "add");
        assert_eq!(result.symbols[0].kind, SymbolKind::FUNCTION);
    }

    #[test]
    fn test_analyze_with_errors() {
        let source = "fn bad() -> Int {\n  \"hello\"\n}\n";
        let result = analyze(source);
        assert!(!result.diagnostics.is_empty());
    }

    #[test]
    fn test_analyze_type_def() {
        let source = "type Shape = Circle(Float) | Rectangle(Float, Float)\n";
        let result = analyze(source);
        assert!(result.diagnostics.is_empty());
        assert_eq!(result.symbols.len(), 1);
        assert_eq!(result.symbols[0].name, "Shape");
        assert_eq!(result.symbols[0].kind, SymbolKind::ENUM);
    }

    #[test]
    fn test_analyze_let_def() {
        let source = "let x: Int = 42;\n";
        let result = analyze(source);
        assert!(result.diagnostics.is_empty());
        assert_eq!(result.symbols.len(), 1);
        assert_eq!(result.symbols[0].name, "x");
        assert_eq!(result.symbols[0].kind, SymbolKind::VARIABLE);
    }

    #[test]
    fn test_analyze_lex_error() {
        let source = "let x = @;\n";
        let result = analyze(source);
        assert!(!result.diagnostics.is_empty());
    }

    #[test]
    fn test_position_in_range() {
        let range = Range::new(Position::new(1, 5), Position::new(1, 10));
        assert!(position_in_range(Position::new(1, 5), range));
        assert!(position_in_range(Position::new(1, 7), range));
        assert!(!position_in_range(Position::new(1, 10), range));
        assert!(!position_in_range(Position::new(0, 7), range));
        assert!(!position_in_range(Position::new(2, 0), range));
    }

    #[test]
    fn test_hover_map_populated() {
        let source = "fn double(x: Int) -> Int {\n  x + x\n}\n";
        let result = analyze(source);
        assert!(!result.hover_map.is_empty());
        // The function declaration should have a hover entry
        let fn_hover = result.hover_map.iter().find(|h| h.description.contains("fn"));
        assert!(fn_hover.is_some());
    }
}
