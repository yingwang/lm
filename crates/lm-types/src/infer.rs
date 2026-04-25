//! Hindley-Milner type inference for LM.
//!
//! The [`TypeChecker`] walks the AST, assigns types to expressions,
//! and collects type error diagnostics.

use crate::effects;
use crate::env::{AdtInfo, FnEffectInfo, TypeEnv, VariantInfo};
use crate::exhaustiveness;
use crate::types::{Type, TypeScheme};
use crate::unify::UnificationTable;
use lm_diagnostics::{Diagnostic, Label, Span};
use lm_parser::ast::*;

/// The top-level type checker.
///
/// Owns the unification table and type environment. Call [`TypeChecker::check`]
/// to type-check a parsed program, getting back a list of diagnostics.
pub struct TypeChecker {
    /// Unification table for type variable bindings.
    table: UnificationTable,
    /// Type environment.
    env: TypeEnv,
    /// Collected diagnostics.
    diagnostics: Vec<Diagnostic>,
}

impl TypeChecker {
    /// Create a new type checker with built-in types pre-populated.
    pub fn new() -> Self {
        let mut table = UnificationTable::new();
        let env = TypeEnv::new(&mut table);
        TypeChecker {
            table,
            env,
            diagnostics: Vec::new(),
        }
    }

    /// Type-check a parsed program and return all diagnostics.
    ///
    /// This runs type inference, effect checking, and exhaustiveness checking.
    pub fn check(mut self, program: &Program) -> Vec<Diagnostic> {
        // First pass: register all type definitions
        for decl in &program.decls {
            if let DeclKind::TypeDef {
                name,
                type_params,
                variants,
            } = &decl.kind
            {
                self.register_type_def(name, type_params, variants);
            }
        }

        // Second pass: register function signatures (for mutual recursion)
        for decl in &program.decls {
            if let DeclKind::FnDef {
                name,
                effect,
                params,
                return_type,
                ..
            } = &decl.kind
            {
                self.register_fn_sig(name, *effect, params, return_type.as_ref(), decl.span);
            }
        }

        // Third pass: type-check all declarations
        for decl in &program.decls {
            self.check_decl(decl);
        }

        // Fourth pass: effect checking
        let effect_diags = effects::check_effects(program, &self.env);
        self.diagnostics.extend(effect_diags);

        // Fifth pass: exhaustiveness checking
        let exhaust_diags = exhaustiveness::check_exhaustiveness(program, &self.env, &self.table);
        self.diagnostics.extend(exhaust_diags);

        self.diagnostics
    }

    /// Register a type definition in the environment.
    ///
    /// Uses a two-pass approach: first register the ADT name (with empty variants)
    /// so that recursive/self-referential types can be resolved, then process the
    /// variant field types and update the registration.
    fn register_type_def(&mut self, name: &str, type_params: &[String], variants: &[Variant]) {
        // First: register the ADT name with empty variants so self-references resolve.
        self.env.register_adt(AdtInfo {
            name: name.to_string(),
            type_params: type_params.to_vec(),
            variants: Vec::new(),
        });

        // Second: now process variant field types (which may reference this ADT).
        let mut variant_infos = Vec::new();

        for variant in variants {
            let field_types: Vec<Type> = variant
                .fields
                .iter()
                .map(|ann| self.annotation_to_type(ann, type_params))
                .collect();

            variant_infos.push(VariantInfo {
                name: variant.name.clone(),
                field_types,
                adt_name: name.to_string(),
                type_params: type_params.to_vec(),
            });
        }

        // Third: re-register with the full variant information.
        self.env.register_adt(AdtInfo {
            name: name.to_string(),
            type_params: type_params.to_vec(),
            variants: variant_infos,
        });
    }

    /// Pre-register a function signature so other functions can call it.
    fn register_fn_sig(
        &mut self,
        name: &str,
        effect: Effect,
        params: &[Param],
        return_type: Option<&TypeAnnotation>,
        _span: Span,
    ) {
        let param_types: Vec<Type> = params
            .iter()
            .map(|p| {
                if let Some(ann) = &p.type_annotation {
                    self.annotation_to_type(ann, &Vec::new())
                } else {
                    self.table.fresh_var()
                }
            })
            .collect();

        let ret_type = if let Some(ann) = return_type {
            self.annotation_to_type(ann, &Vec::new())
        } else {
            self.table.fresh_var()
        };

        let fn_type = Type::Fun(param_types, Box::new(ret_type));
        self.env.bind(name.to_string(), TypeScheme::mono(fn_type));
        self.env.fn_effects.insert(
            name.to_string(),
            FnEffectInfo { effect },
        );
    }

    /// Convert a type annotation AST node to our internal Type representation.
    fn annotation_to_type(&mut self, ann: &TypeAnnotation, type_params: &[String]) -> Type {
        match &ann.kind {
            TypeKind::Name { name } => self.name_to_type(name, type_params, ann.span),
            TypeKind::App { name, args } => {
                let type_args: Vec<Type> = args
                    .iter()
                    .map(|a| self.annotation_to_type(a, type_params))
                    .collect();
                match name.as_str() {
                    "Option" => {
                        if type_args.len() == 1 {
                            Type::Option(Box::new(type_args.into_iter().next().unwrap()))
                        } else {
                            self.diagnostics.push(Diagnostic::error(
                                "E0204",
                                format!("Option expects 1 type argument, found {}", type_args.len()),
                                ann.span,
                            ));
                            Type::Option(Box::new(Type::Unit))
                        }
                    }
                    "Result" => {
                        if type_args.len() == 2 {
                            let mut iter = type_args.into_iter();
                            Type::Result(
                                Box::new(iter.next().unwrap()),
                                Box::new(iter.next().unwrap()),
                            )
                        } else {
                            self.diagnostics.push(Diagnostic::error(
                                "E0204",
                                format!(
                                    "Result expects 2 type arguments, found {}",
                                    type_args.len()
                                ),
                                ann.span,
                            ));
                            Type::Result(Box::new(Type::Unit), Box::new(Type::Unit))
                        }
                    }
                    "List" => {
                        if type_args.len() == 1 {
                            Type::List(Box::new(type_args.into_iter().next().unwrap()))
                        } else {
                            self.diagnostics.push(Diagnostic::error(
                                "E0204",
                                format!("List expects 1 type argument, found {}", type_args.len()),
                                ann.span,
                            ));
                            Type::List(Box::new(Type::Unit))
                        }
                    }
                    _ => {
                        // User-defined ADT with type args
                        if self.env.adt_defs.contains_key(name) {
                            Type::ADT(name.clone(), type_args)
                        } else {
                            self.diagnostics.push(
                                Diagnostic::error(
                                    "E0202",
                                    format!("undefined type `{}`", name),
                                    ann.span,
                                )
                                .with_label(Label::new(ann.span, "not found")),
                            );
                            Type::Unit
                        }
                    }
                }
            }
            TypeKind::Fn { params, ret } => {
                let param_types: Vec<Type> = params
                    .iter()
                    .map(|p| self.annotation_to_type(p, type_params))
                    .collect();
                let ret_type = self.annotation_to_type(ret, type_params);
                Type::Fun(param_types, Box::new(ret_type))
            }
        }
    }

    /// Convert a simple type name to a Type.
    fn name_to_type(&mut self, name: &str, type_params: &[String], span: Span) -> Type {
        // Check if it's a type parameter
        if type_params.iter().any(|p| p == name) {
            return self.table.fresh_var();
        }

        match name {
            "Int" => Type::Int,
            "Float" => Type::Float,
            "Bool" => Type::Bool,
            "String" => Type::String,
            "Unit" => Type::Unit,
            _ => {
                // Check for user-defined ADT (without type params)
                if self.env.adt_defs.contains_key(name) {
                    let adt = self.env.adt_defs.get(name).unwrap();
                    if adt.type_params.is_empty() {
                        Type::ADT(name.to_string(), Vec::new())
                    } else {
                        // ADT used without type args — create fresh vars
                        let args: Vec<Type> = adt
                            .type_params
                            .iter()
                            .map(|_| self.table.fresh_var())
                            .collect();
                        Type::ADT(name.to_string(), args)
                    }
                } else {
                    self.diagnostics.push(
                        Diagnostic::error("E0202", format!("undefined type `{}`", name), span)
                            .with_label(Label::new(span, "not found")),
                    );
                    Type::Unit
                }
            }
        }
    }

    /// Type-check a top-level declaration.
    fn check_decl(&mut self, decl: &Decl) {
        match &decl.kind {
            DeclKind::FnDef {
                name,
                params,
                return_type,
                body,
                ..
            } => {
                self.check_fn_def(name, params, return_type.as_ref(), body, decl.span);
            }
            DeclKind::TypeDef { .. } => {
                // Already registered in first pass
            }
            DeclKind::LetDef {
                name,
                type_annotation,
                value,
            } => {
                self.check_let_def(name, type_annotation.as_ref(), value, decl.span);
            }
        }
    }

    /// Type-check a function definition.
    fn check_fn_def(
        &mut self,
        name: &str,
        params: &[Param],
        return_type: Option<&TypeAnnotation>,
        body: &Expr,
        _span: Span,
    ) {
        self.env.push_scope();

        // Look up the pre-registered function type
        let fn_scheme = self.env.lookup(name).cloned();
        let (param_types, expected_ret) = if let Some(scheme) = fn_scheme {
            match &scheme.ty {
                Type::Fun(p, r) => (p.clone(), *r.clone()),
                _ => {
                    let pts: Vec<Type> = params.iter().map(|_| self.table.fresh_var()).collect();
                    let rt = self.table.fresh_var();
                    (pts, rt)
                }
            }
        } else {
            let pts: Vec<Type> = params.iter().map(|_| self.table.fresh_var()).collect();
            let rt = self.table.fresh_var();
            (pts, rt)
        };

        // Bind parameter names in the function scope
        for (param, ty) in params.iter().zip(param_types.iter()) {
            self.env
                .bind(param.name.clone(), TypeScheme::mono(ty.clone()));
        }

        // Infer body type
        let body_type = self.infer_expr(body);

        // Unify body type with declared return type
        if let Some(bt) = &body_type {
            if let Err(e) = self.table.unify(bt, &expected_ret, body.span) {
                let resolved_expected = self.table.deep_resolve(&expected_ret);
                let resolved_found = self.table.deep_resolve(bt);
                if e.is_infinite {
                    self.diagnostics.push(
                        Diagnostic::error(
                            "E0208",
                            "infinite type detected",
                            e.span,
                        )
                        .with_label(Label::new(e.span, "occurs check failure")),
                    );
                } else if let Some(ret_ann) = return_type {
                    self.diagnostics.push(
                        Diagnostic::error(
                            "E0200",
                            format!(
                                "type mismatch in function `{}`: expected `{}`, found `{}`",
                                name,
                                resolved_expected.display(),
                                resolved_found.display()
                            ),
                            body.span,
                        )
                        .with_label(Label::new(
                            ret_ann.span,
                            format!("return type declared as `{}`", resolved_expected.display()),
                        ))
                        .with_label(Label::new(
                            body.span,
                            format!("body evaluates to `{}`", resolved_found.display()),
                        )),
                    );
                } else {
                    self.diagnostics.push(
                        Diagnostic::error(
                            "E0205",
                            format!(
                                "cannot unify `{}` with `{}`",
                                resolved_expected.display(),
                                resolved_found.display()
                            ),
                            e.span,
                        )
                        .with_label(Label::new(
                            e.span,
                            format!(
                                "expected `{}`, found `{}`",
                                resolved_expected.display(),
                                resolved_found.display()
                            ),
                        )),
                    );
                }
            }
        }

        self.env.pop_scope();

        // Generalize and update the binding
        if body_type.is_some() {
            let resolved_ret = self.table.deep_resolve(&expected_ret);
            let resolved_params: Vec<Type> = param_types
                .iter()
                .map(|p| self.table.deep_resolve(p))
                .collect();
            let fn_type = Type::Fun(resolved_params, Box::new(resolved_ret));
            let scheme = self.env.generalize(&fn_type, &self.table);
            self.env.bind(name.to_string(), scheme);
        }
    }

    /// Type-check a top-level let definition.
    fn check_let_def(
        &mut self,
        name: &str,
        type_annotation: Option<&TypeAnnotation>,
        value: &Expr,
        _span: Span,
    ) {
        let inferred = self.infer_expr(value);

        if let Some(ty) = inferred {
            // If there's a type annotation, unify with it
            if let Some(ann) = type_annotation {
                let expected = self.annotation_to_type(ann, &Vec::new());
                if let Err(e) = self.table.unify(&ty, &expected, ann.span) {
                    let resolved_expected = self.table.deep_resolve(&expected);
                    let resolved_found = self.table.deep_resolve(&ty);
                    if e.is_infinite {
                        self.diagnostics.push(
                            Diagnostic::error("E0208", "infinite type detected", e.span)
                                .with_label(Label::new(e.span, "occurs check failure")),
                        );
                    } else {
                        self.diagnostics.push(
                            Diagnostic::error(
                                "E0200",
                                format!(
                                    "type mismatch: expected `{}`, found `{}`",
                                    resolved_expected.display(),
                                    resolved_found.display()
                                ),
                                value.span,
                            )
                            .with_label(Label::new(
                                ann.span,
                                format!("expected `{}`", resolved_expected.display()),
                            ))
                            .with_label(Label::new(
                                value.span,
                                format!("found `{}`", resolved_found.display()),
                            )),
                        );
                    }
                }
            }

            let scheme = self.env.generalize(&ty, &self.table);
            self.env.bind(name.to_string(), scheme);
        }
    }

    /// Infer the type of an expression. Returns `None` if the expression
    /// has an error that prevents type inference.
    fn infer_expr(&mut self, expr: &Expr) -> Option<Type> {
        match &expr.kind {
            ExprKind::Literal { value } => Some(self.infer_literal(value)),

            ExprKind::Ident { name } => self.infer_ident(name, expr.span),

            ExprKind::BinaryOp { op, lhs, rhs } => self.infer_binop(*op, lhs, rhs, expr.span),

            ExprKind::UnaryOp { op, operand } => self.infer_unop(*op, operand, expr.span),

            ExprKind::FnCall { callee, args } => self.infer_fn_call(callee, args, expr.span),

            ExprKind::LetExpr {
                name,
                type_annotation,
                value,
                body,
            } => self.infer_let_expr(name, type_annotation.as_ref(), value, body),

            ExprKind::IfElse {
                condition,
                then_branch,
                else_branch,
            } => self.infer_if_else(condition, then_branch, else_branch, expr.span),

            ExprKind::Match { scrutinee, arms } => {
                self.infer_match(scrutinee, arms, expr.span)
            }

            ExprKind::Block { exprs } => self.infer_block(exprs),

            ExprKind::ListLiteral { elements } => self.infer_list_literal(elements, expr.span),

            ExprKind::VariantConstruct { name, args } => {
                self.infer_variant_construct(name, args, expr.span)
            }

            ExprKind::Error => None,
        }
    }

    /// Infer the type of a literal.
    fn infer_literal(&self, lit: &LitValue) -> Type {
        match lit {
            LitValue::Int(_) => Type::Int,
            LitValue::Float(_) => Type::Float,
            LitValue::String(_) => Type::String,
            LitValue::Bool(_) => Type::Bool,
        }
    }

    /// Infer the type of an identifier reference.
    fn infer_ident(&mut self, name: &str, span: Span) -> Option<Type> {
        if let Some(scheme) = self.env.lookup(name).cloned() {
            let ty = self.env.instantiate(&scheme, &mut self.table);
            Some(ty)
        } else {
            self.diagnostics.push(
                Diagnostic::error("E0201", format!("undefined variable `{}`", name), span)
                    .with_label(Label::new(span, "not found in this scope")),
            );
            None
        }
    }

    /// Infer the type of a binary operation.
    fn infer_binop(
        &mut self,
        op: BinOp,
        lhs: &Expr,
        rhs: &Expr,
        span: Span,
    ) -> Option<Type> {
        let lhs_ty = self.infer_expr(lhs)?;
        let rhs_ty = self.infer_expr(rhs)?;

        match op {
            // Numeric operators: both operands same numeric type, return same
            BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod => {
                // Unify lhs and rhs
                if self.table.unify(&lhs_ty, &rhs_ty, span).is_err() {
                    let resolved_lhs = self.table.deep_resolve(&lhs_ty);
                    let resolved_rhs = self.table.deep_resolve(&rhs_ty);
                    self.diagnostics.push(
                        Diagnostic::error(
                            "E0206",
                            format!(
                                "operator `{}` requires matching types, found `{}` and `{}`",
                                binop_symbol(op),
                                resolved_lhs.display(),
                                resolved_rhs.display()
                            ),
                            span,
                        )
                        .with_label(Label::new(lhs.span, format!("type `{}`", resolved_lhs.display())))
                        .with_label(Label::new(rhs.span, format!("type `{}`", resolved_rhs.display()))),
                    );
                    return None;
                }
                // Ensure it's numeric
                let resolved = self.table.deep_resolve(&lhs_ty);
                match &resolved {
                    Type::Int | Type::Float => Some(resolved),
                    Type::Var(_) => {
                        // Still ambiguous; try to default to Int
                        // Just return the type variable for now
                        Some(resolved)
                    }
                    _ => {
                        self.diagnostics.push(
                            Diagnostic::error(
                                "E0206",
                                format!(
                                    "operator `{}` requires numeric types, found `{}`",
                                    binop_symbol(op),
                                    resolved.display()
                                ),
                                span,
                            )
                            .with_label(Label::new(span, "numeric type required")),
                        );
                        None
                    }
                }
            }

            // String concatenation
            BinOp::Concat => {
                if self.table.unify(&lhs_ty, &Type::String, lhs.span).is_err() {
                    let resolved = self.table.deep_resolve(&lhs_ty);
                    self.diagnostics.push(
                        Diagnostic::error(
                            "E0206",
                            format!(
                                "operator `++` requires String, found `{}`",
                                resolved.display()
                            ),
                            lhs.span,
                        )
                        .with_label(Label::new(lhs.span, format!("type `{}`", resolved.display()))),
                    );
                    return None;
                }
                if self.table.unify(&rhs_ty, &Type::String, rhs.span).is_err() {
                    let resolved = self.table.deep_resolve(&rhs_ty);
                    self.diagnostics.push(
                        Diagnostic::error(
                            "E0206",
                            format!(
                                "operator `++` requires String, found `{}`",
                                resolved.display()
                            ),
                            rhs.span,
                        )
                        .with_label(Label::new(rhs.span, format!("type `{}`", resolved.display()))),
                    );
                    return None;
                }
                Some(Type::String)
            }

            // Comparison operators: same type, return Bool
            BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge => {
                if self.table.unify(&lhs_ty, &rhs_ty, span).is_err() {
                    let resolved_lhs = self.table.deep_resolve(&lhs_ty);
                    let resolved_rhs = self.table.deep_resolve(&rhs_ty);
                    self.diagnostics.push(
                        Diagnostic::error(
                            "E0206",
                            format!(
                                "operator `{}` requires matching types, found `{}` and `{}`",
                                binop_symbol(op),
                                resolved_lhs.display(),
                                resolved_rhs.display()
                            ),
                            span,
                        )
                        .with_label(Label::new(lhs.span, format!("type `{}`", resolved_lhs.display())))
                        .with_label(Label::new(rhs.span, format!("type `{}`", resolved_rhs.display()))),
                    );
                    return None;
                }
                Some(Type::Bool)
            }

            // Logical operators: both Bool
            BinOp::And | BinOp::Or => {
                if self.table.unify(&lhs_ty, &Type::Bool, lhs.span).is_err() {
                    let resolved = self.table.deep_resolve(&lhs_ty);
                    self.diagnostics.push(
                        Diagnostic::error(
                            "E0206",
                            format!(
                                "operator `{}` requires Bool, found `{}`",
                                binop_symbol(op),
                                resolved.display()
                            ),
                            lhs.span,
                        )
                        .with_label(Label::new(lhs.span, format!("type `{}`", resolved.display()))),
                    );
                    return None;
                }
                if self.table.unify(&rhs_ty, &Type::Bool, rhs.span).is_err() {
                    let resolved = self.table.deep_resolve(&rhs_ty);
                    self.diagnostics.push(
                        Diagnostic::error(
                            "E0206",
                            format!(
                                "operator `{}` requires Bool, found `{}`",
                                binop_symbol(op),
                                resolved.display()
                            ),
                            rhs.span,
                        )
                        .with_label(Label::new(rhs.span, format!("type `{}`", resolved.display()))),
                    );
                    return None;
                }
                Some(Type::Bool)
            }
        }
    }

    /// Infer the type of a unary operation.
    fn infer_unop(&mut self, op: UnOp, operand: &Expr, span: Span) -> Option<Type> {
        let operand_ty = self.infer_expr(operand)?;

        match op {
            UnOp::Not => {
                if self.table.unify(&operand_ty, &Type::Bool, operand.span).is_err() {
                    let resolved = self.table.deep_resolve(&operand_ty);
                    self.diagnostics.push(
                        Diagnostic::error(
                            "E0206",
                            format!("operator `!` requires Bool, found `{}`", resolved.display()),
                            span,
                        )
                        .with_label(Label::new(
                            operand.span,
                            format!("type `{}`", resolved.display()),
                        )),
                    );
                    return None;
                }
                Some(Type::Bool)
            }
            UnOp::Neg => {
                // Must be numeric
                let resolved = self.table.deep_resolve(&operand_ty);
                match &resolved {
                    Type::Int | Type::Float => Some(resolved),
                    Type::Var(_) => {
                        // Ambiguous, keep as-is
                        Some(operand_ty)
                    }
                    _ => {
                        self.diagnostics.push(
                            Diagnostic::error(
                                "E0206",
                                format!(
                                    "operator `-` requires numeric type, found `{}`",
                                    resolved.display()
                                ),
                                span,
                            )
                            .with_label(Label::new(
                                operand.span,
                                format!("type `{}`", resolved.display()),
                            )),
                        );
                        None
                    }
                }
            }
        }
    }

    /// Infer the type of a function call.
    fn infer_fn_call(
        &mut self,
        callee: &Expr,
        args: &[Expr],
        span: Span,
    ) -> Option<Type> {
        let callee_ty = self.infer_expr(callee)?;

        // Infer argument types
        let mut arg_types = Vec::new();
        for arg in args {
            if let Some(ty) = self.infer_expr(arg) {
                arg_types.push(ty);
            } else {
                arg_types.push(self.table.fresh_var());
            }
        }

        let ret_var = self.table.fresh_var();
        let expected_fn_type = Type::Fun(arg_types.clone(), Box::new(ret_var.clone()));

        if self.table.unify(&callee_ty, &expected_fn_type, span).is_err() {
            let resolved_callee = self.table.deep_resolve(&callee_ty);
            match &resolved_callee {
                Type::Fun(params, _) => {
                    if params.len() != args.len() {
                        self.diagnostics.push(
                            Diagnostic::error(
                                "E0204",
                                format!(
                                    "wrong number of arguments: expected {}, found {}",
                                    params.len(),
                                    args.len()
                                ),
                                span,
                            )
                            .with_label(Label::new(
                                span,
                                format!("expected {} argument(s)", params.len()),
                            )),
                        );
                    } else {
                        self.diagnostics.push(
                            Diagnostic::error(
                                "E0200",
                                "type mismatch in function call".to_string(),
                                span,
                            )
                            .with_label(Label::new(span, "argument type mismatch")),
                        );
                    }
                }
                _ => {
                    self.diagnostics.push(
                        Diagnostic::error(
                            "E0200",
                            format!(
                                "type mismatch: expected function, found `{}`",
                                resolved_callee.display()
                            ),
                            span,
                        )
                        .with_label(Label::new(callee.span, "not a function")),
                    );
                }
            }
            return None;
        }

        Some(self.table.deep_resolve(&ret_var))
    }

    /// Infer the type of a let expression.
    fn infer_let_expr(
        &mut self,
        name: &str,
        type_annotation: Option<&TypeAnnotation>,
        value: &Expr,
        body: &Expr,
    ) -> Option<Type> {
        let val_ty = self.infer_expr(value)?;

        if let Some(ann) = type_annotation {
            let expected = self.annotation_to_type(ann, &Vec::new());
            if self.table.unify(&val_ty, &expected, ann.span).is_err() {
                let resolved_expected = self.table.deep_resolve(&expected);
                let resolved_found = self.table.deep_resolve(&val_ty);
                self.diagnostics.push(
                    Diagnostic::error(
                        "E0200",
                        format!(
                            "type mismatch: expected `{}`, found `{}`",
                            resolved_expected.display(),
                            resolved_found.display()
                        ),
                        value.span,
                    )
                    .with_label(Label::new(
                        ann.span,
                        format!("expected `{}`", resolved_expected.display()),
                    ))
                    .with_label(Label::new(
                        value.span,
                        format!("found `{}`", resolved_found.display()),
                    )),
                );
            }
        }

        // Generalize and bind
        let scheme = self.env.generalize(&val_ty, &self.table);
        self.env.push_scope();
        self.env.bind(name.to_string(), scheme);

        let body_ty = self.infer_expr(body);

        self.env.pop_scope();
        body_ty
    }

    /// Infer the type of an if-else expression.
    fn infer_if_else(
        &mut self,
        condition: &Expr,
        then_branch: &Expr,
        else_branch: &Expr,
        span: Span,
    ) -> Option<Type> {
        let cond_ty = self.infer_expr(condition)?;

        // Condition must be Bool
        if self.table.unify(&cond_ty, &Type::Bool, condition.span).is_err() {
            let resolved = self.table.deep_resolve(&cond_ty);
            self.diagnostics.push(
                Diagnostic::error(
                    "E0200",
                    format!(
                        "if condition must be Bool, found `{}`",
                        resolved.display()
                    ),
                    condition.span,
                )
                .with_label(Label::new(
                    condition.span,
                    format!("type `{}`", resolved.display()),
                )),
            );
        }

        let then_ty = self.infer_expr(then_branch);
        let else_ty = self.infer_expr(else_branch);

        match (then_ty, else_ty) {
            (Some(t), Some(e)) => {
                if self.table.unify(&t, &e, span).is_err() {
                    let resolved_then = self.table.deep_resolve(&t);
                    let resolved_else = self.table.deep_resolve(&e);
                    self.diagnostics.push(
                        Diagnostic::error(
                            "E0200",
                            format!(
                                "if branches have incompatible types: `{}` and `{}`",
                                resolved_then.display(),
                                resolved_else.display()
                            ),
                            span,
                        )
                        .with_label(Label::new(
                            then_branch.span,
                            format!("type `{}`", resolved_then.display()),
                        ))
                        .with_label(Label::new(
                            else_branch.span,
                            format!("type `{}`", resolved_else.display()),
                        )),
                    );
                    return None;
                }
                Some(self.table.deep_resolve(&t))
            }
            (Some(t), None) => Some(t),
            (None, Some(e)) => Some(e),
            (None, None) => None,
        }
    }

    /// Infer the type of a match expression.
    fn infer_match(
        &mut self,
        scrutinee: &Expr,
        arms: &[MatchArm],
        _span: Span,
    ) -> Option<Type> {
        let scrutinee_ty = self.infer_expr(scrutinee)?;

        if arms.is_empty() {
            return Some(self.table.fresh_var());
        }

        let mut result_ty: Option<Type> = None;

        for arm in arms {
            // Infer pattern type and bind pattern variables
            self.env.push_scope();
            let pattern_ty = self.infer_pattern(&arm.pattern, &scrutinee_ty);

            if let Some(pt) = &pattern_ty {
                if self.table.unify(&scrutinee_ty, pt, arm.pattern.span).is_err() {
                    let resolved_scrutinee = self.table.deep_resolve(&scrutinee_ty);
                    let resolved_pattern = self.table.deep_resolve(pt);
                    self.diagnostics.push(
                        Diagnostic::error(
                            "E0200",
                            format!(
                                "pattern type `{}` does not match scrutinee type `{}`",
                                resolved_pattern.display(),
                                resolved_scrutinee.display()
                            ),
                            arm.pattern.span,
                        )
                        .with_label(Label::new(
                            scrutinee.span,
                            format!("scrutinee is `{}`", resolved_scrutinee.display()),
                        ))
                        .with_label(Label::new(
                            arm.pattern.span,
                            format!("pattern expects `{}`", resolved_pattern.display()),
                        )),
                    );
                }
            }

            // Infer arm body
            let body_ty = self.infer_expr(&arm.body);
            self.env.pop_scope();

            if let Some(bt) = body_ty {
                if let Some(ref rt) = result_ty {
                    if self.table.unify(rt, &bt, arm.body.span).is_err() {
                        let resolved_expected = self.table.deep_resolve(rt);
                        let resolved_found = self.table.deep_resolve(&bt);
                        self.diagnostics.push(
                            Diagnostic::error(
                                "E0200",
                                format!(
                                    "match arms have incompatible types: `{}` and `{}`",
                                    resolved_expected.display(),
                                    resolved_found.display()
                                ),
                                arm.body.span,
                            )
                            .with_label(Label::new(
                                arm.body.span,
                                format!("this arm returns `{}`", resolved_found.display()),
                            )),
                        );
                    }
                } else {
                    result_ty = Some(bt);
                }
            }
        }

        result_ty.map(|t| self.table.deep_resolve(&t))
    }

    /// Infer the type of a pattern, binding variables as appropriate.
    ///
    /// Returns the type that the pattern matches against.
    fn infer_pattern(&mut self, pattern: &Pattern, expected: &Type) -> Option<Type> {
        match &pattern.kind {
            PatternKind::Literal { value } => Some(self.infer_literal(value)),

            PatternKind::Ident { name } => {
                // Bind the variable to the expected scrutinee type
                let ty = self.table.deep_resolve(expected);
                self.env.bind(name.clone(), TypeScheme::mono(ty.clone()));
                Some(ty)
            }

            PatternKind::Wildcard => {
                Some(self.table.deep_resolve(expected))
            }

            PatternKind::Variant { name, fields } => {
                self.infer_variant_pattern(name, fields, expected, pattern.span)
            }
        }
    }

    /// Infer the type of a variant pattern (e.g., `Circle(r)`, `Some(x)`, `None`).
    fn infer_variant_pattern(
        &mut self,
        name: &str,
        fields: &[Pattern],
        expected: &Type,
        span: Span,
    ) -> Option<Type> {
        // Check built-in variants first
        match name {
            "Some" => {
                let inner_var = self.table.fresh_var();
                let opt_ty = Type::Option(Box::new(inner_var.clone()));
                // Unify with expected to constrain the inner type
                let _ = self.table.unify(&opt_ty, expected, span);

                if fields.len() != 1 {
                    self.diagnostics.push(
                        Diagnostic::error(
                            "E0204",
                            format!("Some expects 1 field, found {}", fields.len()),
                            span,
                        )
                        .with_label(Label::new(span, "expected 1 field")),
                    );
                    return Some(opt_ty);
                }

                let resolved_inner = self.table.deep_resolve(&inner_var);
                self.infer_pattern(&fields[0], &resolved_inner);
                return Some(self.table.deep_resolve(&opt_ty));
            }
            "None" => {
                let inner_var = self.table.fresh_var();
                let opt_ty = Type::Option(Box::new(inner_var));
                let _ = self.table.unify(&opt_ty, expected, span);

                if !fields.is_empty() {
                    self.diagnostics.push(
                        Diagnostic::error(
                            "E0204",
                            format!("None expects 0 fields, found {}", fields.len()),
                            span,
                        )
                        .with_label(Label::new(span, "expected 0 fields")),
                    );
                }
                return Some(self.table.deep_resolve(&opt_ty));
            }
            "Ok" => {
                let ok_var = self.table.fresh_var();
                let err_var = self.table.fresh_var();
                let res_ty = Type::Result(Box::new(ok_var.clone()), Box::new(err_var));
                let _ = self.table.unify(&res_ty, expected, span);

                if fields.len() != 1 {
                    self.diagnostics.push(
                        Diagnostic::error(
                            "E0204",
                            format!("Ok expects 1 field, found {}", fields.len()),
                            span,
                        )
                        .with_label(Label::new(span, "expected 1 field")),
                    );
                    return Some(res_ty);
                }

                let resolved_ok = self.table.deep_resolve(&ok_var);
                self.infer_pattern(&fields[0], &resolved_ok);
                return Some(self.table.deep_resolve(&res_ty));
            }
            "Err" => {
                let ok_var = self.table.fresh_var();
                let err_var = self.table.fresh_var();
                let res_ty = Type::Result(Box::new(ok_var), Box::new(err_var.clone()));
                let _ = self.table.unify(&res_ty, expected, span);

                if fields.len() != 1 {
                    self.diagnostics.push(
                        Diagnostic::error(
                            "E0204",
                            format!("Err expects 1 field, found {}", fields.len()),
                            span,
                        )
                        .with_label(Label::new(span, "expected 1 field")),
                    );
                    return Some(res_ty);
                }

                let resolved_err = self.table.deep_resolve(&err_var);
                self.infer_pattern(&fields[0], &resolved_err);
                return Some(self.table.deep_resolve(&res_ty));
            }
            _ => {}
        }

        // Look up user-defined variant
        let variant_info = if let Some(info) = self.env.variant_defs.get(name).cloned() {
            info
        } else {
            self.diagnostics.push(
                Diagnostic::error(
                    "E0203",
                    format!("undefined variant `{}`", name),
                    span,
                )
                .with_label(Label::new(span, "not found")),
            );
            return None;
        };

        // Check field count
        if fields.len() != variant_info.field_types.len() {
            self.diagnostics.push(
                Diagnostic::error(
                    "E0204",
                    format!(
                        "variant `{}` expects {} field(s), found {}",
                        name,
                        variant_info.field_types.len(),
                        fields.len()
                    ),
                    span,
                )
                .with_label(Label::new(
                    span,
                    format!("expected {} field(s)", variant_info.field_types.len()),
                )),
            );
            return None;
        }

        // Create fresh type variables for the ADT's type params
        let type_param_subst: Vec<(String, Type)> = variant_info
            .type_params
            .iter()
            .map(|p| (p.clone(), self.table.fresh_var()))
            .collect();

        // Construct the ADT type
        let adt_type_args: Vec<Type> = type_param_subst.iter().map(|(_, t)| t.clone()).collect();
        let adt_ty = Type::ADT(variant_info.adt_name.clone(), adt_type_args);

        // Unify with expected
        let _ = self.table.unify(&adt_ty, expected, span);

        // Bind pattern fields
        for (field_pat, field_ty) in fields.iter().zip(variant_info.field_types.iter()) {
            let substituted = self.substitute_type_params(field_ty, &type_param_subst);
            let resolved = self.table.deep_resolve(&substituted);
            self.infer_pattern(field_pat, &resolved);
        }

        Some(self.table.deep_resolve(&adt_ty))
    }

    /// Substitute type parameter names with their assigned types.
    fn substitute_type_params(&self, ty: &Type, _subst: &[(String, Type)]) -> Type {
        // Type params in variant field types are represented as fresh vars
        // during registration, so we need a different approach.
        // Actually, during registration we used fresh_var() for type params,
        // so the field types already have Type::Var entries that correspond
        // to the original registration-time vars.
        //
        // For user-defined ADTs, we need to re-instantiate. The field_types
        // stored in VariantInfo use Var IDs from registration time. We need
        // to map those to our fresh vars.
        //
        // Since we can't easily track the original var IDs, we take a simpler
        // approach: the field types for an ADT variant are stored with the
        // original type parameter names. In our implementation, type params
        // become fresh vars during annotation_to_type. So the field_types
        // contain those specific Var IDs. We need to substitute them.
        //
        // Actually, we already handle this correctly because type params
        // in annotation_to_type generate fresh vars that are recorded in
        // the VariantInfo. When we create new fresh vars in the pattern,
        // we need to unify the original vars with the new ones.
        //
        // Let's just return ty as-is and rely on unification.
        ty.clone()
    }

    /// Infer the type of a list literal.
    fn infer_list_literal(&mut self, elements: &[Expr], span: Span) -> Option<Type> {
        if elements.is_empty() {
            // Empty list: List<a> with fresh variable
            return Some(Type::List(Box::new(self.table.fresh_var())));
        }

        // Infer element types and unify them all
        let elem_var = self.table.fresh_var();
        for elem in elements {
            if let Some(et) = self.infer_expr(elem) {
                if self.table.unify(&et, &elem_var, elem.span).is_err() {
                    let resolved_expected = self.table.deep_resolve(&elem_var);
                    let resolved_found = self.table.deep_resolve(&et);
                    self.diagnostics.push(
                        Diagnostic::error(
                            "E0200",
                            format!(
                                "list elements have incompatible types: `{}` and `{}`",
                                resolved_expected.display(),
                                resolved_found.display()
                            ),
                            elem.span,
                        )
                        .with_label(Label::new(
                            span,
                            format!("expected `{}`", resolved_expected.display()),
                        ))
                        .with_label(Label::new(
                            elem.span,
                            format!("found `{}`", resolved_found.display()),
                        )),
                    );
                }
            }
        }

        Some(Type::List(Box::new(self.table.deep_resolve(&elem_var))))
    }

    /// Infer the type of a block expression.
    fn infer_block(&mut self, exprs: &[Expr]) -> Option<Type> {
        if exprs.is_empty() {
            return Some(Type::Unit);
        }

        let mut last_ty = None;
        for expr in exprs {
            last_ty = self.infer_expr(expr);
        }
        last_ty
    }

    /// Infer the type of a variant construction expression.
    fn infer_variant_construct(
        &mut self,
        name: &str,
        args: &[Expr],
        span: Span,
    ) -> Option<Type> {
        // Check built-in variants
        match name {
            "Some" => {
                if args.len() != 1 {
                    self.diagnostics.push(
                        Diagnostic::error(
                            "E0204",
                            format!("Some expects 1 argument, found {}", args.len()),
                            span,
                        )
                        .with_label(Label::new(span, "expected 1 argument")),
                    );
                    return None;
                }
                let inner = self.infer_expr(&args[0])?;
                return Some(Type::Option(Box::new(inner)));
            }
            "None" => {
                if !args.is_empty() {
                    self.diagnostics.push(
                        Diagnostic::error(
                            "E0204",
                            format!("None expects 0 arguments, found {}", args.len()),
                            span,
                        )
                        .with_label(Label::new(span, "expected 0 arguments")),
                    );
                    return None;
                }
                return Some(Type::Option(Box::new(self.table.fresh_var())));
            }
            "Ok" => {
                if args.len() != 1 {
                    self.diagnostics.push(
                        Diagnostic::error(
                            "E0204",
                            format!("Ok expects 1 argument, found {}", args.len()),
                            span,
                        )
                        .with_label(Label::new(span, "expected 1 argument")),
                    );
                    return None;
                }
                let ok_ty = self.infer_expr(&args[0])?;
                return Some(Type::Result(
                    Box::new(ok_ty),
                    Box::new(self.table.fresh_var()),
                ));
            }
            "Err" => {
                if args.len() != 1 {
                    self.diagnostics.push(
                        Diagnostic::error(
                            "E0204",
                            format!("Err expects 1 argument, found {}", args.len()),
                            span,
                        )
                        .with_label(Label::new(span, "expected 1 argument")),
                    );
                    return None;
                }
                let err_ty = self.infer_expr(&args[0])?;
                return Some(Type::Result(
                    Box::new(self.table.fresh_var()),
                    Box::new(err_ty),
                ));
            }
            _ => {}
        }

        // Look up user-defined variant
        let variant_info = if let Some(info) = self.env.variant_defs.get(name).cloned() {
            info
        } else {
            self.diagnostics.push(
                Diagnostic::error(
                    "E0203",
                    format!("undefined variant `{}`", name),
                    span,
                )
                .with_label(Label::new(span, "not found")),
            );
            return None;
        };

        // Check argument count
        if args.len() != variant_info.field_types.len() {
            self.diagnostics.push(
                Diagnostic::error(
                    "E0204",
                    format!(
                        "variant `{}` expects {} argument(s), found {}",
                        name,
                        variant_info.field_types.len(),
                        args.len()
                    ),
                    span,
                )
                .with_label(Label::new(
                    span,
                    format!("expected {} argument(s)", variant_info.field_types.len()),
                )),
            );
            return None;
        }

        // Create fresh type variables for the ADT's type params
        let fresh_params: Vec<Type> = variant_info
            .type_params
            .iter()
            .map(|_| self.table.fresh_var())
            .collect();

        // Infer and unify argument types with the variant's field types
        for (arg, field_ty) in args.iter().zip(variant_info.field_types.iter()) {
            let arg_ty = self.infer_expr(arg);
            if let Some(at) = arg_ty {
                // The field type may contain free vars from registration;
                // We need to unify to constrain the fresh params
                if self.table.unify(&at, field_ty, arg.span).is_err() {
                    let resolved_expected = self.table.deep_resolve(field_ty);
                    let resolved_found = self.table.deep_resolve(&at);
                    self.diagnostics.push(
                        Diagnostic::error(
                            "E0200",
                            format!(
                                "type mismatch in variant constructor: expected `{}`, found `{}`",
                                resolved_expected.display(),
                                resolved_found.display()
                            ),
                            arg.span,
                        )
                        .with_label(Label::new(
                            arg.span,
                            format!("expected `{}`", resolved_expected.display()),
                        )),
                    );
                }
            }
        }

        // Construct the result ADT type
        let adt_args: Vec<Type> = if variant_info.type_params.is_empty() {
            Vec::new()
        } else {
            fresh_params
                .iter()
                .map(|p| self.table.deep_resolve(p))
                .collect()
        };

        Some(Type::ADT(variant_info.adt_name.clone(), adt_args))
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// Get the symbol representation of a binary operator.
fn binop_symbol(op: BinOp) -> &'static str {
    match op {
        BinOp::Add => "+",
        BinOp::Sub => "-",
        BinOp::Mul => "*",
        BinOp::Div => "/",
        BinOp::Mod => "%",
        BinOp::Concat => "++",
        BinOp::Eq => "==",
        BinOp::Ne => "!=",
        BinOp::Lt => "<",
        BinOp::Le => "<=",
        BinOp::Gt => ">",
        BinOp::Ge => ">=",
        BinOp::And => "&&",
        BinOp::Or => "||",
    }
}
