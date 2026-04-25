//! Core type representations for the LM type system.
//!
//! [`Type`] is the main type representation used during inference.
//! [`TypeScheme`] represents polymorphic types (forall-quantified).

use std::fmt;

/// Unique identifier for a type variable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeVarId(pub u32);

impl fmt::Display for TypeVarId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "t{}", self.0)
    }
}

/// A monomorphic type in the LM type system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    /// Integer type.
    Int,
    /// Floating-point type.
    Float,
    /// Boolean type.
    Bool,
    /// String type.
    String,
    /// Unit type (void-like).
    Unit,
    /// A unification variable, to be resolved during inference.
    Var(TypeVarId),
    /// Function type: parameter types -> return type.
    Fun(Vec<Type>, Box<Type>),
    /// A user-defined algebraic data type with type arguments.
    ADT(std::string::String, Vec<Type>),
    /// `Option<T>`.
    Option(Box<Type>),
    /// `Result<T, E>`.
    Result(Box<Type>, Box<Type>),
    /// `List<T>`.
    List(Box<Type>),
}

impl Type {
    /// Pretty-print the type for user-facing error messages.
    pub fn display(&self) -> std::string::String {
        match self {
            Type::Int => "Int".to_string(),
            Type::Float => "Float".to_string(),
            Type::Bool => "Bool".to_string(),
            Type::String => "String".to_string(),
            Type::Unit => "Unit".to_string(),
            Type::Var(id) => format!("{}", id),
            Type::Fun(params, ret) => {
                let param_strs: Vec<_> = params.iter().map(|p| p.display()).collect();
                format!("({}) -> {}", param_strs.join(", "), ret.display())
            }
            Type::ADT(name, args) => {
                if args.is_empty() {
                    name.clone()
                } else {
                    let arg_strs: Vec<_> = args.iter().map(|a| a.display()).collect();
                    format!("{}<{}>", name, arg_strs.join(", "))
                }
            }
            Type::Option(inner) => format!("Option<{}>", inner.display()),
            Type::Result(ok, err) => format!("Result<{}, {}>", ok.display(), err.display()),
            Type::List(inner) => format!("List<{}>", inner.display()),
        }
    }

    /// Collect all free type variable IDs in this type.
    pub fn free_vars(&self) -> Vec<TypeVarId> {
        let mut vars = Vec::new();
        self.collect_free_vars(&mut vars);
        vars
    }

    fn collect_free_vars(&self, vars: &mut Vec<TypeVarId>) {
        match self {
            Type::Var(id) => {
                if !vars.contains(id) {
                    vars.push(*id);
                }
            }
            Type::Fun(params, ret) => {
                for p in params {
                    p.collect_free_vars(vars);
                }
                ret.collect_free_vars(vars);
            }
            Type::ADT(_, args) => {
                for a in args {
                    a.collect_free_vars(vars);
                }
            }
            Type::Option(inner) | Type::List(inner) => inner.collect_free_vars(vars),
            Type::Result(ok, err) => {
                ok.collect_free_vars(vars);
                err.collect_free_vars(vars);
            }
            Type::Int | Type::Float | Type::Bool | Type::String | Type::Unit => {}
        }
    }

    /// Substitute type variables according to a mapping.
    pub fn substitute(&self, subst: &[(TypeVarId, Type)]) -> Type {
        match self {
            Type::Var(id) => {
                for (var_id, replacement) in subst {
                    if id == var_id {
                        return replacement.clone();
                    }
                }
                self.clone()
            }
            Type::Fun(params, ret) => Type::Fun(
                params.iter().map(|p| p.substitute(subst)).collect(),
                Box::new(ret.substitute(subst)),
            ),
            Type::ADT(name, args) => {
                Type::ADT(name.clone(), args.iter().map(|a| a.substitute(subst)).collect())
            }
            Type::Option(inner) => Type::Option(Box::new(inner.substitute(subst))),
            Type::Result(ok, err) => Type::Result(
                Box::new(ok.substitute(subst)),
                Box::new(err.substitute(subst)),
            ),
            Type::List(inner) => Type::List(Box::new(inner.substitute(subst))),
            Type::Int | Type::Float | Type::Bool | Type::String | Type::Unit => self.clone(),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}

/// A polymorphic type scheme: forall vars. type.
///
/// Used to represent let-bound polymorphic values.
#[derive(Debug, Clone)]
pub struct TypeScheme {
    /// Quantified type variable IDs.
    pub vars: Vec<TypeVarId>,
    /// The body type.
    pub ty: Type,
}

impl TypeScheme {
    /// Create a monomorphic scheme (no quantified variables).
    pub fn mono(ty: Type) -> Self {
        TypeScheme {
            vars: Vec::new(),
            ty,
        }
    }
}
