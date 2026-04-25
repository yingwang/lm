//! Type system for the LM programming language.
//!
//! This crate implements:
//! - **Hindley-Milner type inference** with union-find based unification
//! - **Effect checking** (pure vs io)
//! - **Pattern match exhaustiveness checking**
//!
//! # Error code ranges
//!
//! | Range       | Category                      |
//! |-------------|-------------------------------|
//! | E0200-E0299 | Type checking errors          |
//! | E0300-E0399 | Effect checking errors        |
//! | E0400-E0499 | Pattern exhaustiveness errors |
//!
//! # Usage
//!
//! ```
//! use lm_types::TypeChecker;
//! use lm_parser::ast::Program;
//!
//! // After parsing a program:
//! // let checker = TypeChecker::new();
//! // let diagnostics = checker.check(&program);
//! ```

mod types;
mod unify;
mod env;
mod infer;
mod effects;
mod exhaustiveness;

#[cfg(test)]
mod tests;

pub use types::{Type, TypeVarId, TypeScheme};
pub use env::TypeEnv;
pub use infer::TypeChecker;
