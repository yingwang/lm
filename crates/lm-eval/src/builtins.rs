//! Built-in functions for the LM interpreter.
//!
//! These are functions available in every LM program without explicit import.

use crate::value::Value;
use crate::{runtime_err, EvalResult, Interpreter};
use lm_diagnostics::Span;

/// The set of built-in function names.
const BUILTINS: &[&str] = &[
    "print",
    "read_line",
    "int_to_string",
    "float_to_string",
    "string_to_int",
    "len",
    "str_len",
    "list_get",
    "list_push",
    "list_map",
    "to_string",
];

/// Check if a name is a built-in function.
pub fn is_builtin(name: &str) -> bool {
    BUILTINS.contains(&name)
}

/// Call a built-in function with the given arguments.
pub fn call_builtin(
    interp: &mut Interpreter,
    name: &str,
    args: Vec<Value>,
    span: Span,
) -> EvalResult {
    match name {
        "print" => builtin_print(interp, args, span),
        "read_line" => builtin_read_line(interp, args, span),
        "int_to_string" => builtin_int_to_string(args, span),
        "float_to_string" => builtin_float_to_string(args, span),
        "string_to_int" => builtin_string_to_int(args, span),
        "len" => builtin_len(args, span),
        "str_len" => builtin_str_len(args, span),
        "list_get" => builtin_list_get(args, span),
        "list_push" => builtin_list_push(args, span),
        "list_map" => builtin_list_map(interp, args, span),
        "to_string" => builtin_to_string(args, span),
        _ => Err(runtime_err(
            "E0503",
            format!("unknown built-in function `{name}`"),
            span,
        )),
    }
}

/// `print(value) -> Unit` — print to stdout with newline.
fn builtin_print(interp: &mut Interpreter, args: Vec<Value>, span: Span) -> EvalResult {
    if args.len() != 1 {
        return Err(runtime_err(
            "E0503",
            format!("print expects 1 argument, got {}", args.len()),
            span,
        ));
    }
    let s = format!("{}", args[0]);
    interp.write_output(&s);
    Ok(Value::Unit)
}

/// `read_line() -> String` — read a line from stdin.
fn builtin_read_line(interp: &mut Interpreter, args: Vec<Value>, span: Span) -> EvalResult {
    if !args.is_empty() {
        return Err(runtime_err(
            "E0503",
            format!("read_line expects 0 arguments, got {}", args.len()),
            span,
        ));
    }
    let line = interp.read_input_line();
    Ok(Value::String(line))
}

/// `int_to_string(n: Int) -> String`.
fn builtin_int_to_string(args: Vec<Value>, span: Span) -> EvalResult {
    if args.len() != 1 {
        return Err(runtime_err(
            "E0503",
            format!("int_to_string expects 1 argument, got {}", args.len()),
            span,
        ));
    }
    match &args[0] {
        Value::Int(n) => Ok(Value::String(n.to_string())),
        _ => Err(runtime_err(
            "E0503",
            "int_to_string expects an Int",
            span,
        )),
    }
}

/// `float_to_string(f: Float) -> String`.
fn builtin_float_to_string(args: Vec<Value>, span: Span) -> EvalResult {
    if args.len() != 1 {
        return Err(runtime_err(
            "E0503",
            format!("float_to_string expects 1 argument, got {}", args.len()),
            span,
        ));
    }
    match &args[0] {
        Value::Float(f) => Ok(Value::String(format!("{f}"))),
        _ => Err(runtime_err(
            "E0503",
            "float_to_string expects a Float",
            span,
        )),
    }
}

/// `string_to_int(s: String) -> Result<Int, String>`.
fn builtin_string_to_int(args: Vec<Value>, span: Span) -> EvalResult {
    if args.len() != 1 {
        return Err(runtime_err(
            "E0503",
            format!("string_to_int expects 1 argument, got {}", args.len()),
            span,
        ));
    }
    match &args[0] {
        Value::String(s) => match s.parse::<i64>() {
            Ok(n) => Ok(Value::Result(Ok(Box::new(Value::Int(n))))),
            Err(e) => Ok(Value::Result(Err(Box::new(Value::String(format!(
                "cannot parse `{s}` as integer: {e}"
            )))))),
        },
        _ => Err(runtime_err(
            "E0502",
            "string_to_int expects a String",
            span,
        )),
    }
}

/// `len(list: List<T>) -> Int`.
fn builtin_len(args: Vec<Value>, span: Span) -> EvalResult {
    if args.len() != 1 {
        return Err(runtime_err(
            "E0503",
            format!("len expects 1 argument, got {}", args.len()),
            span,
        ));
    }
    match &args[0] {
        Value::List(items) => Ok(Value::Int(items.len() as i64)),
        Value::String(s) => Ok(Value::Int(s.len() as i64)),
        _ => Err(runtime_err(
            "E0503",
            "len expects a List or String",
            span,
        )),
    }
}

/// `str_len(s: String) -> Int`.
fn builtin_str_len(args: Vec<Value>, span: Span) -> EvalResult {
    if args.len() != 1 {
        return Err(runtime_err(
            "E0503",
            format!("str_len expects 1 argument, got {}", args.len()),
            span,
        ));
    }
    match &args[0] {
        Value::String(s) => Ok(Value::Int(s.len() as i64)),
        _ => Err(runtime_err(
            "E0503",
            "str_len expects a String",
            span,
        )),
    }
}

/// `list_get(list: List<T>, index: Int) -> Option<T>`.
fn builtin_list_get(args: Vec<Value>, span: Span) -> EvalResult {
    if args.len() != 2 {
        return Err(runtime_err(
            "E0503",
            format!("list_get expects 2 arguments, got {}", args.len()),
            span,
        ));
    }
    match (&args[0], &args[1]) {
        (Value::List(items), Value::Int(idx)) => {
            let idx = *idx;
            if idx < 0 || idx as usize >= items.len() {
                Ok(Value::Option(None))
            } else {
                Ok(Value::Option(Some(Box::new(items[idx as usize].clone()))))
            }
        }
        _ => Err(runtime_err(
            "E0503",
            "list_get expects (List, Int)",
            span,
        )),
    }
}

/// `list_push(list: List<T>, item: T) -> List<T>` — returns new list with item appended.
fn builtin_list_push(args: Vec<Value>, span: Span) -> EvalResult {
    if args.len() != 2 {
        return Err(runtime_err(
            "E0503",
            format!("list_push expects 2 arguments, got {}", args.len()),
            span,
        ));
    }
    let mut args_iter = args.into_iter();
    let first = args_iter.next().unwrap();
    let second = args_iter.next().unwrap();
    match first {
        Value::List(mut items) => {
            items.push(second);
            Ok(Value::List(items))
        }
        _ => Err(runtime_err(
            "E0503",
            "list_push expects (List, T)",
            span,
        )),
    }
}

/// `list_map(list: List<T>, f: (T) -> U) -> List<U>`.
fn builtin_list_map(interp: &mut Interpreter, args: Vec<Value>, span: Span) -> EvalResult {
    if args.len() != 2 {
        return Err(runtime_err(
            "E0503",
            format!("list_map expects 2 arguments, got {}", args.len()),
            span,
        ));
    }
    let list = &args[0];
    let func = &args[1];
    match (list, func) {
        (Value::List(items), Value::Closure { params, body, env }) => {
            if params.len() != 1 {
                return Err(runtime_err(
                    "E0503",
                    "list_map callback must take exactly 1 parameter",
                    span,
                ));
            }
            let mut results = Vec::with_capacity(items.len());
            for item in items {
                let saved_env = interp.env.clone();
                interp.env = env.clone();
                interp.env.define(params[0].clone(), item.clone());
                let val = interp.eval_expr(body)?;
                interp.env = saved_env;
                results.push(val);
            }
            Ok(Value::List(results))
        }
        (Value::List(items), Value::BuiltinFn(name)) => {
            let mut results = Vec::with_capacity(items.len());
            for item in items {
                let val = call_builtin(interp, name, vec![item.clone()], span)?;
                results.push(val);
            }
            Ok(Value::List(results))
        }
        _ => Err(runtime_err(
            "E0503",
            "list_map expects (List, Function)",
            span,
        )),
    }
}

/// `to_string(value) -> String` — generic, works on any type.
fn builtin_to_string(args: Vec<Value>, span: Span) -> EvalResult {
    if args.len() != 1 {
        return Err(runtime_err(
            "E0503",
            format!("to_string expects 1 argument, got {}", args.len()),
            span,
        ));
    }
    Ok(Value::String(format!("{}", args[0])))
}
