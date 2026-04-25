//! Runtime values for the LM interpreter.

use crate::env::Env;
use lm_parser::ast::Expr;
use std::fmt;

/// A runtime value produced by evaluating an LM expression.
#[derive(Debug, Clone)]
pub enum Value {
    /// A 64-bit integer.
    Int(i64),
    /// A 64-bit floating-point number.
    Float(f64),
    /// A boolean value.
    Bool(bool),
    /// A string value.
    String(String),
    /// The unit value (returned by IO operations like print).
    Unit,
    /// A list of values.
    List(Vec<Value>),
    /// A closure: a function with captured environment.
    Closure {
        /// Parameter names.
        params: Vec<String>,
        /// The function body expression.
        body: Box<Expr>,
        /// The captured environment.
        env: Box<Env>,
    },
    /// An algebraic data type instance.
    ADTInstance {
        /// The variant name (e.g., "Circle", "Rect").
        variant: String,
        /// The field values.
        fields: Vec<Value>,
    },
    /// An Option value: None or Some(v).
    Option(Option<Box<Value>>),
    /// A Result value: Ok(v) or Err(e).
    Result(std::result::Result<Box<Value>, Box<Value>>),
    /// A reference to a built-in function by name.
    BuiltinFn(String),
    /// An ADT constructor function (variant with arity > 0, used as a first-class value).
    Constructor {
        /// The variant name.
        variant: String,
        /// Number of fields expected.
        arity: usize,
    },
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Unit, Value::Unit) => true,
            (Value::List(a), Value::List(b)) => a == b,
            (
                Value::ADTInstance {
                    variant: v1,
                    fields: f1,
                },
                Value::ADTInstance {
                    variant: v2,
                    fields: f2,
                },
            ) => v1 == v2 && f1 == f2,
            (Value::Option(a), Value::Option(b)) => a == b,
            (Value::Result(a), Value::Result(b)) => a == b,
            _ => false,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{n}"),
            Value::Float(v) => {
                // Format float: show decimal point even for whole numbers
                if v.fract() == 0.0 && v.is_finite() {
                    write!(f, "{v:.1}")
                } else {
                    write!(f, "{v}")
                }
            }
            Value::Bool(b) => write!(f, "{b}"),
            Value::String(s) => write!(f, "{s}"),
            Value::Unit => write!(f, "()"),
            Value::List(items) => {
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{item}")?;
                }
                write!(f, "]")
            }
            Value::Closure { .. } => write!(f, "<function>"),
            Value::ADTInstance { variant, fields } => {
                write!(f, "{variant}")?;
                if !fields.is_empty() {
                    write!(f, "(")?;
                    for (i, field) in fields.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{field}")?;
                    }
                    write!(f, ")")?;
                }
                Ok(())
            }
            Value::Option(None) => write!(f, "None"),
            Value::Option(Some(v)) => write!(f, "Some({v})"),
            Value::Result(Ok(v)) => write!(f, "Ok({v})"),
            Value::Result(Err(e)) => write!(f, "Err({e})"),
            Value::BuiltinFn(name) => write!(f, "<builtin:{name}>"),
            Value::Constructor { variant, .. } => write!(f, "<constructor:{variant}>"),
        }
    }
}
