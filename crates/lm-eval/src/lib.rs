//! Tree-walking interpreter for the LM programming language.
//!
//! This crate implements an environment-based interpreter that evaluates
//! type-checked LM programs. The pipeline is: lex -> parse -> type check -> **evaluate**.
//!
//! # Error code ranges
//!
//! | Range       | Category       |
//! |-------------|----------------|
//! | E0500       | Division by zero |
//! | E0501       | Index out of bounds |
//! | E0502       | String to int conversion failed |
//! | E0503       | Pattern match failure |
//!
//! # Usage
//!
//! ```no_run
//! use lm_eval::Interpreter;
//! use lm_parser::ast::Program;
//!
//! // After parsing + type checking a program:
//! // let mut interp = Interpreter::new();
//! // let result = interp.eval_program(&program);
//! ```

mod env;
mod value;
mod builtins;

#[cfg(test)]
mod tests;

pub use env::Env;
pub use value::Value;

use lm_diagnostics::{Diagnostic, Span};
use lm_parser::ast::*;
use std::io::Write;

/// Runtime error produced during evaluation.
#[derive(Debug, Clone)]
pub struct RuntimeError {
    /// The diagnostic describing the error.
    pub diagnostic: Diagnostic,
}

impl RuntimeError {
    /// Create a new runtime error with code, message, and source span.
    pub fn new(code: &str, message: impl Into<String>, span: Span) -> Self {
        Self {
            diagnostic: Diagnostic::error(code, message, span),
        }
    }
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.diagnostic.code, self.diagnostic.message)
    }
}

impl std::error::Error for RuntimeError {}

/// Convenience: create a boxed runtime error.
pub(crate) fn runtime_err(code: &str, message: impl Into<String>, span: Span) -> Box<RuntimeError> {
    Box::new(RuntimeError::new(code, message, span))
}

/// Result type for interpreter operations.
///
/// Uses `Box<RuntimeError>` to keep the `Result` size small (RuntimeError contains
/// a Diagnostic which is large).
pub type EvalResult = Result<Value, Box<RuntimeError>>;

/// A tree-walking interpreter for LM programs.
///
/// Evaluates a type-checked AST, maintaining an environment of bindings.
/// IO operations (print, read_line) are performed via configurable writers/readers.
pub struct Interpreter {
    /// The global environment.
    env: Env,
    /// Output writer for print() calls.
    output: Box<dyn Write>,
    /// Input reader for read_line() calls.
    input: Box<dyn std::io::BufRead>,
}

impl Interpreter {
    /// Create a new interpreter with stdout/stdin.
    pub fn new() -> Self {
        let env = Env::new();
        Self {
            env,
            output: Box::new(std::io::stdout()),
            input: Box::new(std::io::BufReader::new(std::io::stdin())),
        }
    }

    /// Create an interpreter that captures output to a buffer (for testing).
    pub fn with_test_io(input: &str) -> (Self, std::sync::Arc<std::sync::Mutex<Vec<u8>>>) {
        let env = Env::new();
        let output_buf = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let writer = SharedWriter(output_buf.clone());
        let interp = Self {
            env,
            output: Box::new(writer),
            input: Box::new(std::io::Cursor::new(input.to_string())),
        };
        (interp, output_buf)
    }

    /// Evaluate a complete program. Returns the value of the last top-level declaration.
    pub fn eval_program(&mut self, program: &Program) -> EvalResult {
        let mut last_value = Value::Unit;

        // First pass: register all type definitions (ADT constructors)
        for decl in &program.decls {
            if let DeclKind::TypeDef { variants, .. } = &decl.kind {
                for variant in variants {
                    let name = variant.name.clone();
                    let arity = variant.fields.len();
                    if arity == 0 {
                        // Unit variant — bind as a value directly
                        self.env.define(
                            name.clone(),
                            Value::ADTInstance {
                                variant: name,
                                fields: vec![],
                            },
                        );
                    } else {
                        // Variant with fields — bind as a constructor function
                        self.env.define(
                            name.clone(),
                            Value::Constructor {
                                variant: name,
                                arity,
                            },
                        );
                    }
                }
            }
        }

        // Second pass: register all functions (so they can reference each other)
        for decl in &program.decls {
            if let DeclKind::FnDef {
                name,
                params,
                body,
                ..
            } = &decl.kind
            {
                let param_names: Vec<String> =
                    params.iter().map(|p| p.name.clone()).collect();
                // Create a closure that captures the current env.
                // We'll fix up the env after all functions are registered.
                let closure = Value::Closure {
                    params: param_names,
                    body: Box::new(body.clone()),
                    env: Box::new(self.env.clone()),
                };
                self.env.define(name.clone(), closure);
            }
        }

        // Fix up closures so they capture the environment that includes all functions.
        // This enables mutual recursion.
        let full_env = self.env.clone();
        for decl in &program.decls {
            if let DeclKind::FnDef { name, .. } = &decl.kind {
                if let Some(Value::Closure { params, body, .. }) = self.env.get(name) {
                    let fixed = Value::Closure {
                        params,
                        body,
                        env: Box::new(full_env.clone()),
                    };
                    self.env.define(name.clone(), fixed);
                }
            }
        }

        // Third pass: evaluate let definitions in order
        for decl in &program.decls {
            if let DeclKind::LetDef { name, value, .. } = &decl.kind {
                let val = self.eval_expr(value)?;
                self.env.define(name.clone(), val.clone());
                last_value = val;
            }
        }

        Ok(last_value)
    }

    /// Evaluate an expression in the current environment.
    pub fn eval_expr(&mut self, expr: &Expr) -> EvalResult {
        match &expr.kind {
            ExprKind::Literal { value } => Ok(self.eval_literal(value)),

            ExprKind::Ident { name } => {
                // Check builtins first, then environment
                if let Some(val) = self.env.get(name) {
                    Ok(val)
                } else if builtins::is_builtin(name) {
                    Ok(Value::BuiltinFn(name.clone()))
                } else {
                    // Should not happen after type checking, but be defensive
                    Err(runtime_err(
                        "E0503",
                        format!("undefined variable `{name}`"),
                        expr.span,
                    ))
                }
            }

            ExprKind::BinaryOp { op, lhs, rhs } => {
                self.eval_binary_op(*op, lhs, rhs, expr.span)
            }

            ExprKind::UnaryOp { op, operand } => self.eval_unary_op(*op, operand, expr.span),

            ExprKind::FnCall { callee, args } => self.eval_fn_call(callee, args, expr.span),

            ExprKind::LetExpr {
                name,
                value,
                body,
                ..
            } => {
                let val = self.eval_expr(value)?;
                let saved_env = self.env.clone();
                self.env.define(name.clone(), val);
                let result = self.eval_expr(body);
                self.env = saved_env;
                result
            }

            ExprKind::IfElse {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond = self.eval_expr(condition)?;
                match cond {
                    Value::Bool(true) => self.eval_expr(then_branch),
                    Value::Bool(false) => self.eval_expr(else_branch),
                    _ => Err(runtime_err(
                        "E0503",
                        "if condition must be a boolean",
                        condition.span,
                    )),
                }
            }

            ExprKind::Match { scrutinee, arms } => {
                let val = self.eval_expr(scrutinee)?;
                self.eval_match(&val, arms, expr.span)
            }

            ExprKind::Block { exprs } => {
                if exprs.is_empty() {
                    return Ok(Value::Unit);
                }
                let mut result = Value::Unit;
                for e in exprs {
                    result = self.eval_expr(e)?;
                }
                Ok(result)
            }

            ExprKind::ListLiteral { elements } => {
                let mut items = Vec::with_capacity(elements.len());
                for elem in elements {
                    items.push(self.eval_expr(elem)?);
                }
                Ok(Value::List(items))
            }

            ExprKind::VariantConstruct { name, args } => {
                // Check if this is a known constructor in the environment
                if let Some(Value::Constructor { variant, arity }) = self.env.get(name) {
                    if args.len() != arity {
                        return Err(runtime_err(
                            "E0503",
                            format!(
                                "variant `{variant}` expects {arity} fields, got {}",
                                args.len()
                            ),
                            expr.span,
                        ));
                    }
                    let mut field_vals = Vec::with_capacity(arity);
                    for arg in args {
                        field_vals.push(self.eval_expr(arg)?);
                    }
                    Ok(Value::ADTInstance {
                        variant,
                        fields: field_vals,
                    })
                } else if let Some(val) = self.env.get(name) {
                    // Unit variant already stored as ADTInstance
                    if args.is_empty() {
                        Ok(val)
                    } else {
                        Err(runtime_err(
                            "E0503",
                            format!("cannot call `{name}` with arguments"),
                            expr.span,
                        ))
                    }
                } else {
                    // Handle built-in constructors: Some, None, Ok, Err
                    match name.as_str() {
                        "Some" => {
                            if args.len() != 1 {
                                return Err(runtime_err(
                                    "E0503",
                                    "Some expects exactly 1 argument",
                                    expr.span,
                                ));
                            }
                            let val = self.eval_expr(&args[0])?;
                            Ok(Value::Option(Some(Box::new(val))))
                        }
                        "None" => Ok(Value::Option(None)),
                        "Ok" => {
                            if args.len() != 1 {
                                return Err(runtime_err(
                                    "E0503",
                                    "Ok expects exactly 1 argument",
                                    expr.span,
                                ));
                            }
                            let val = self.eval_expr(&args[0])?;
                            Ok(Value::Result(Ok(Box::new(val))))
                        }
                        "Err" => {
                            if args.len() != 1 {
                                return Err(runtime_err(
                                    "E0503",
                                    "Err expects exactly 1 argument",
                                    expr.span,
                                ));
                            }
                            let val = self.eval_expr(&args[0])?;
                            Ok(Value::Result(Err(Box::new(val))))
                        }
                        _ => Err(runtime_err(
                            "E0503",
                            format!("unknown variant `{name}`"),
                            expr.span,
                        )),
                    }
                }
            }

            ExprKind::Error => Err(runtime_err(
                "E0503",
                "cannot evaluate error node",
                expr.span,
            )),
        }
    }

    /// Evaluate a literal.
    fn eval_literal(&self, lit: &LitValue) -> Value {
        match lit {
            LitValue::Int(n) => Value::Int(*n),
            LitValue::Float(f) => Value::Float(*f),
            LitValue::String(s) => Value::String(s.clone()),
            LitValue::Bool(b) => Value::Bool(*b),
        }
    }

    /// Evaluate a binary operation.
    fn eval_binary_op(
        &mut self,
        op: BinOp,
        lhs: &Expr,
        rhs: &Expr,
        span: Span,
    ) -> EvalResult {
        // Short-circuit for && and ||
        if op == BinOp::And {
            let l = self.eval_expr(lhs)?;
            return match l {
                Value::Bool(false) => Ok(Value::Bool(false)),
                Value::Bool(true) => self.eval_expr(rhs),
                _ => Err(runtime_err("E0503", "expected boolean", lhs.span)),
            };
        }
        if op == BinOp::Or {
            let l = self.eval_expr(lhs)?;
            return match l {
                Value::Bool(true) => Ok(Value::Bool(true)),
                Value::Bool(false) => self.eval_expr(rhs),
                _ => Err(runtime_err("E0503", "expected boolean", lhs.span)),
            };
        }

        let l = self.eval_expr(lhs)?;
        let r = self.eval_expr(rhs)?;

        match op {
            BinOp::Add => match (&l, &r) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
                _ => Err(runtime_err("E0503", "type error in addition", span)),
            },
            BinOp::Sub => match (&l, &r) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
                _ => Err(runtime_err("E0503", "type error in subtraction", span)),
            },
            BinOp::Mul => match (&l, &r) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
                _ => Err(runtime_err(
                    "E0503",
                    "type error in multiplication",
                    span,
                )),
            },
            BinOp::Div => match (&l, &r) {
                (Value::Int(_, ), Value::Int(0)) => Err(runtime_err(
                    "E0500",
                    "division by zero",
                    span,
                )),
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a / b)),
                (Value::Float(_), Value::Float(b)) if *b == 0.0 => Err(runtime_err(
                    "E0500",
                    "division by zero",
                    span,
                )),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
                _ => Err(runtime_err("E0503", "type error in division", span)),
            },
            BinOp::Mod => match (&l, &r) {
                (Value::Int(_), Value::Int(0)) => Err(runtime_err(
                    "E0500",
                    "division by zero",
                    span,
                )),
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a % b)),
                (Value::Float(_), Value::Float(b)) if *b == 0.0 => Err(runtime_err(
                    "E0500",
                    "division by zero",
                    span,
                )),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a % b)),
                _ => Err(runtime_err("E0503", "type error in modulo", span)),
            },
            BinOp::Concat => match (&l, &r) {
                (Value::String(a), Value::String(b)) => {
                    Ok(Value::String(format!("{a}{b}")))
                }
                _ => Err(runtime_err(
                    "E0503",
                    "type error in string concatenation",
                    span,
                )),
            },
            BinOp::Eq => Ok(Value::Bool(l == r)),
            BinOp::Ne => Ok(Value::Bool(l != r)),
            BinOp::Lt => self.eval_comparison(&l, &r, |a, b| a < b, |a, b| a < b, span),
            BinOp::Le => self.eval_comparison(&l, &r, |a, b| a <= b, |a, b| a <= b, span),
            BinOp::Gt => self.eval_comparison(&l, &r, |a, b| a > b, |a, b| a > b, span),
            BinOp::Ge => self.eval_comparison(&l, &r, |a, b| a >= b, |a, b| a >= b, span),
            BinOp::And | BinOp::Or => unreachable!("handled above"),
        }
    }

    /// Helper for comparison operators.
    fn eval_comparison(
        &self,
        l: &Value,
        r: &Value,
        int_cmp: impl FnOnce(i64, i64) -> bool,
        float_cmp: impl FnOnce(f64, f64) -> bool,
        span: Span,
    ) -> EvalResult {
        match (l, r) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(int_cmp(*a, *b))),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(float_cmp(*a, *b))),
            (Value::String(a), Value::String(b)) => Ok(Value::Bool(int_cmp(a.cmp(b) as i64, 0))),
            _ => Err(runtime_err(
                "E0503",
                "type error in comparison",
                span,
            )),
        }
    }

    /// Evaluate a unary operation.
    fn eval_unary_op(&mut self, op: UnOp, operand: &Expr, span: Span) -> EvalResult {
        let val = self.eval_expr(operand)?;
        match op {
            UnOp::Not => match val {
                Value::Bool(b) => Ok(Value::Bool(!b)),
                _ => Err(runtime_err("E0503", "expected boolean for `!`", span)),
            },
            UnOp::Neg => match val {
                Value::Int(n) => Ok(Value::Int(-n)),
                Value::Float(f) => Ok(Value::Float(-f)),
                _ => Err(runtime_err(
                    "E0503",
                    "expected number for negation",
                    span,
                )),
            },
        }
    }

    /// Evaluate a function call.
    fn eval_fn_call(&mut self, callee: &Expr, args: &[Expr], span: Span) -> EvalResult {
        // Remember the callee name for recursion support
        let callee_name = if let ExprKind::Ident { name } = &callee.kind {
            Some(name.clone())
        } else {
            None
        };

        let callee_val = self.eval_expr(callee)?;

        // Evaluate arguments
        let mut arg_vals = Vec::with_capacity(args.len());
        for arg in args {
            arg_vals.push(self.eval_expr(arg)?);
        }

        match callee_val {
            Value::Closure { params, body, env } => {
                if params.len() != arg_vals.len() {
                    return Err(runtime_err(
                        "E0503",
                        format!(
                            "function expects {} arguments, got {}",
                            params.len(),
                            arg_vals.len()
                        ),
                        span,
                    ));
                }
                // Move the current environment out instead of cloning it. Recursive calls can
                // otherwise spend most of their stack cloning nested closure environments.
                let saved_env = std::mem::replace(&mut self.env, (*env).clone());

                // Enable recursion: inject the function itself into the closure env
                // under its own name so it can call itself.
                if let Some(ref name) = callee_name {
                    self.env.define(
                        name.clone(),
                        Value::Closure {
                            params: params.clone(),
                            body: body.clone(),
                            env,
                        },
                    );
                }

                for (param, val) in params.iter().zip(arg_vals) {
                    self.env.define(param.clone(), val);
                }
                let result = self.eval_expr(&body);
                self.env = saved_env;
                result
            }
            Value::BuiltinFn(name) => self.call_builtin(&name, arg_vals, span),
            Value::Constructor { variant, arity } => {
                if arity != arg_vals.len() {
                    return Err(runtime_err(
                        "E0503",
                        format!(
                            "variant `{variant}` expects {arity} fields, got {}",
                            arg_vals.len()
                        ),
                        span,
                    ));
                }
                Ok(Value::ADTInstance {
                    variant,
                    fields: arg_vals,
                })
            }
            _ => Err(runtime_err(
                "E0503",
                "cannot call a non-function value",
                span,
            )),
        }
    }

    /// Call a built-in function.
    fn call_builtin(
        &mut self,
        name: &str,
        args: Vec<Value>,
        span: Span,
    ) -> EvalResult {
        builtins::call_builtin(self, name, args, span)
    }

    /// Evaluate a match expression.
    fn eval_match(&mut self, scrutinee: &Value, arms: &[MatchArm], span: Span) -> EvalResult {
        for arm in arms {
            let mut bindings = Vec::new();
            if self.pattern_matches(&arm.pattern, scrutinee, &mut bindings) {
                let saved_env = self.env.clone();
                for (name, val) in bindings {
                    self.env.define(name, val);
                }
                let result = self.eval_expr(&arm.body);
                self.env = saved_env;
                return result;
            }
        }
        Err(runtime_err(
            "E0503",
            "non-exhaustive pattern match",
            span,
        ))
    }

    /// Check if a pattern matches a value, collecting bindings.
    fn pattern_matches(
        &self,
        pattern: &Pattern,
        value: &Value,
        bindings: &mut Vec<(String, Value)>,
    ) -> bool {
        match &pattern.kind {
            PatternKind::Wildcard => true,

            PatternKind::Ident { name } => {
                bindings.push((name.clone(), value.clone()));
                true
            }

            PatternKind::Literal { value: lit } => {
                let lit_val = self.eval_literal(lit);
                &lit_val == value
            }

            PatternKind::Variant { name, fields } => {
                // Match against ADTInstance
                if let Value::ADTInstance {
                    variant,
                    fields: field_vals,
                } = value
                {
                    if name != variant || fields.len() != field_vals.len() {
                        return false;
                    }
                    for (pat, val) in fields.iter().zip(field_vals.iter()) {
                        if !self.pattern_matches(pat, val, bindings) {
                            return false;
                        }
                    }
                    return true;
                }
                // Match Option: Some(x)
                if name == "Some" {
                    if let Value::Option(Some(inner)) = value {
                        if fields.len() == 1 {
                            return self.pattern_matches(&fields[0], inner, bindings);
                        }
                    }
                    return false;
                }
                // Match Option: None
                if name == "None" {
                    if let Value::Option(None) = value {
                        return fields.is_empty();
                    }
                    return false;
                }
                // Match Result: Ok(x)
                if name == "Ok" {
                    if let Value::Result(Ok(inner)) = value {
                        if fields.len() == 1 {
                            return self.pattern_matches(&fields[0], inner, bindings);
                        }
                    }
                    return false;
                }
                // Match Result: Err(e)
                if name == "Err" {
                    if let Value::Result(Err(inner)) = value {
                        if fields.len() == 1 {
                            return self.pattern_matches(&fields[0], inner, bindings);
                        }
                    }
                    return false;
                }
                false
            }
        }
    }

    /// Write output (used by print builtin).
    pub(crate) fn write_output(&mut self, s: &str) {
        let _ = writeln!(self.output, "{s}");
    }

    /// Read a line of input (used by read_line builtin).
    pub(crate) fn read_input_line(&mut self) -> String {
        let mut buf = String::new();
        let _ = self.input.read_line(&mut buf);
        // Trim trailing newline
        if buf.ends_with('\n') {
            buf.pop();
            if buf.ends_with('\r') {
                buf.pop();
            }
        }
        buf
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

/// A shared writer that wraps an `Arc<Mutex<Vec<u8>>>` for test output capture.
struct SharedWriter(std::sync::Arc<std::sync::Mutex<Vec<u8>>>);

impl Write for SharedWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut inner = self.0.lock().unwrap();
        inner.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
