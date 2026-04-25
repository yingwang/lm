//! Type environment for tracking variable bindings and type definitions.
//!
//! The environment maps variable names to type schemes and tracks
//! user-defined ADT definitions.

use crate::types::{Type, TypeScheme, TypeVarId};
use crate::unify::UnificationTable;
use lm_parser::ast::Effect;
use std::collections::HashMap;

/// Information about a variant of an ADT.
#[derive(Debug, Clone)]
pub struct VariantInfo {
    /// The variant's name (e.g., "Circle", "Some", "Ok").
    pub name: String,
    /// The field types of the variant (empty for unit variants).
    pub field_types: Vec<Type>,
    /// The name of the ADT this variant belongs to.
    pub adt_name: String,
    /// Type parameters of the parent ADT.
    pub type_params: Vec<String>,
}

/// Information about a user-defined ADT.
#[derive(Debug, Clone)]
pub struct AdtInfo {
    /// The ADT name.
    pub name: String,
    /// Type parameter names.
    pub type_params: Vec<String>,
    /// All variants.
    pub variants: Vec<VariantInfo>,
}

/// Information about a function's effect.
#[derive(Debug, Clone)]
pub struct FnEffectInfo {
    /// The declared effect.
    pub effect: Effect,
}

/// The type environment.
///
/// Tracks variable bindings (with type schemes for polymorphism),
/// ADT definitions, variant constructors, and function effects.
#[derive(Debug)]
pub struct TypeEnv {
    /// Variable name -> type scheme.
    bindings: Vec<HashMap<String, TypeScheme>>,
    /// ADT name -> definition info.
    pub adt_defs: HashMap<String, AdtInfo>,
    /// Variant name -> variant info.
    pub variant_defs: HashMap<String, VariantInfo>,
    /// Function name -> effect info.
    pub fn_effects: HashMap<String, FnEffectInfo>,
}

impl TypeEnv {
    /// Create a new type environment with built-in types pre-populated.
    pub fn new(table: &mut UnificationTable) -> Self {
        let mut env = TypeEnv {
            bindings: vec![HashMap::new()],
            adt_defs: HashMap::new(),
            variant_defs: HashMap::new(),
            fn_effects: HashMap::new(),
        };

        // Register built-in functions
        env.register_builtins(table);

        env
    }

    fn register_builtins(&mut self, table: &mut UnificationTable) {
        // print : (String) -> Unit [io]
        let print_ty = TypeScheme::mono(Type::Fun(vec![Type::String], Box::new(Type::Unit)));
        self.bind("print".to_string(), print_ty);
        self.fn_effects.insert(
            "print".to_string(),
            FnEffectInfo {
                effect: Effect::Io,
            },
        );

        // to_string : forall a. (a) -> String [pure]
        let a = table.fresh_var();
        let a_id = match &a {
            Type::Var(id) => *id,
            _ => unreachable!(),
        };
        self.bind(
            "to_string".to_string(),
            TypeScheme {
                vars: vec![a_id],
                ty: Type::Fun(vec![a], Box::new(Type::String)),
            },
        );
        self.fn_effects.insert(
            "to_string".to_string(),
            FnEffectInfo {
                effect: Effect::Pure,
            },
        );

        // int_to_string : (Int) -> String [pure]
        self.bind(
            "int_to_string".to_string(),
            TypeScheme::mono(Type::Fun(vec![Type::Int], Box::new(Type::String))),
        );
        self.fn_effects.insert(
            "int_to_string".to_string(),
            FnEffectInfo {
                effect: Effect::Pure,
            },
        );

        // float_to_string : (Float) -> String [pure]
        self.bind(
            "float_to_string".to_string(),
            TypeScheme::mono(Type::Fun(vec![Type::Float], Box::new(Type::String))),
        );
        self.fn_effects.insert(
            "float_to_string".to_string(),
            FnEffectInfo {
                effect: Effect::Pure,
            },
        );

        // string_to_int : (String) -> Result<Int, String> [pure]
        self.bind(
            "string_to_int".to_string(),
            TypeScheme::mono(Type::Fun(
                vec![Type::String],
                Box::new(Type::Result(Box::new(Type::Int), Box::new(Type::String))),
            )),
        );
        self.fn_effects.insert(
            "string_to_int".to_string(),
            FnEffectInfo {
                effect: Effect::Pure,
            },
        );

        // len : forall a. (List<a>) -> Int [pure]
        let len_a = table.fresh_var();
        let len_a_id = match &len_a {
            Type::Var(id) => *id,
            _ => unreachable!(),
        };
        self.bind(
            "len".to_string(),
            TypeScheme {
                vars: vec![len_a_id],
                ty: Type::Fun(vec![Type::List(Box::new(len_a))], Box::new(Type::Int)),
            },
        );
        self.fn_effects.insert(
            "len".to_string(),
            FnEffectInfo {
                effect: Effect::Pure,
            },
        );

        // list_get : forall a. (List<a>, Int) -> Option<a> [pure]
        let lg_a = table.fresh_var();
        let lg_a_id = match &lg_a {
            Type::Var(id) => *id,
            _ => unreachable!(),
        };
        self.bind(
            "list_get".to_string(),
            TypeScheme {
                vars: vec![lg_a_id],
                ty: Type::Fun(
                    vec![Type::List(Box::new(lg_a.clone())), Type::Int],
                    Box::new(Type::Option(Box::new(lg_a))),
                ),
            },
        );
        self.fn_effects.insert(
            "list_get".to_string(),
            FnEffectInfo {
                effect: Effect::Pure,
            },
        );

        // list_push : forall a. (List<a>, a) -> List<a> [pure]
        let lp_a = table.fresh_var();
        let lp_a_id = match &lp_a {
            Type::Var(id) => *id,
            _ => unreachable!(),
        };
        self.bind(
            "list_push".to_string(),
            TypeScheme {
                vars: vec![lp_a_id],
                ty: Type::Fun(
                    vec![Type::List(Box::new(lp_a.clone())), lp_a.clone()],
                    Box::new(Type::List(Box::new(lp_a))),
                ),
            },
        );
        self.fn_effects.insert(
            "list_push".to_string(),
            FnEffectInfo {
                effect: Effect::Pure,
            },
        );

        // list_map : forall a b. (List<a>, (a) -> b) -> List<b> [pure]
        let lm_a = table.fresh_var();
        let lm_a_id = match &lm_a {
            Type::Var(id) => *id,
            _ => unreachable!(),
        };
        let lm_b = table.fresh_var();
        let lm_b_id = match &lm_b {
            Type::Var(id) => *id,
            _ => unreachable!(),
        };
        self.bind(
            "list_map".to_string(),
            TypeScheme {
                vars: vec![lm_a_id, lm_b_id],
                ty: Type::Fun(
                    vec![
                        Type::List(Box::new(lm_a.clone())),
                        Type::Fun(vec![lm_a], Box::new(lm_b.clone())),
                    ],
                    Box::new(Type::List(Box::new(lm_b))),
                ),
            },
        );
        self.fn_effects.insert(
            "list_map".to_string(),
            FnEffectInfo {
                effect: Effect::Pure,
            },
        );

        // read_line : () -> String [io]
        self.bind(
            "read_line".to_string(),
            TypeScheme::mono(Type::Fun(vec![], Box::new(Type::String))),
        );
        self.fn_effects.insert(
            "read_line".to_string(),
            FnEffectInfo {
                effect: Effect::Io,
            },
        );
    }

    /// Push a new scope onto the environment.
    pub fn push_scope(&mut self) {
        self.bindings.push(HashMap::new());
    }

    /// Pop the current scope.
    pub fn pop_scope(&mut self) {
        self.bindings.pop();
    }

    /// Bind a name to a type scheme in the current scope.
    pub fn bind(&mut self, name: String, scheme: TypeScheme) {
        if let Some(scope) = self.bindings.last_mut() {
            scope.insert(name, scheme);
        }
    }

    /// Look up a name, searching from innermost scope outward.
    pub fn lookup(&self, name: &str) -> Option<&TypeScheme> {
        for scope in self.bindings.iter().rev() {
            if let Some(scheme) = scope.get(name) {
                return Some(scheme);
            }
        }
        None
    }

    /// Instantiate a type scheme with fresh type variables.
    pub fn instantiate(&self, scheme: &TypeScheme, table: &mut UnificationTable) -> Type {
        if scheme.vars.is_empty() {
            return scheme.ty.clone();
        }
        let subst: Vec<(TypeVarId, Type)> = scheme
            .vars
            .iter()
            .map(|v| (*v, table.fresh_var()))
            .collect();
        scheme.ty.substitute(&subst)
    }

    /// Generalize a type into a type scheme by quantifying over free variables
    /// that are not bound in the environment.
    pub fn generalize(&self, ty: &Type, table: &UnificationTable) -> TypeScheme {
        let resolved = table.deep_resolve(ty);
        let free = resolved.free_vars();

        // Collect all type vars that are free in the environment
        let env_vars = self.env_free_vars(table);

        // Quantify over variables in the type that are NOT free in the environment
        let quantified: Vec<TypeVarId> = free
            .into_iter()
            .filter(|v| !env_vars.contains(v))
            .collect();

        TypeScheme {
            vars: quantified,
            ty: resolved,
        }
    }

    /// Collect all free type variables in the environment.
    fn env_free_vars(&self, table: &UnificationTable) -> Vec<TypeVarId> {
        let mut vars = Vec::new();
        for scope in &self.bindings {
            for scheme in scope.values() {
                let resolved = table.deep_resolve(&scheme.ty);
                let free = resolved.free_vars();
                for v in free {
                    if !scheme.vars.contains(&v) && !vars.contains(&v) {
                        vars.push(v);
                    }
                }
            }
        }
        vars
    }

    /// Register an ADT definition with all its variants.
    pub fn register_adt(&mut self, info: AdtInfo) {
        for variant in &info.variants {
            self.variant_defs
                .insert(variant.name.clone(), variant.clone());
        }
        self.adt_defs.insert(info.name.clone(), info);
    }
}
