//! Environment for the LM interpreter.
//!
//! Uses a simple HashMap-based environment with clone-on-write semantics.
//! Since LM is a pure functional language (no mutation), cloning the env
//! for nested scopes is safe and straightforward.

use crate::value::Value;
use std::collections::HashMap;

/// An environment mapping names to values.
///
/// Environments are cloned when entering new scopes (let bindings,
/// function bodies, match arms). This is efficient because LM is
/// immutable — no value sharing/aliasing issues.
#[derive(Debug, Clone)]
pub struct Env {
    /// The bindings in this environment.
    bindings: HashMap<String, Value>,
}

impl Env {
    /// Create a new empty environment.
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    /// Look up a name in the environment.
    pub fn get(&self, name: &str) -> Option<Value> {
        self.bindings.get(name).cloned()
    }

    /// Define a new binding (or overwrite an existing one).
    pub fn define(&mut self, name: String, value: Value) {
        self.bindings.insert(name, value);
    }
}

impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}
