//! Union-find based type unification.
//!
//! This module implements the core unification algorithm for Hindley-Milner
//! type inference, with occurs check to prevent infinite types.

use crate::types::{Type, TypeVarId};
use lm_diagnostics::Span;
use std::collections::HashMap;

/// A unification context that tracks type variable bindings.
#[derive(Debug)]
pub struct UnificationTable {
    /// Maps type variable IDs to their current binding (another type variable or a concrete type).
    bindings: HashMap<TypeVarId, Type>,
    /// Counter for generating fresh type variable IDs.
    next_var: u32,
}

/// An error produced by unification.
#[derive(Debug)]
pub struct UnifyError {
    /// Expected type.
    pub expected: Type,
    /// Found type.
    pub found: Type,
    /// Source span where the conflict arose.
    pub span: Span,
    /// Whether this is an infinite type (occurs check failure).
    pub is_infinite: bool,
}

impl UnificationTable {
    /// Create a new empty unification table.
    pub fn new() -> Self {
        UnificationTable {
            bindings: HashMap::new(),
            next_var: 0,
        }
    }

    /// Generate a fresh type variable.
    pub fn fresh_var(&mut self) -> Type {
        let id = TypeVarId(self.next_var);
        self.next_var += 1;
        Type::Var(id)
    }

    /// Find the representative type for a type, following binding chains.
    pub fn resolve(&self, ty: &Type) -> Type {
        match ty {
            Type::Var(id) => {
                if let Some(bound) = self.bindings.get(id) {
                    self.resolve(bound)
                } else {
                    ty.clone()
                }
            }
            _ => ty.clone(),
        }
    }

    /// Deeply resolve a type, replacing all bound variables throughout the structure.
    pub fn deep_resolve(&self, ty: &Type) -> Type {
        match ty {
            Type::Var(id) => {
                if let Some(bound) = self.bindings.get(id) {
                    self.deep_resolve(bound)
                } else {
                    ty.clone()
                }
            }
            Type::Fun(params, ret) => Type::Fun(
                params.iter().map(|p| self.deep_resolve(p)).collect(),
                Box::new(self.deep_resolve(ret)),
            ),
            Type::ADT(name, args) => {
                Type::ADT(name.clone(), args.iter().map(|a| self.deep_resolve(a)).collect())
            }
            Type::Option(inner) => Type::Option(Box::new(self.deep_resolve(inner))),
            Type::Result(ok, err) => Type::Result(
                Box::new(self.deep_resolve(ok)),
                Box::new(self.deep_resolve(err)),
            ),
            Type::List(inner) => Type::List(Box::new(self.deep_resolve(inner))),
            Type::Int | Type::Float | Type::Bool | Type::String | Type::Unit => ty.clone(),
        }
    }

    /// Unify two types. On success, records bindings. On failure, returns an error.
    #[allow(clippy::result_large_err)]
    pub fn unify(&mut self, t1: &Type, t2: &Type, span: Span) -> Result<(), UnifyError> {
        let t1 = self.resolve(t1);
        let t2 = self.resolve(t2);

        match (&t1, &t2) {
            // Same concrete types
            (Type::Int, Type::Int)
            | (Type::Float, Type::Float)
            | (Type::Bool, Type::Bool)
            | (Type::String, Type::String)
            | (Type::Unit, Type::Unit) => Ok(()),

            // Same variable
            (Type::Var(a), Type::Var(b)) if a == b => Ok(()),

            // Bind a variable
            (Type::Var(id), other) => {
                if self.occurs_in(*id, other) {
                    Err(UnifyError {
                        expected: t1.clone(),
                        found: t2.clone(),
                        span,
                        is_infinite: true,
                    })
                } else {
                    self.bindings.insert(*id, other.clone());
                    Ok(())
                }
            }
            (other, Type::Var(id)) => {
                if self.occurs_in(*id, other) {
                    Err(UnifyError {
                        expected: t1.clone(),
                        found: t2.clone(),
                        span,
                        is_infinite: true,
                    })
                } else {
                    self.bindings.insert(*id, other.clone());
                    Ok(())
                }
            }

            // Function types
            (Type::Fun(params1, ret1), Type::Fun(params2, ret2)) => {
                if params1.len() != params2.len() {
                    return Err(UnifyError {
                        expected: t1.clone(),
                        found: t2.clone(),
                        span,
                        is_infinite: false,
                    });
                }
                for (p1, p2) in params1.iter().zip(params2.iter()) {
                    self.unify(p1, p2, span)?;
                }
                self.unify(ret1, ret2, span)
            }

            // ADT types
            (Type::ADT(name1, args1), Type::ADT(name2, args2)) => {
                if name1 != name2 || args1.len() != args2.len() {
                    return Err(UnifyError {
                        expected: t1.clone(),
                        found: t2.clone(),
                        span,
                        is_infinite: false,
                    });
                }
                for (a1, a2) in args1.iter().zip(args2.iter()) {
                    self.unify(a1, a2, span)?;
                }
                Ok(())
            }

            // Option types
            (Type::Option(a), Type::Option(b)) => self.unify(a, b, span),

            // Result types
            (Type::Result(ok1, err1), Type::Result(ok2, err2)) => {
                self.unify(ok1, ok2, span)?;
                self.unify(err1, err2, span)
            }

            // List types
            (Type::List(a), Type::List(b)) => self.unify(a, b, span),

            // Mismatch
            _ => Err(UnifyError {
                expected: t1.clone(),
                found: t2.clone(),
                span,
                is_infinite: false,
            }),
        }
    }

    /// Occurs check: does `var` appear anywhere in `ty`?
    fn occurs_in(&self, var: TypeVarId, ty: &Type) -> bool {
        let ty = self.resolve(ty);
        match &ty {
            Type::Var(id) => *id == var,
            Type::Fun(params, ret) => {
                params.iter().any(|p| self.occurs_in(var, p)) || self.occurs_in(var, ret)
            }
            Type::ADT(_, args) => args.iter().any(|a| self.occurs_in(var, a)),
            Type::Option(inner) | Type::List(inner) => self.occurs_in(var, inner),
            Type::Result(ok, err) => self.occurs_in(var, ok) || self.occurs_in(var, err),
            Type::Int | Type::Float | Type::Bool | Type::String | Type::Unit => false,
        }
    }

    /// Get the current next variable counter (for generalization).
    pub fn current_var_count(&self) -> u32 {
        self.next_var
    }
}
